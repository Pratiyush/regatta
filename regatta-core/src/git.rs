//! Parse `git` CLI output (status, diff) into structured changes — pure, so it's fully testable.

/// One changed file from `git status`.
#[derive(Debug, Clone, PartialEq)]
pub struct FileChange {
    pub path: String,
    pub status: char,
}

/// Parse `git status --porcelain` into per-file changes (status M/A/D/R/?; a rename → its new path).
pub fn parse_status(porcelain: &str) -> Vec<FileChange> {
    porcelain.lines().filter_map(parse_status_line).collect()
}

fn parse_status_line(line: &str) -> Option<FileChange> {
    if line.len() < 4 {
        return None;
    }
    let bytes = line.as_bytes();
    let (x, y) = (bytes[0] as char, bytes[1] as char);
    let rest = line[3..].trim();
    if rest.is_empty() {
        return None;
    }
    let status = if x == '?' {
        '?'
    } else if x != ' ' {
        x
    } else {
        y
    };
    // a rename shows "old -> new"; keep the new path
    let path = match rest.split_once(" -> ") {
        Some((_, new)) => new.to_string(),
        None => rest.to_string(),
    };
    Some(FileChange { path, status })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_git_status_porcelain() {
        let out = " M src/a.rs\nA  src/b.rs\n?? new.rs\nR  old.rs -> renamed.rs\nD  gone.rs\nx\n";
        assert_eq!(
            parse_status(out),
            vec![
                FileChange {
                    path: "src/a.rs".into(),
                    status: 'M'
                },
                FileChange {
                    path: "src/b.rs".into(),
                    status: 'A'
                },
                FileChange {
                    path: "new.rs".into(),
                    status: '?'
                },
                FileChange {
                    path: "renamed.rs".into(),
                    status: 'R'
                },
                FileChange {
                    path: "gone.rs".into(),
                    status: 'D'
                },
            ]
        );
    }

    #[test]
    fn skips_blank_and_pathless_lines() {
        assert!(parse_status("").is_empty());
        assert!(parse_status("MM   \n").is_empty()); // status code but empty path
    }
}
