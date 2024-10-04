use once_cell::sync::OnceCell;
use std::sync::RwLock;
use tiny_id::ShortCodeGenerator;

static ID_GEN: OnceCell<RwLock<ShortCodeGenerator<char>>> = OnceCell::new();

pub fn new_id() -> String {
    let mut w = ID_GEN
        .get_or_init(|| RwLock::new(ShortCodeGenerator::new_alphanumeric(8)))
        .write()
        .unwrap();
    w.next_string()
}

pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn now() -> String {
    //2024-05-28T09:00:00
    // "2024-05-28T09:00:00".to_string()
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}
