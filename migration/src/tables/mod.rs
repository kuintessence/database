mod available_zone;
mod cluster;
mod dictionary;
mod dictionary_value;
mod file_metadata;
mod file_storage;
mod file_system;
mod file_transmit;
mod flow_draft;
mod flow_instance;
mod flow_instance_billing;
mod flow_template;
mod installed_software;
mod local_software_source;
mod node_draft_file;
mod node_instance;
mod node_instance_billing;
mod notification;
mod org;
mod project;
mod project_user;
mod provider;
mod provider_list;
mod provider_users;
mod queue;
mod queue_bill_config;
mod region;
mod software_block_list;
mod software_install_history;
mod software_source;
mod storage_server;
mod task;
mod user_component;
mod user_favorite;
mod user_infomation;
mod user_log;
mod user_resource;
mod work_order;

#[rustfmt::skip]
pub use self::{
    available_zone::AvailableZone,
    cluster::Cluster,
    dictionary::Dictionary,
    dictionary_value::DictionaryValue,
    file_metadata::FileMetadata,
    file_storage::FileStorage,
    file_system::FileSystem,
    file_transmit::FileTransmit,
    flow_draft::FlowDraft,
    flow_instance::FlowInstance,
    flow_instance_billing::FlowInstanceBilling,
    flow_template::FlowTemplate,
    installed_software::InstalledSoftware,
    local_software_source::LocalSoftwareSource,
    node_draft_file::NodeDraftFile,
    node_instance::NodeInstance,
    node_instance_billing::NodeInstanceBilling,
    notification::Notification,
    org::Org,
    project::Project,
    project_user::ProjectUser,
    provider::Provider,
    provider_list::ProviderList,
    provider_users::ProviderUsers,
    queue::Queue,
    queue_bill_config::QueueBillConfig,
    region::Region,
    software_block_list::SoftwareBlockList,
    software_install_history::SoftwareInstallHistory,
    software_source::SoftwareSource,
    storage_server::StorageServer,
    task::Task,
    user_component::UserComponent,
    user_favorite::UserFavorite,
    user_infomation::UserInformation,
    user_log::UserLog,
    user_resource::UserResource,
    work_order::WorkOrder,
};

use sea_orm_migration::prelude::*;

const TRANSACTION_TIMESTAMP: &str = "transaction_timestamp()";

/// UUID primary key initialized with random value
#[inline]
fn uuid_pkey(name: impl IntoIden) -> ColumnDef {
    let mut cd = ColumnDef::new(name);
    cd.uuid().primary_key().default(Expr::cust("gen_random_uuid()"));
    cd
}
