use serde::{Deserialize, Serialize};

pub mod body;

// 接口返回结果
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseResult<T> {
    pub code: i32,
    pub data: T,
    pub message: String,
}

// 搜索结果
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub total_count: u32,
    pub search_book_data_list: Vec<BookInfo>,
}

// 搜索书籍信息
#[derive(Serialize, Deserialize, Debug)]
pub struct BookInfo {
    pub author: String,
    /// 简介
    pub book_abstract: String,
    pub book_id: String,
    pub book_name: String,
    pub category: String,
    pub first_chapter_id: String,
    pub last_chapter_id: String,
    pub last_chapter_time: String,
    pub last_chapter_title: String,
    pub thumb_url: String,
    pub word_count: u32,
}

// 标题字体映射
#[derive(Serialize, Deserialize, Debug)]
pub struct TitleFont {
    pub code: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseInterface<T> {
    pub code: i32,
    pub msg: String,
    pub result: T,
}
