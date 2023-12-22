//! NOTE: Creating tables must be in the opposite order to droping tables.

use sea_orm_migration::prelude::*;

use crate::schema::Statements;
use crate::tables::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("m20230823_000001_init_migration/create_triggers_and_functions.sql");
        let db = manager.get_connection();
        Statements::create_table()
            .push::<Region>()
            .push::<AvailableZone>()
            .push::<Cluster>()
            .push::<Dictionary>()
            .push::<DictionaryValue>()
            .push::<FileMetadata>()
            .push::<StorageServer>()
            .push::<FileStorage>()
            .push::<FileSystem>()
            .push::<Project>()
            .push::<FlowDraft>()
            .push::<FlowInstance>()
            .push::<Queue>()
            .push::<NodeInstance>()
            .push::<Task>()
            .push::<FileTransmit>()
            .push::<FlowInstanceBilling>()
            .push::<FlowTemplate>()
            .push::<SoftwareSource>()
            .push::<InstalledSoftware>()
            .push::<LocalSoftwareSource>()
            .push::<NodeDraftFile>()
            .push::<NodeInstanceBilling>()
            .push::<Notification>()
            .push::<Provider>()
            .push::<Org>()
            .push::<ProjectUser>()
            .push::<ProviderList>()
            .push::<ProviderUsers>()
            .push::<QueueBillConfig>()
            .push::<SoftwareBlockList>()
            .push::<SoftwareInstallHistory>()
            .push::<UserComponent>()
            .push::<UserFavorite>()
            .push::<UserInformation>()
            .push::<UserLog>()
            .push::<UserResource>()
            .push::<WorkOrder>()
            .exec(manager)
            .await?;
        db.execute_unprepared(sql).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = include_str!("m20230823_000001_init_migration/drop_functions.sql");
        let db = manager.get_connection();
        Statements::drop_table()
            .push::<WorkOrder>()
            .push::<UserResource>()
            .push::<UserLog>()
            .push::<UserInformation>()
            .push::<UserFavorite>()
            .push::<UserComponent>()
            .push::<SoftwareInstallHistory>()
            .push::<SoftwareBlockList>()
            .push::<QueueBillConfig>()
            .push::<ProviderUsers>()
            .push::<ProviderList>()
            .push::<ProjectUser>()
            .push::<Org>()
            .push::<Provider>()
            .push::<Notification>()
            .push::<NodeInstanceBilling>()
            .push::<NodeDraftFile>()
            .push::<LocalSoftwareSource>()
            .push::<InstalledSoftware>()
            .push::<SoftwareSource>()
            .push::<FlowTemplate>()
            .push::<FlowInstanceBilling>()
            .push::<FileTransmit>()
            .push::<Task>()
            .push::<NodeInstance>()
            .push::<Queue>()
            .push::<FlowInstance>()
            .push::<FlowDraft>()
            .push::<Project>()
            .push::<FileSystem>()
            .push::<FileStorage>()
            .push::<StorageServer>()
            .push::<FileMetadata>()
            .push::<DictionaryValue>()
            .push::<Dictionary>()
            .push::<Cluster>()
            .push::<AvailableZone>()
            .push::<Region>()
            .exec(manager)
            .await?;
        db.execute_unprepared(sql).await?;
        Ok(())
    }
}
