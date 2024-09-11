/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-11 14:18:53
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-11 14:51:13
 * @FilePath: /mutual-fund/src/migrator/mod.rs
 * @Description: 数据库接口
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_stock_info_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20220101_000001_create_stock_info_table::Migration,
        )]
    }
}
