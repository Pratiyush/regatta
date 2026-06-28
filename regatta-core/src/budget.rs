//! Pure budget model: classify spend against a budget and decide whether to auto-pause.

/// What happens when a budget ceiling is reached.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BudgetAction {
    Warn,
    Throttle,
    Block,
}

/// A spend ceiling and the action taken at it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Budget {
    pub limit_usd: f64,
    pub action: BudgetAction,
}

/// Where spend stands against a budget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BudgetStatus {
    Ok,
    Warn,
    Exceeded,
}

/// Classify spend against a budget: Warn at ≥80%, Exceeded at ≥100%; a non-positive limit is unbounded.
pub fn budget_status(spent_usd: f64, budget: &Budget) -> BudgetStatus {
    if budget.limit_usd <= 0.0 {
        return BudgetStatus::Ok;
    }
    let frac = spent_usd / budget.limit_usd;
    if frac >= 1.0 {
        BudgetStatus::Exceeded
    } else if frac >= 0.8 {
        BudgetStatus::Warn
    } else {
        BudgetStatus::Ok
    }
}

/// Auto-pause only when spend is Exceeded and the budget's action is Block.
pub fn should_pause(status: BudgetStatus, action: BudgetAction) -> bool {
    status == BudgetStatus::Exceeded && action == BudgetAction::Block
}

#[cfg(test)]
mod tests {
    use super::*;

    fn budget(limit: f64, action: BudgetAction) -> Budget {
        Budget {
            limit_usd: limit,
            action,
        }
    }

    #[test]
    fn classifies_spend_against_budget() {
        let b = budget(10.0, BudgetAction::Block);
        assert_eq!(budget_status(5.0, &b), BudgetStatus::Ok);
        assert_eq!(budget_status(8.0, &b), BudgetStatus::Warn);
        assert_eq!(budget_status(10.0, &b), BudgetStatus::Exceeded);
        assert_eq!(budget_status(12.0, &b), BudgetStatus::Exceeded);
        // a non-positive limit is unbounded
        assert_eq!(
            budget_status(999.0, &budget(0.0, BudgetAction::Warn)),
            BudgetStatus::Ok
        );
    }

    #[test]
    fn pauses_only_when_exceeded_and_block() {
        assert!(should_pause(BudgetStatus::Exceeded, BudgetAction::Block));
        assert!(!should_pause(BudgetStatus::Exceeded, BudgetAction::Warn));
        assert!(!should_pause(BudgetStatus::Warn, BudgetAction::Block));
        assert!(!should_pause(BudgetStatus::Ok, BudgetAction::Block));
    }
}
