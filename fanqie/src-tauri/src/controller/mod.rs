use crate::service;

// 查询
#[tauri::command]
pub fn search(text: String) -> String {
    let _ = service::search(&text);
    return text;
}
