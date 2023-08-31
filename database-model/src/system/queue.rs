//! 队列资源

use crate::system::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "queue")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub topic_name: String,
    /// 内存大小，单位为字节
    pub memory: i64,
    /// 使用达到多少时（单位为字节）向算力提供者以及算力运营者告警
    pub memory_alert: Option<i64>,
    /// 核心个数
    pub core_number: i64,
    pub core_number_alert: Option<i64>,
    /// 存储空间大小，单位为字节
    pub storage_capacity: i64,
    pub storage_capacity_alert: Option<i64>,
    pub node_count: i64,
    pub max_node_count: Option<i64>,
    pub max_queuing_task_count: Option<i64>,
    pub max_running_task_count: Option<i64>,
    pub cluster_id: Option<Uuid>,
    pub provider_id: Option<Uuid>,
    pub description: Option<String>,
    pub scheduler_tech: i64,
    pub enabled: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cluster::Entity",
        from = "Column::ClusterId",
        to = "super::cluster::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Cluster,
    #[sea_orm(has_many = "super::node_instance::Entity")]
    NodeInstance,
    #[sea_orm(has_one = "super::queue_bill_config::Entity")]
    QueueBillConfig,
}

impl Related<ClusterEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Cluster.def()
    }
}

impl Related<NodeInstanceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::NodeInstance.def()
    }
}

impl Related<QueueBillConfigEntity> for Entity {
    fn to() -> RelationDef {
        Relation::QueueBillConfig.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
