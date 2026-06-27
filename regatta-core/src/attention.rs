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

/// Order sessions for the Attention Dock. Input is each session's `(state, seen)`.
/// Returns the indices of sessions that still need the human (unseen, priority > 0),
/// most-urgent first. Ties preserve input order (stable), so older items surface first
/// when the caller passes them oldest-first.
pub fn dock_order(sessions: &[(SessionState, bool)]) -> Vec<usize> {
    let mut idx: Vec<usize> = sessions
        .iter()
        .enumerate()
        .filter(|(_, (state, seen))| !*seen && attention_priority(*state) > 0)
        .map(|(i, _)| i)
        .collect();
    // Stable sort by priority descending → ties keep input order.
    idx.sort_by(|&a, &b| attention_priority(sessions[b].0).cmp(&attention_priority(sessions[a].0)));
    idx
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

    #[test]
    fn dock_orders_by_urgency() {
        // 0 Running(skip) · 1 WaitingApproval(5) · 2 NeedsInput(4) · 3 Done-seen(skip) · 4 Error(3)
        let sessions = [
            (Running, false),
            (WaitingApproval, false),
            (NeedsInput, false),
            (Done, true),
            (Error, false),
        ];
        assert_eq!(dock_order(&sessions), vec![1, 2, 4]);
    }

    #[test]
    fn dock_excludes_working_and_seen() {
        let sessions = [(Running, false), (Done, true), (Starting, false)];
        assert_eq!(dock_order(&sessions), Vec::<usize>::new());
    }
}
