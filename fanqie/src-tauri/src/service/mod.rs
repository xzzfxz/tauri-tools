mod tools;

use crate::model::{BookInfo, ResponseResult, SearchResult};
use anyhow::Result;
use reqwest::header::USER_AGENT;
use tools::turn_search_text;

/**
 * @description: 查询
 * @param {*} text  关键字
 * @return {*}
 */
pub async fn search(text: &str) -> Result<Vec<BookInfo>> {
    println!("search: {}", text);
    let url = format!(
        "https://fanqienovel.com/api/author/search/search_book/v1?\
        filter=127%2C127%2C127%2C127\
        &page_count=10\
        &page_index=0\
        &query_type=0\
        &query_word={}\
        &msToken=mUEYxyDzWRtBVovBxeK7k3hwOzaumwZ8Ko5zg4kCZHcRbhRbzwWNlK3Zhgx4dRpsWkpQAFnlhMbmWGFENpSgPKpNdGlNQo-rN_nK4aL3CvVuq22O885Q1x12vfszkYM%3D\
        &a_bogus=EJUmgO2IMsm1XE3adhkz9r4mIr60YW-LgZEzJpgELUwQ",
        text
    );
    let client = reqwest::Client::new();
    let res: reqwest::Response = client.get(url).header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36").send().await?;
    if res.status().is_success() {
        let result = res.json::<ResponseResult<SearchResult>>().await?;
        let mut book_list = result.data.search_book_data_list;
        let _ = turn_search_text(&mut book_list);
        println!("{:?}", book_list);
        return Ok(book_list);
    }
    Ok(vec![])
}
