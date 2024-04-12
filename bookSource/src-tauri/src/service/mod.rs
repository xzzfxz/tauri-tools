use anyhow::Result;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    sync::Mutex,
};
use url::Url;

use crate::model::{RepeatRes, SourceItem};

lazy_static! {
    static ref REPEAT_LIST: Mutex<Vec<Value>> = Mutex::new(Vec::new());
}

/// 获取链接的域名
fn get_domain(url: String) -> String {
    let pre_url = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            println!("Url格式不正确: {}, {}", e, url);
            return "".to_string();
        }
    };
    let domain = match pre_url.domain() {
        Some(d) => d,
        None => {
            println!("域名不可用: {}", url);
            ""
        }
    };
    domain.to_string()
}

/**
 * @description: 书源去重
 * @param {*} Result
 * @return {*}
 */
pub fn delete_repeat(path_list: Vec<String>) -> Result<RepeatRes> {
    let mut pre_len = 0;
    let mut book_map: HashMap<String, Value> = HashMap::new();

    for file_path in path_list {
        let mut file = File::open(file_path)?;
        // 将文件内容转换成字符串
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // 转换为json格式的数据
        let raw_list: Vec<Value> = serde_json::from_str(&contents)?;
        pre_len = pre_len + raw_list.len();

        // 遍历生成map去重
        for item in raw_list.iter() {
            let source_item: SourceItem = serde_json::from_value(item.clone())?;

            // 搜索字段为空
            if source_item.search_url == None {
                continue;
            }

            let ori_url = source_item.book_source_url;
            if ori_url == "" {
                continue;
            }

            let domain = get_domain(ori_url);
            if domain == "" {
                continue;
            }

            // 判断更新时间
            if book_map.contains_key(&domain) {
                let pre_value = book_map.get(&domain).unwrap().clone();
                let pre_item: SourceItem = serde_json::from_value(pre_value.clone())?;
                // 使用最近更新的
                if pre_item.last_update_time < source_item.last_update_time {
                    book_map.insert(domain.clone(), item.clone());
                }
            } else {
                book_map.insert(domain, item.clone());
            }
        }
    }
    println!("去重前的长度: {}", pre_len);
    let mut last_list: Vec<Value> = book_map.into_values().collect();
    let cur_len = last_list.len();
    REPEAT_LIST.lock().unwrap().clear();
    REPEAT_LIST.lock().unwrap().append(&mut last_list);
    Ok(RepeatRes { pre_len, cur_len })
}

/**
 * @description: 下载文件
 * @param {String} save_path
 * @return {*}
 */
pub fn download_file(save_path: String) -> Result<String> {
    println!("{:?}", save_path);
    let last_list = REPEAT_LIST.lock().unwrap();
    let list_str = serde_json::to_string(&*last_list)?;
    let mut file = File::create(save_path)?;
    file.write_all(list_str.as_bytes())?;
    Ok(format!(""))
}
