pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn now() -> String {
    //2024-05-28T09:00:00
    // "2024-05-28T09:00:00".to_string()
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}