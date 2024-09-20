use anyhow::Result;

/**
 * @description: 查询
 * @param {*} text  关键字
 * @return {*}
 */
pub fn search(text: &str) -> Result<()> {
    println!("search: {}", text);
    Ok(())
}
