use actix_web::{get, web, HttpResponse, Responder};

use crate::model::{body::SearchBody, ResponseInterface};
use crate::service;

// 查询
#[get("/api/search")]
pub async fn search(params: web::Query<SearchBody>) -> impl Responder {
    let res = match service::search(&params.text).await {
        Ok(list) => {
            println!("成功");
            ResponseInterface {
                code: 0,
                msg: "success".to_string(),
                result: list,
            }
        }
        Err(info) => {
            println!("错误：{:#?}", info);
            ResponseInterface {
                code: -1,
                msg: "error".to_string(),
                result: vec![],
            }
        }
    };
    HttpResponse::Ok().json(res)
}
