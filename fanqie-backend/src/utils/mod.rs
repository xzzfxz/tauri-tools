mod usual;

use serde_json::to_string;
use usual::{CHARSET, CODE_ED, CODE_ST, CONFIG};

fn turn_text(code_num: u32) -> String {
    let index = code_num as i32 - CODE_ST;
    if index >= CHARSET.len() as i32 || index < 0 {
        format!("{}", char::from_u32(code_num).unwrap())
    } else {
        CHARSET[index as usize].to_string()
    }
}

// 将正文乱码转为正常文字
pub fn get_true_text(text: &str) -> String {
    let mut last_text = "".to_string();
    for c in text.chars() {
        let code_num = c as u32;
        let mut text = c.to_string();
        if code_num >= CODE_ST as u32 && code_num <= CODE_ED {
            text = turn_text(code_num);
        }
        last_text.push_str(&text);
    }
    last_text
}

// 将标题乱码转为正常文字
pub fn get_title_true_text(text: &str) -> String {
    let config = CONFIG.lock().unwrap();
    let mut last_text = "".to_string();
    for c in text.chars() {
        let code_str = (c as u32).to_string();
        let char_str = c.to_string();
        let res = config.get(&code_str).unwrap_or(&char_str);
        last_text.push_str(res);
    }
    last_text
}
