//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "price_candle")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub token: String,
    #[sea_orm(column_type = "Float")]
    pub open: f32,
    #[sea_orm(column_type = "Float")]
    pub high: f32,
    #[sea_orm(column_type = "Float")]
    pub low: f32,
    #[sea_orm(column_type = "Float")]
    pub close: f32,
    pub timestamp: i64,
    pub period: String,
    pub chain_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
