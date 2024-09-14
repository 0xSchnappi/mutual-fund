/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-11 14:18:53
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-13 16:16:46
 * @FilePath: /mutual-fund/src/migrator/m20220101_000001_create_StockInfo_table.rs
 * @Description: 接口
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StockInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StockInfo::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(StockInfo::Name).string().not_null())
                    .col(ColumnDef::new(StockInfo::DisplayName).string().not_null())
                    .col(ColumnDef::new(StockInfo::StartDate).string().not_null())
                    .col(ColumnDef::new(StockInfo::EndDate).string().not_null())
                    .col(ColumnDef::new(StockInfo::Type).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum StockInfo {
    Table,
    Id,
    Code,
    Name,
    DisplayName,
    StartDate,
    EndDate,
    Type,
}
