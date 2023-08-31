//! 集群

use crate::system::prelude::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cluster")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub available_zone_id: Uuid,
    pub provider_id: Option<Uuid>,
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
    #[sea_orm(has_many = "super::queue::Entity")]
    Queue,
    #[sea_orm(has_many = "super::user_resource::Entity")]
    UserResource,
}

impl Related<AvailableZoneEntity> for Entity {
    fn to() -> RelationDef {
        Relation::AvailableZone.def()
    }
}

impl Related<QueueEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Queue.def()
    }
}

impl Related<UserResourceEntity> for Entity {
    fn to() -> RelationDef {
        Relation::UserResource.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
