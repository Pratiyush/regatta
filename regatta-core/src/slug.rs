//! Turn a session name into a filesystem- and git-safe slug for worktree branches.

/// Slugify a session name: lowercase, only `[a-z0-9-]`, runs of other characters
/// collapsed to a single hyphen, leading/trailing hyphens removed. Falls back to
/// `"session"` when nothing alphanumeric remains.
pub fn slugify(name: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "session".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugifies_a_name() {
        assert_eq!(slugify("Fix: OAuth Bug!"), "fix-oauth-bug");
    }

    #[test]
    fn empty_slug_falls_back() {
        assert_eq!(slugify("!!! @@@"), "session");
    }
}
