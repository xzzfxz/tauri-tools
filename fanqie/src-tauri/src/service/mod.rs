use anyhow::Result;

/**
 * @description: 查询
 * @param {*} text  关键字
 * @return {*}
 */
pub fn search(text: &str) -> Result<()> {
    println!("search: {}", text);
    let url = format!("");

    // let client = reqwest::Client::new();
    // let res: reqwest::Response = client.get(url).send().await?;
    // let list = res.json::<Vec<Value>>().await?;
    // let pre_len = list.len();

    // delete_fn(list, &mut book_map)?;

    // let mut last_list: Vec<Value> = book_map.into_values().collect();
    // let cur_len = last_list.len();
    // REPEAT_LIST.lock().unwrap().clear();
    // REPEAT_LIST.lock().unwrap().append(&mut last_list);
    // Ok(RepeatRes { pre_len, cur_len })
    Ok(())
}
