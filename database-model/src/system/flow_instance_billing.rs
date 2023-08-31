use sea_orm::entity::prelude::*;
use sea_orm::{Set, Unchanged};

use crate::system::prelude::*;
use crate::utils::WithDecimalFileds;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "flow_instance_billing")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub flow_instance_id: Uuid,
    pub cpu: i64,
    pub memory: i64,
    pub storage: i64,
    pub cpu_time: i64,
    pub wall_time: i64,
    #[sea_orm(column_type = "Decimal(Some((12, 2)))")]
    pub total_price: Decimal,
    pub user_id: Uuid,
    pub created_time: DateTimeUtc,
    pub modified_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "FlowInstanceEntity",
        from = "Column::FlowInstanceId",
        to = "FlowInstanceColumn::Id"
    )]
    FlowInstance,
}
impl Related<FlowInstanceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowInstance.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_set(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            flow_instance_id: Set(self.flow_instance_id),
            cpu: Set(self.cpu),
            memory: Set(self.memory),
            storage: Set(self.storage),
            cpu_time: Set(self.cpu_time),
            wall_time: Set(self.wall_time),
            total_price: Set(self.total_price),
            user_id: Set(self.user_id),
            created_time: Unchanged(self.created_time),
            modified_time: Unchanged(self.modified_time),
        }
    }
}

impl WithDecimalFileds for Model {
    fn rescale_all_to(&mut self, n: u32) {
        self.total_price.rescale(n);
    }
}
