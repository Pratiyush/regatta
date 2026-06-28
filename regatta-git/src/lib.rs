//! Glue: run the real `git` CLI in a working directory and parse its output via `regatta_core::git`.

use regatta_core::git::{parse_numstat, parse_status, DiffStat, FileChange};
use std::path::Path;
use std::process::Command;

fn run_git(cwd: &Path, args: &[&str]) -> std::io::Result<String> {
    let out = Command::new("git").args(args).current_dir(cwd).output()?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Working-tree changes in `cwd` (parsed `git status --porcelain`); empty if `cwd` is not a repo.
pub fn status(cwd: &Path) -> std::io::Result<Vec<FileChange>> {
    Ok(parse_status(&run_git(cwd, &["status", "--porcelain"])?))
}

/// Per-file +/- counts vs HEAD in `cwd` (parsed `git diff --numstat HEAD`).
pub fn diffstat(cwd: &Path) -> std::io::Result<Vec<DiffStat>> {
    Ok(parse_numstat(&run_git(
        cwd,
        &["diff", "--numstat", "HEAD"],
    )?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &std::path::Path, args: &[&str]) {
        std::process::Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap();
    }

    #[test]
    fn reads_status_and_diffstat_from_a_repo() {
        let dir = std::env::temp_dir().join(format!("regatta_git_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        git(&dir, &["init", "-q"]);
        git(&dir, &["config", "user.email", "t@t"]);
        git(&dir, &["config", "user.name", "t"]);
        std::fs::write(dir.join("a.txt"), "one\ntwo\n").unwrap();
        git(&dir, &["add", "."]);
        git(&dir, &["commit", "-qm", "init"]);
        // modify a tracked file + add an untracked one
        std::fs::write(dir.join("a.txt"), "one\ntwo\nthree\n").unwrap();
        std::fs::write(dir.join("new.txt"), "x\n").unwrap();

        let st = status(&dir).unwrap();
        assert!(st.iter().any(|c| c.path == "a.txt" && c.status == 'M'));
        assert!(st.iter().any(|c| c.path == "new.txt" && c.status == '?'));

        let ds = diffstat(&dir).unwrap();
        let a = ds
            .iter()
            .find(|d| d.path == "a.txt")
            .expect("a.txt diffstat");
        assert_eq!(a.added, Some(1)); // one line added
        assert_eq!(a.removed, Some(0));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn non_repo_yields_empty_status() {
        let dir = std::env::temp_dir().join(format!("regatta_nogit_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert!(status(&dir).unwrap().is_empty()); // not a repo → git errors, stdout empty
        let _ = std::fs::remove_dir_all(&dir);
    }
}
