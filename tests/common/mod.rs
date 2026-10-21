//! Shared helpers for the golden-file tests.
//!
//! Golden files are compared, not overwritten: a test run leaves the working
//! tree clean. Regenerate them deliberately with
//!
//! ```sh
//! UPDATE_FIXTURES=1 cargo test
//! ```

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn updating() -> bool {
    std::env::var("UPDATE_FIXTURES").is_ok_and(|value| value != "0" && !value.is_empty())
}

/// Compare `content` with the golden file (or write it when `UPDATE_FIXTURES`
/// is set). `path` is relative to the crate root, e.g. `tests/quality_report.json`.
pub fn golden(path: &str, content: &str) -> anyhow::Result<()> {
    let file = root().join(path);
    if updating() {
        std::fs::write(&file, content)?;
        return Ok(());
    }
    let expected = std::fs::read_to_string(&file).map_err(|err| {
        anyhow::anyhow!(
            "{}: {err} (regenerate with UPDATE_FIXTURES=1 cargo test)",
            file.display()
        )
    })?;
    if normalise(&expected) != normalise(content) {
        anyhow::bail!(
            "{} differs from the generated value - if the change is intended, run \
             `UPDATE_FIXTURES=1 cargo test`",
            display(&file)
        );
    }
    Ok(())
}

fn display(path: &Path) -> String {
    path.strip_prefix(root())
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Trailing newlines and line endings must not decide whether a test passes.
fn normalise(value: &str) -> String {
    value.replace("\r\n", "\n").trim_end().to_string()
}
