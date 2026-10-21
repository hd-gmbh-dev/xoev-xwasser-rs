use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{OnceLock, RwLock};

use tiny_id::ShortCodeGenerator;

static ID_GEN: OnceLock<RwLock<ShortCodeGenerator<char>>> = OnceLock::new();

/// Fixture mode: tests pin the volatile values (uuid, ids, clock) so that the
/// golden files stay byte identical between runs.
static FIXTURES: AtomicBool = AtomicBool::new(false);
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Value used for every "now" in fixture mode.
pub const FIXED_TIMESTAMP: &str = "2024-05-28T09:00:00";
/// Value used for every "today" in fixture mode.
pub const FIXED_DATE: &str = "2024-05-28";

/// Pin uuid, id and clock values to a deterministic sequence.
///
/// Call once from a test that writes golden files; afterwards every value the
/// builders generate stays the same between runs, so a test run no longer
/// dirties the fixtures.
pub fn use_fixed_fixtures() {
    FIXTURES.store(true, Ordering::Relaxed);
    SEQUENCE.store(0, Ordering::Relaxed);
}

pub fn is_fixed_fixtures() -> bool {
    FIXTURES.load(Ordering::Relaxed)
}

fn next_fixture_value() -> u64 {
    SEQUENCE.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn new_id() -> String {
    if is_fixed_fixtures() {
        return format!("fixture{:08}", next_fixture_value());
    }
    let mut write = ID_GEN
        .get_or_init(|| RwLock::new(ShortCodeGenerator::new_alphanumeric(8)))
        .write()
        .unwrap();
    write.next_string()
}

pub fn new_uuid() -> String {
    if is_fixed_fixtures() {
        return format!("00000000-0000-4000-8000-{:012}", next_fixture_value());
    }
    uuid::Uuid::new_v4().to_string()
}

pub fn now() -> String {
    if is_fixed_fixtures() {
        return FIXED_TIMESTAMP.to_string();
    }
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn today() -> String {
    if is_fixed_fixtures() {
        return FIXED_DATE.to_string();
    }
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[cfg(test)]
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
