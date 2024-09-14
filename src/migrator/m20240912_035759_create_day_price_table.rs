/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-12 11:57:59
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-13 17:48:30
 * @FilePath: /mutual-fund/src/migrator/m20240912_035759_create_DayPrice_table.rs
 * @Description: 股票日价格表
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_stock_info_table::StockInfo;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(DayPrice::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DayPrice::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DayPrice::Code).string().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk-stock_info-code")
                        .from(DayPrice::Table, DayPrice::Code)
                        .to(StockInfo::Table, StockInfo::Code)
                    )
                    
                    .col(ColumnDef::new(DayPrice::DayTime).date_time().not_null())
                    .col(ColumnDef::new(DayPrice::Open).double().not_null())
                    .col(ColumnDef::new(DayPrice::Close).double().not_null())
                    .col(ColumnDef::new(DayPrice::High).double().not_null())
                    .col(ColumnDef::new(DayPrice::Low).double().not_null())
                    .col(ColumnDef::new(DayPrice::Volume).double().not_null())
                    .col(ColumnDef::new(DayPrice::Money).double().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(DayPrice::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DayPrice {
    Table,
    Id,
    Code,
    DayTime,
    Open,
    Close,
    High,
    Low,
    Volume,
    Money,
}
