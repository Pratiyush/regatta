//! Pure layout model for the multi-pane review Grid.

/// How the Grid is split.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridLayout {
    Focus,
    Split,
    Quad,
}

impl GridLayout {
    /// The number of panes this layout shows.
    pub fn pane_count(self) -> usize {
        match self {
            GridLayout::Focus => 1,
            GridLayout::Split => 2,
            GridLayout::Quad => 4,
        }
    }

    /// The layout for an ⌥1–4 / count selection (1 → Focus, 2 → Split, 3+ → Quad).
    pub fn from_count(n: usize) -> GridLayout {
        match n {
            0 | 1 => GridLayout::Focus,
            2 => GridLayout::Split,
            _ => GridLayout::Quad,
        }
    }
}

/// Fill the panes with the first `pane_count` session ids, padding empty slots with `None`.
pub fn panes(layout: GridLayout, sessions: &[String]) -> Vec<Option<String>> {
    (0..layout.pane_count())
        .map(|i| sessions.get(i).cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_from_count() {
        assert_eq!(GridLayout::from_count(0), GridLayout::Focus);
        assert_eq!(GridLayout::from_count(1), GridLayout::Focus);
        assert_eq!(GridLayout::from_count(2), GridLayout::Split);
        assert_eq!(GridLayout::from_count(4), GridLayout::Quad);
        assert_eq!(GridLayout::from_count(9), GridLayout::Quad);
        assert_eq!(GridLayout::Focus.pane_count(), 1);
        assert_eq!(GridLayout::Split.pane_count(), 2);
        assert_eq!(GridLayout::Quad.pane_count(), 4);
    }

    #[test]
    fn fills_panes_with_sessions() {
        let s = vec!["a".to_string(), "b".to_string()];
        assert_eq!(
            panes(GridLayout::Quad, &s),
            vec![Some("a".into()), Some("b".into()), None, None]
        );
        assert_eq!(panes(GridLayout::Focus, &s), vec![Some("a".into())]);
        assert_eq!(panes(GridLayout::Split, &[]), vec![None, None]);
    }
}
