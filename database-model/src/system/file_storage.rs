//! 文件副本，由文件元数据与存储服务器共同标识

use sea_orm::entity::prelude::*;
use sea_orm::Set;

use crate::system::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "file_storage")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub storage_server_id: Uuid,
    pub file_metadata_id: Uuid,
    /// 文件副本在存储服务器中的 uri
    pub server_url: String,
    pub created_time: Option<DateTimeWithTimeZone>,
    pub created_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file_metadata::Entity",
        from = "Column::FileMetadataId",
        to = "super::file_metadata::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FileMetadata,
    #[sea_orm(
        belongs_to = "super::storage_server::Entity",
        from = "Column::StorageServerId",
        to = "super::storage_server::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    StorageServer,
}

impl Related<FileMetadataEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FileMetadata.def()
    }
}

impl Related<StorageServerEntity> for Entity {
    fn to() -> RelationDef {
        Relation::StorageServer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_set(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            storage_server_id: Set(self.storage_server_id),
            file_metadata_id: Set(self.file_metadata_id),
            server_url: Set(self.server_url),
            created_time: Set(self.created_time),
            created_user_id: Set(self.created_user_id),
        }
    }
}
