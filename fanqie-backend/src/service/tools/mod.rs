use crate::utils::get_title_true_text;
use anyhow::Result;

use crate::model::BookInfo;

pub fn turn_search_text(book_list: &mut Vec<BookInfo>) -> Result<()> {
    // println!("initList: {:#?}", book_list);
    for book in book_list.iter_mut() {
        book.book_name = get_title_true_text(&book.book_name);
        book.author = get_title_true_text(&book.author);
        book.book_abstract = get_title_true_text(&book.book_abstract);
    }
    Ok(())
}
