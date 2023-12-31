//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "dictionary_value")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub key: String,
    pub value: i32,
    pub parent_id: Option<Uuid>,
    pub dictionary_id: Uuid,
    pub created_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dictionary::Entity",
        from = "Column::DictionaryId",
        to = "super::dictionary::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Dictionary,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
}

impl Related<super::dictionary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dictionary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
