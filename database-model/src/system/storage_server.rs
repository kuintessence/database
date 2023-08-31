//! 存储服务器

use crate::system::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "storage_server")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    /// 存储服务器配置选项
    pub options: Json,
    /// 存储容量
    pub capacity: String,
    pub storage_type: i32,
    pub available_zone_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::available_zone::Entity",
        from = "Column::AvailableZoneId",
        to = "super::available_zone::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AvailableZone,
    #[sea_orm(has_many = "super::file_storage::Entity")]
    FileStorage,
}

impl Related<AvailableZoneEntity> for Entity {
    fn to() -> RelationDef {
        Relation::AvailableZone.def()
    }
}

impl Related<FileStorageEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FileStorage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
