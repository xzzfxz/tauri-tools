use crate::utils::get_true_text;
use anyhow::Result;

use crate::model::BookInfo;

pub fn turn_search_text(book_list: &mut Vec<BookInfo>) -> Result<()> {
    // println!("initList: {:#?}", book_list);
    for book in book_list.iter_mut() {
        let name = get_true_text("雨傍晚，冷卷落叶肆虐座城市，瑶撑伞站市妇幼医院，潮涌，苍。");
        println!("书名：{}, {}", book.book_name, name);
    }
    Ok(())
}
