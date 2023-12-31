//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "dictionary")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: bool,
    pub created_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::dictionary_value::Entity")]
    DictionaryValue,
}

impl Related<super::dictionary_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DictionaryValue.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
