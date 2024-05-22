use bottlerocket_settings_derive::SettingsPlugin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use model::{
    BootSettings, BootstrapContainer, CloudFormationSettings, DnsSettings, HostContainer,
    MetricsSettings, NetworkSettings, OciHooks, PemCertificate,
};
use modeled_types::Identifier;

// Note: we have to use 'rename' here because the top-level Settings structure is the only one
// that uses its name in serialization; internal structures use the field name that points to it
#[derive(Debug, Default, Clone, PartialEq, SettingsPlugin, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", rename = "settings")]
struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    motd: Option<settings_extension_motd::MotdV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updates: Option<settings_extension_updates::UpdatesSettingsV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_containers: Option<HashMap<Identifier, HostContainer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_containers: Option<HashMap<Identifier, BootstrapContainer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ntp: Option<settings_extension_ntp::NtpSettingsV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<NetworkSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernel: Option<settings_extension_kernel::KernelSettingsV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot: Option<BootSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws: Option<settings_extension_aws::AwsSettingsV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<MetricsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pki: Option<HashMap<Identifier, PemCertificate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_registry: Option<settings_extension_container_registry::RegistrySettingsV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_hooks: Option<OciHooks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudformation: Option<CloudFormationSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns: Option<DnsSettings>,
}
