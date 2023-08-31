use sea_orm_migration::prelude::*;

use crate::schema::SchemaTable;

use super::{uuid_pkey, Org, Provider};

#[derive(Iden)]
pub enum ProviderUsers {
    Table,

    Id,
    ProviderId,
    UserId,
    Role,
    OrgId,
    Enabled,
    ManageCluster,
    UseCluster,
}

impl SchemaTable for ProviderUsers {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::ProviderId).uuid().not_null())
            .col(ColumnDef::new(Self::UserId).uuid().not_null())
            .col(ColumnDef::new(Self::Role).integer().not_null().default(0))
            .col(ColumnDef::new(Self::OrgId).uuid())
            .col(ColumnDef::new(Self::Enabled).boolean().default(true))
            .col(ColumnDef::new(Self::ManageCluster).json_binary())
            .col(ColumnDef::new(Self::UseCluster).json_binary())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::OrgId)
                    .to(Org::Table, Org::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ProviderId)
                    .to(Provider::Table, Provider::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}

// 	TODO:
// 	CONSTRAINT provider_users_org_id_fkey FOREIGN KEY (org_id) REFERENCES public.org(id) ON DELETE RESTRICT ON UPDATE RESTRICT,
// 	CONSTRAINT provider_users_provider_id_fkey FOREIGN KEY (provider_id) REFERENCES public.provider_list(id) ON DELETE RESTRICT ON UPDATE RESTRICT
