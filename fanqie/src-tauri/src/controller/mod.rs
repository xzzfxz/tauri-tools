use crate::service;

// 查询
#[tauri::command]
pub async fn search(text: String) -> String {
    match service::search(&text).await {
        Ok(_) => {
            println!("成功");
        }
        Err(info) => {
            println!("错误：{:#?}", info)
        }
    };
    return text;
}
