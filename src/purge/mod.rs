use std::fs;

pub fn purge(path: &std::path::Path) -> std::io::Result<()> {
    if path.is_dir() && path.ends_with("node_modules") {
        fs::remove_dir(path)?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            purge(&entry.path())?;
        }
    }
    Ok(())
}
