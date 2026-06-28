//! Pure cost math: price agent token usage and pick the effective cost of a usage event.

use crate::stream::NormalizedEvent;

/// Per-1M-token rates (input, output) in USD by model family. Unknown/local → free.
fn rates(model: &str) -> (f64, f64) {
    let m = model.to_ascii_lowercase();
    if m.contains("opus") {
        (5.0, 25.0)
    } else if m.contains("sonnet") {
        (3.0, 15.0)
    } else if m.contains("haiku") {
        (1.0, 5.0)
    } else {
        (0.0, 0.0)
    }
}

/// Price token usage in USD for backends/models that don't report authoritative USD.
/// Cache reads bill at 0.1× the input rate; cache writes at 1.25×.
pub fn price_tokens(
    model: &str,
    input: u64,
    output: u64,
    cache_read: u64,
    cache_create: u64,
) -> f64 {
    let (in_rate, out_rate) = rates(model);
    let i = input as f64 * in_rate;
    let o = output as f64 * out_rate;
    let cr = cache_read as f64 * in_rate * 0.1;
    let cw = cache_create as f64 * in_rate * 1.25;
    (i + o + cr + cw) / 1_000_000.0
}

/// The effective cost of a usage event: prefer authoritative `cost_usd`, else price the tokens.
pub fn effective_cost(model: &str, ev: &NormalizedEvent) -> f64 {
    if let NormalizedEvent::Usage {
        cost_usd,
        input,
        output,
        cache_read,
        cache_create,
    } = ev
    {
        if *cost_usd > 0.0 {
            *cost_usd
        } else {
            price_tokens(model, *input, *output, *cache_read, *cache_create)
        }
    } else {
        0.0
    }
}

/// Spend rate in USD/hour from total spent and elapsed seconds (zero elapsed → zero rate).
pub fn burn_rate(spent_usd: f64, elapsed_secs: u64) -> f64 {
    if elapsed_secs == 0 {
        return 0.0;
    }
    spent_usd / (elapsed_secs as f64 / 3600.0)
}

/// Seconds until `spent` reaches `limit` at `rate` USD/hour. None if not advancing; Some(0) if at/over.
pub fn time_to_ceiling(spent_usd: f64, rate_per_hour: f64, limit_usd: f64) -> Option<u64> {
    if rate_per_hour <= 0.0 {
        return None;
    }
    let remaining = limit_usd - spent_usd;
    if remaining <= 0.0 {
        return Some(0);
    }
    Some((remaining / rate_per_hour * 3600.0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::NormalizedEvent::*;

    #[test]
    fn prices_tokens_by_model() {
        assert!((price_tokens("claude-opus-4-8", 1_000_000, 0, 0, 0) - 5.0).abs() < 1e-9);
        assert!((price_tokens("claude-opus-4-8", 0, 1_000_000, 0, 0) - 25.0).abs() < 1e-9);
        assert!((price_tokens("claude-sonnet-4-6", 1_000_000, 0, 0, 0) - 3.0).abs() < 1e-9);
        assert!((price_tokens("claude-haiku-4-5", 1_000_000, 0, 0, 0) - 1.0).abs() < 1e-9);
        // cache read 0.1× input, write 1.25× input (opus input $5/M)
        assert!((price_tokens("opus", 0, 0, 1_000_000, 0) - 0.5).abs() < 1e-9);
        assert!((price_tokens("opus", 0, 0, 0, 1_000_000) - 6.25).abs() < 1e-9);
        // unknown / local model is free
        assert_eq!(price_tokens("local-llama", 1_000_000, 1_000_000, 0, 0), 0.0);
    }

    #[test]
    fn effective_cost_prefers_authoritative_usd() {
        let reported = Usage {
            cost_usd: 0.42,
            input: 1_000_000,
            output: 0,
            cache_read: 0,
            cache_create: 0,
        };
        assert!((effective_cost("opus", &reported) - 0.42).abs() < 1e-9); // uses cost_usd, not $5
        let codex = Usage {
            cost_usd: 0.0,
            input: 1_000_000,
            output: 0,
            cache_read: 0,
            cache_create: 0,
        };
        assert!((effective_cost("claude-opus-4-8", &codex) - 5.0).abs() < 1e-9); // prices tokens
        assert_eq!(
            effective_cost("opus", &SessionStarted { model: "x".into() }),
            0.0
        );
    }

    #[test]
    fn computes_burn_rate() {
        assert!((burn_rate(6.0, 3600) - 6.0).abs() < 1e-9);
        assert!((burn_rate(6.0, 1800) - 12.0).abs() < 1e-9);
        assert_eq!(burn_rate(5.0, 0), 0.0); // no elapsed time → no rate
    }

    #[test]
    fn predicts_time_to_ceiling() {
        assert_eq!(time_to_ceiling(5.0, 5.0, 10.0), Some(3600)); // $5 left at $5/hr = 1h
        assert_eq!(time_to_ceiling(0.0, 0.0, 10.0), None); // not advancing
        assert_eq!(time_to_ceiling(10.0, 5.0, 10.0), Some(0)); // already at ceiling
        assert_eq!(time_to_ceiling(12.0, 5.0, 10.0), Some(0)); // already over
    }
}
