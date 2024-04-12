use crate::{
    model::{RepeatRes, ResponseResult},
    service,
};

/// 去重
#[tauri::command]
pub fn delete_repeat(path_list: Vec<String>) -> ResponseResult<RepeatRes> {
    let res = match service::delete_repeat(path_list) {
        Ok(res) => ResponseResult {
            code: 0,
            msg: "success".to_string(),
            result: res,
        },
        Err(info) => {
            println!("去重发生错误：{:#?}", info);
            ResponseResult {
                code: 1,
                msg: format!("去重发生错误：{:#?}", info),
                result: RepeatRes {
                    pre_len: 0,
                    cur_len: 0,
                },
            }
        }
    };
    res
}

/// 下载文件到本地
#[tauri::command]
pub fn download_file(save_path: String) -> ResponseResult<String> {
    let res = match service::download_file(save_path) {
        Ok(_) => ResponseResult {
            code: 0,
            msg: "success".to_string(),
            result: format!(""),
        },
        Err(info) => {
            println!("下载出现错误：{:#?}", info);
            ResponseResult {
                code: 1,
                msg: format!("下载出现错误{:#?}", info),
                result: format!(""),
            }
        }
    };
    res
}
