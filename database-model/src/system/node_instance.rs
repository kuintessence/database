//! 工作流节点实例

use sea_orm::entity::prelude::*;
use sea_orm::Set;

use crate::system::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "node_instance")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub kind: i32,
    pub is_parent: bool,
    pub batch_parent_id: Option<Uuid>,
    pub status: i32,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub resource_meter: Option<Json>,
    pub log: Option<String>,
    pub queue_id: Option<Uuid>,
    pub flow_instance_id: Uuid,
    pub created_time: DateTimeWithTimeZone,
    pub last_modified_time: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::flow_instance::Entity",
        from = "Column::FlowInstanceId",
        to = "super::flow_instance::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FlowInstance,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::BatchParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(
        belongs_to = "super::queue::Entity",
        from = "Column::QueueId",
        to = "super::queue::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Queue,
}

impl Related<FlowInstanceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowInstance.def()
    }
}

impl Related<QueueEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Queue.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_set(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            name: Set(self.name),
            kind: Set(self.kind),
            is_parent: Set(self.is_parent),
            batch_parent_id: Set(self.batch_parent_id),
            status: Set(self.status),
            resource_meter: Set(self.resource_meter),
            log: Set(self.log),
            queue_id: Set(self.queue_id),
            flow_instance_id: Set(self.flow_instance_id),
            created_time: sea_orm::ActiveValue::Unchanged(self.created_time),
            last_modified_time: sea_orm::ActiveValue::Unchanged(self.last_modified_time),
        }
    }
}
