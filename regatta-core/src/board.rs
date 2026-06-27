//! Pure helpers for the Resume board — recency grouping for the session list.

/// Bucket a session by how long ago it was last active (unix seconds), relative to `now`.
/// Future timestamps (clock skew) collapse to "Today" rather than a negative bucket.
pub fn recency_group(last_activity: i64, now: i64) -> &'static str {
    const DAY: i64 = 86_400;
    let diff = (now - last_activity).max(0);
    if diff < DAY {
        "Today"
    } else if diff < 2 * DAY {
        "Yesterday"
    } else if diff < 7 * DAY {
        "Earlier this week"
    } else if diff < 14 * DAY {
        "Last two weeks"
    } else {
        "Older"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: i64 = 86_400;

    #[test]
    fn buckets_by_recency() {
        let now = 100 * DAY;
        assert_eq!(recency_group(now, now), "Today");
        assert_eq!(recency_group(now - DAY / 2, now), "Today");
        assert_eq!(recency_group(now - DAY - 1, now), "Yesterday");
        assert_eq!(recency_group(now - 3 * DAY, now), "Earlier this week");
        assert_eq!(recency_group(now - 10 * DAY, now), "Last two weeks");
        assert_eq!(recency_group(now - 30 * DAY, now), "Older");
    }

    #[test]
    fn future_or_zero_diff_is_today() {
        let now = 100 * DAY;
        assert_eq!(recency_group(now + DAY, now), "Today"); // clock skew → never a negative bucket
    }
}
