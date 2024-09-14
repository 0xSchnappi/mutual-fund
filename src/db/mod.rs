/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-11 10:28:02
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-14 09:42:35
 * @FilePath: /mutual-fund/src/db/mod.rs
 * @Description: postgres数据库操作
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

use sea_orm::*;

use crate::AppConfig;

pub(super) async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database,
    ));

    println!(
        "postgres://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database,
    );

    opts.sqlx_logging(false); // 关闭日志

    Database::connect(opts).await
}
