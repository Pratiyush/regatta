//! Attention priority — how urgently a session needs the human.
//! Drives the ordering of the Attention Dock (the headline "triage over tabs" surface).

/// The lifecycle state of an agent session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Starting,
    Running,
    NeedsInput,
    WaitingApproval,
    Compacting,
    RateLimited,
    Paused,
    Error,
    Done,
}

/// Attention priority for the dock: higher = more urgent; 0 = does not need the human.
pub fn attention_priority(state: SessionState) -> u8 {
    use SessionState::*;
    // Exhaustive on purpose: adding a state without ranking it is a compile error.
    match state {
        WaitingApproval => 5,
        NeedsInput => 4,
        Error => 3,
        Done => 2,
        Starting | Running | Compacting | RateLimited | Paused => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::SessionState::*;
    use super::*;

    #[test]
    fn ranks_blocked_sessions() {
        assert_eq!(attention_priority(WaitingApproval), 5);
        assert_eq!(attention_priority(NeedsInput), 4);
        assert_eq!(attention_priority(Error), 3);
        assert_eq!(attention_priority(Done), 2);
    }

    #[test]
    fn working_sessions_score_zero() {
        for s in [Starting, Running, Compacting, RateLimited, Paused] {
            assert_eq!(attention_priority(s), 0);
        }
    }
}
