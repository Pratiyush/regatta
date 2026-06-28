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

/// One file's line counts from `git diff --numstat`. `None` means a binary file.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffStat {
    pub path: String,
    pub added: Option<u64>,
    pub removed: Option<u64>,
}

/// Parse `git diff --numstat` into per-file added/removed counts (binary files → None counts).
pub fn parse_numstat(out: &str) -> Vec<DiffStat> {
    out.lines().filter_map(parse_numstat_line).collect()
}

fn parse_numstat_line(line: &str) -> Option<DiffStat> {
    let parts: Vec<&str> = line.splitn(3, '\t').collect();
    if parts.len() < 3 {
        return None;
    }
    let path = parts[2].trim();
    if path.is_empty() {
        return None;
    }
    let parse = |s: &str| {
        if s == "-" {
            None
        } else {
            s.parse::<u64>().ok()
        }
    };
    Some(DiffStat {
        path: path.to_string(),
        added: parse(parts[0]),
        removed: parse(parts[1]),
    })
}

/// Total added and removed lines across diff stats (binary files count as 0).
pub fn summarize_diff(stats: &[DiffStat]) -> (u64, u64) {
    stats.iter().fold((0, 0), |(a, r), s| {
        (a + s.added.unwrap_or(0), r + s.removed.unwrap_or(0))
    })
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

    #[test]
    fn parses_git_numstat() {
        let out = "10\t5\tsrc/a.rs\n-\t-\timg.png\n0\t3\tgone.rs\ngarbage\n1\t2\t  \n";
        assert_eq!(
            parse_numstat(out),
            vec![
                DiffStat {
                    path: "src/a.rs".into(),
                    added: Some(10),
                    removed: Some(5)
                },
                DiffStat {
                    path: "img.png".into(),
                    added: None,
                    removed: None
                },
                DiffStat {
                    path: "gone.rs".into(),
                    added: Some(0),
                    removed: Some(3)
                },
            ]
        );
    }

    #[test]
    fn numstat_skips_malformed_lines() {
        assert!(parse_numstat("").is_empty());
        assert!(parse_numstat("nope\n").is_empty()); // no tabs
        assert!(parse_numstat("1\t2\t\n").is_empty()); // empty path
    }

    #[test]
    fn sums_diff_stats() {
        let stats = vec![
            DiffStat {
                path: "a".into(),
                added: Some(10),
                removed: Some(5),
            },
            DiffStat {
                path: "b".into(),
                added: Some(0),
                removed: Some(3),
            },
            DiffStat {
                path: "img".into(),
                added: None,
                removed: None,
            }, // binary → 0
        ];
        assert_eq!(summarize_diff(&stats), (10, 8));
    }

    #[test]
    fn empty_diff_is_zero() {
        assert_eq!(summarize_diff(&[]), (0, 0));
    }
}
