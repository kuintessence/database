//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "file_metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub hash: String,
    pub hash_algorithm: String,
    pub size: i64,
    pub created_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::file_storage::Entity")]
    FileStorage,
    #[sea_orm(has_many = "super::file_system::Entity")]
    FileSystem,
    #[sea_orm(has_many = "super::file_transmit::Entity")]
    FileTransmit,
    #[sea_orm(has_many = "super::node_draft_file::Entity")]
    NodeDraftFile,
}

impl Related<super::file_storage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileStorage.def()
    }
}

impl Related<super::file_system::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileSystem.def()
    }
}

impl Related<super::file_transmit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileTransmit.def()
    }
}

impl Related<super::node_draft_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NodeDraftFile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
