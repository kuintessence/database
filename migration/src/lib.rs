pub mod schema;
mod tables;
mod versions;

use crate::versions::*;
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230823_000001_init_migration::Migration),
            Box::new(m20231110_000001_add_task::Migration),
        ]
    }
}
