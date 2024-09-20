use anyhow::Result;
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
        filter=127.127.127.127\
        &page_count=10\
        &page_index=0\
        &query_word={}\
        &msToken=mUEYxyDzWRtBVovBxeK7k3hwOzaumwZ8Ko5zg4kCZHcRbhRbzwWNlK3Zhgx4dRpsWkpQAFnlhMbmWGFENpSgPKpNdGlNQo-\
        &a_bogus=EJUmgO2IMsm1XE3adhkz9r4mIr60YW-LgZEzJpgELUwQ",
        text
    );
    let client = reqwest::Client::new();
    let res: reqwest::Response = client.get(url).send().await?;
    let list = res.json::<Vec<Value>>().await?;
    println!("{:?}", list);

    // delete_fn(list, &mut book_map)?;

    // let mut last_list: Vec<Value> = book_map.into_values().collect();
    // let cur_len = last_list.len();
    // REPEAT_LIST.lock().unwrap().clear();
    // REPEAT_LIST.lock().unwrap().append(&mut last_list);
    // Ok(RepeatRes { pre_len, cur_len })
    Ok(())
}
