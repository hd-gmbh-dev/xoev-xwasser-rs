use once_cell::sync::OnceCell;
use std::sync::RwLock;
use tiny_id::ShortCodeGenerator;

static ID_GEN: OnceCell<RwLock<ShortCodeGenerator<char>>> = OnceCell::new();

/// Deterministic values for the golden-file tests.
///
/// Only compiled with the `test-fixtures` feature, which the crate enables for
/// its own test builds (dev-dependency). A consumer never sees this and the
/// production builders keep generating random ids and the real clock.
#[cfg(feature = "test-fixtures")]
mod fixtures {
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

    /// Value used for every "now" in fixture mode.
    pub const FIXED_TIMESTAMP: &str = "2024-05-28T09:00:00";
    /// Value used for every "today" in fixture mode.
    pub const FIXED_DATE: &str = "2024-05-28";

    static FIXTURES: AtomicBool = AtomicBool::new(false);
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);

    /// Pin uuid, id and clock values to a deterministic sequence so a test run
    /// leaves the golden files untouched.
    pub fn use_fixed_fixtures() {
        FIXTURES.store(true, Ordering::Relaxed);
        SEQUENCE.store(0, Ordering::Relaxed);
    }

    pub fn enabled() -> bool {
        FIXTURES.load(Ordering::Relaxed)
    }

    pub fn next() -> u64 {
        SEQUENCE.fetch_add(1, Ordering::Relaxed) + 1
    }
}

#[cfg(feature = "test-fixtures")]
pub use fixtures::{FIXED_DATE, FIXED_TIMESTAMP, use_fixed_fixtures};

pub fn new_id() -> String {
    #[cfg(feature = "test-fixtures")]
    if fixtures::enabled() {
        return format!("fixture{:08}", fixtures::next());
    }
    let mut w = ID_GEN
        .get_or_init(|| RwLock::new(ShortCodeGenerator::new_alphanumeric(8)))
        .write()
        .unwrap();
    w.next_string()
}

pub fn new_uuid() -> String {
    #[cfg(feature = "test-fixtures")]
    if fixtures::enabled() {
        return format!("00000000-0000-4000-8000-{:012}", fixtures::next());
    }
    uuid::Uuid::new_v4().to_string()
}

pub fn now() -> String {
    #[cfg(feature = "test-fixtures")]
    if fixtures::enabled() {
        return fixtures::FIXED_TIMESTAMP.to_string();
    }
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn today() -> String {
    #[cfg(feature = "test-fixtures")]
    if fixtures::enabled() {
        return fixtures::FIXED_DATE.to_string();
    }
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[cfg(all(test, feature = "test-fixtures"))]
mod tests {
    #[test]
    fn fixture_mode_is_deterministic() {
        super::use_fixed_fixtures();
        assert_eq!(super::new_id(), "fixture00000001");
        assert_eq!(super::new_uuid(), "00000000-0000-4000-8000-000000000002");
        assert_eq!(super::now(), super::FIXED_TIMESTAMP);
        assert_eq!(super::today(), super::FIXED_DATE);
    }
}
