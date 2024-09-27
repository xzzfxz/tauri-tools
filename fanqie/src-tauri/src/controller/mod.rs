use crate::model::{BookInfo, ResponseInterface};
use crate::service;

// 查询
#[tauri::command]
pub async fn search(text: String) -> ResponseInterface<Vec<BookInfo>> {
    match service::search(&text).await {
        Ok(list) => {
            println!("成功");
            ResponseInterface {
                code: 0,
                msg: "success".to_string(),
                result: list,
            }
        }
        Err(info) => {
            println!("错误：{:#?}", info);
            ResponseInterface {
                code: -1,
                msg: "error".to_string(),
                result: vec![],
            }
        }
    }
}
