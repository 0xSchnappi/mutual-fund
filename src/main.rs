/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-08-30 11:21:32
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-11 11:25:21
 * @FilePath: /mutual-fund/src/main.rs
 * @Description: 启动
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

use core::panic;

#[macro_use]
extern crate rocket;

mod db;
mod fairings;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_sercert: String,
}

impl AppConfig {
    fn new() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("bookstore".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD")
                .unwrap_or("ZhangYing.730298".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("bookstore".to_string()),
            jwt_sercert: std::env::var("BOOKSTORE_JWT_SECRET")
                .expect("Please set the BOOKSTORE_JWT_SECRET env variable."),
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello 0xSchnappi QT"
}

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::new();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("[-]数据库连接失败{}", err),
    };

    rocket::build()
        .manage(db)
        .manage(config)
        .attach(fairings::cors::CORS)
        .mount("/", routes![index])
}
