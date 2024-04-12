use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceItem {
    /// 书源标识
    #[serde(rename = "bookSourceUrl")]
    pub book_source_url: String,
    /// 上次更新时间
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: u64,
    #[serde(rename = "searchUrl")]
    /// 搜索url
    pub search_url: Option<String>,
}

// 返回前端的结果
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseResult<T> {
    pub code: i32,
    pub msg: String,
    pub result: T,
}

// 去重结果
#[derive(Debug, Deserialize, Serialize)]
pub struct RepeatRes {
    #[serde(rename = "preLen")]
    pub pre_len: usize,
    #[serde(rename = "curLen")]
    pub cur_len: usize,
}
