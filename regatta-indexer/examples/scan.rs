//! Lived test / dev tool: index the real `~/.claude/projects` and print a summary.
//! Run: `cargo run -p regatta_indexer --example scan`

fn main() -> std::io::Result<()> {
    let home = std::env::var("HOME").expect("HOME");
    let dir = std::path::Path::new(&home).join(".claude/projects");
    let store = regatta_store::Store::open_in_memory().expect("store");
    let n = regatta_indexer::index_dir(&store, &dir)?;
    println!("indexed {n} sessions from {}", dir.display());
    for r in store.board_query("").expect("query").into_iter().take(8) {
        println!("  {} — {}", r.project, r.title);
    }
    Ok(())
}
