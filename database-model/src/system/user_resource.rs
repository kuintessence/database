//! 用户队列资源

use crate::system::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_resource")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    /// 已使用内存大小，单位为字节
    pub memory: Option<i64>,
    /// 设置内存可用最大大小，单位为字节
    pub memory_max: Option<i64>,
    /// 使用达到多少时（单位为字节）向算力提供者以及算力运营者告警
    pub memory_alert: Option<i64>,
    /// 核心数
    pub core_number: Option<i64>,
    pub core_number_max: Option<i64>,
    pub core_number_alert: Option<i64>,
    /// 存储空间大小，单位为字节
    pub storage_capacity: Option<i64>,
    pub storage_capacity_max: Option<i64>,
    pub storage_capacity_alert: Option<i64>,
    pub queue_id: Uuid,
    pub max_pending_task_count: Option<i64>,
    pub max_node_count: Option<i64>,
    pub max_running_task_count: Option<i64>,
    /// 已使用的节点数
    pub node_count: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cluster::Entity",
        from = "Column::QueueId",
        to = "super::cluster::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Cluster,
}

impl Related<ClusterEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Cluster.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
