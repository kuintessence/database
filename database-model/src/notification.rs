//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "notification")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub r#type: i32,
    pub priority: i32,
    pub is_read: bool,
    pub related_item_id: Uuid,
    pub user_id: Uuid,
    pub is_deleted: bool,
    pub created_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
