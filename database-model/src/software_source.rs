//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "software_source")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub r#type: i32,
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cluster_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::installed_software::Entity")]
    InstalledSoftware,
}

impl Related<super::installed_software::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InstalledSoftware.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
