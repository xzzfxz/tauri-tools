use anyhow::Result;
use reqwest::header::USER_AGENT;
use serde_json::Value;

/**
 * @description: 查询
 * @param {*} text  关键字
 * @return {*}
 */
pub async fn search(text: &str) -> Result<()> {
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
        let text = res.text().await?;
        println!("{:?}", text);
    }

    // delete_fn(list, &mut book_map)?;

    // let mut last_list: Vec<Value> = book_map.into_values().collect();
    // let cur_len = last_list.len();
    // REPEAT_LIST.lock().unwrap().clear();
    // REPEAT_LIST.lock().unwrap().append(&mut last_list);
    // Ok(RepeatRes { pre_len, cur_len })
    Ok(())
}
