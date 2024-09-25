mod usual;

use usual::{CHARSET, CODE_ED, CODE_ST};

fn turn_text(code_num: u32) -> String {
    let index = code_num as i32 - CODE_ST;
    if index >= CHARSET.len() as i32 || index < 0 {
        format!("{}", char::from_u32(code_num).unwrap())
    } else {
        CHARSET[index as usize].to_string()
    }
}

// 将乱码转为正常文字
pub fn get_true_text(text: &str) -> String {
    println!("text:{}", text);
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
