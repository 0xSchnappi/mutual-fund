/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-08-30 11:21:32
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-14 09:42:22
 * @FilePath: /mutual-fund/src/main.rs
 * @Description: 启动
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

use core::panic;

use migrator::Migrator;
use sea_orm_migration::MigratorTrait;

#[macro_use]
extern crate rocket;

mod db;
mod fairings;
mod migrator;

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
            db_host: std::env::var("FUND_DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("FUND_DB_PORT").unwrap_or("5432".to_string()),
            db_username: std::env::var("FUND_DB_USERNAME").unwrap_or("postgres".to_string()),
            db_password: std::env::var("FUND_DB_PASSWORD").unwrap_or("postgres".to_string()),
            db_database: std::env::var("FUND_DB_DATABASE").unwrap_or("postgres".to_string()),
            jwt_sercert: std::env::var("FUND_JWT_SECRET")
                .expect("Please set the FUND_JWT_SECRET env variable."),
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

    match Migrator::up(&db, None).await {
        Ok(_) => (),
        Err(err) => panic!("[-] 数据库迁移失败{}", err),
    };

    // match Migrator::down(&db, None).await {
    //     Ok(_) => (),
    //     Err(err) => panic!("[-] 数据库迁移失败{}", err),
    // };

    rocket::build()
        .manage(db)
        .manage(config)
        .attach(fairings::cors::CORS)
        .mount("/", routes![index])
}
