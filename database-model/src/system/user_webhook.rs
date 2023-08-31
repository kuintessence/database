use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_webhook")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn into_set(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            user_id: Set(self.user_id),
            url: Set(self.url),
        }
    }
}
