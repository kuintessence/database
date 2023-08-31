//! 工作流实例

use crate::system::prelude::*;
use sea_orm::entity::prelude::*;

// use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "flow_instance")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub logo: Option<String>,
    pub status: i32,
    pub spec: Json,
    pub user_id: Uuid,
    pub created_time: DateTimeUtc,
    pub last_modified_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "NodeInstanceEntity")]
    NodeInstance,
    #[sea_orm(has_one = "FlowInstanceBillingEntity")]
    FlowInstanceBilling,
}

impl Related<NodeInstanceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::NodeInstance.def()
    }
}
impl Related<FlowInstanceBillingEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowInstanceBilling.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
