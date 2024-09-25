use crate::utils::get_true_text;
use anyhow::Result;

use crate::model::BookInfo;

pub fn turn_search_text(book_list: &mut Vec<BookInfo>) -> Result<()> {
    // println!("initList: {:#?}", book_list);
    for book in book_list.iter_mut() {
        // let name = get_true_text("超级坦科");
        println!("书名：{}", book.book_name,);
    }
    Ok(())
}
