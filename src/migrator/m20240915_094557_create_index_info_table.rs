/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-09-15 17:45:57
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-09-15 18:08:48
 * @FilePath: /mutual-fund/src/migrator/m20240915_094557_create_index_info_table.rs
 * @Description: 指数信息
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
                    .table(IndexInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IndexInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IndexInfo::Code)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(IndexInfo::DisplayName).string().not_null())
                    .col(ColumnDef::new(IndexInfo::Name).string().not_null())
                    .col(ColumnDef::new(IndexInfo::StartDate).date_time().not_null())
                    .col(ColumnDef::new(IndexInfo::EndDate).date_time().not_null())
                    .col(ColumnDef::new(IndexInfo::Type).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IndexInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IndexInfo {
    Table,
    Id,
    Code,
    DisplayName,
    Name,
    StartDate,
    EndDate,
    Type,
}
