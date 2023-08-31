//! 文件元数据

use crate::system::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 文件通常名字
    pub name: String,
    /// 文件哈希值
    pub hash: String,
    /// 使用的哈希算法
    pub hash_algorithm: String,
    pub size: i64,
    pub created_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "FileSystemEntity")]
    FileSystem,
    #[sea_orm(has_many = "NodeDraftFileEntity")]
    NodeDraftFile,
    #[sea_orm(has_many = "NodeInstanceFileEntity")]
    NodeInstanceFile,
}

impl Related<StorageServerEntity> for Entity {
    // The final relation is Cake -> CakeFilling -> Filling
    fn to() -> RelationDef {
        FileStorageRelation::StorageServer.def()
    }

    fn via() -> Option<RelationDef> {
        // The original relation is CakeFilling -> Cake,
        // after `rev` it becomes Cake -> CakeFilling
        Some(FileStorageRelation::FileMetadata.def().rev())
    }
}

impl Related<FileSystemEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FileSystem.def()
    }
}

impl Related<NodeDraftFileEntity> for Entity {
    fn to() -> RelationDef {
        Relation::NodeDraftFile.def()
    }
}

impl Related<NodeInstanceFileEntity> for Entity {
    fn to() -> RelationDef {
        Relation::NodeInstanceFile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_set(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            name: Set(self.name),
            hash: Set(self.hash),
            hash_algorithm: Set(self.hash_algorithm),
            size: Set(self.size),
            created_time: Set(self.created_time),
        }
    }
}
