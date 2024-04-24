/*!
# API models

Bottlerocket has different variants supporting different features and use cases.
Each variant has its own set of software, and therefore needs its own configuration.
We support having an API model for each variant to support these different configurations.

Each model defines a top-level `Settings` structure.
It can use pre-defined structures inside, or custom ones as needed.

This `Settings` essentially becomes the schema for the variant's data store.
`apiserver::datastore` offers serialization and deserialization modules that make it easy to map between Rust types and the data store, and thus, all inputs and outputs are type-checked.

At the field level, standard Rust types can be used, or ["modeled types"](src/modeled_types) that add input validation.

Default values are specified in .toml files in each variant's `defaults.d` directory under [src](src).
(For example, see the [aws-ecs-1 defaults](src/aws-ecs-1/defaults.d/).)
Entries are sorted by filename, and later entries take precedence.

The `#[model]` attribute on Settings and its sub-structs reduces duplication and adds some required metadata; see [its docs](model-derive/) for details.

## aws-k8s-1.23: Kubernetes 1.23

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

## aws-k8s-1.23-nvidia: Kubernetes 1.23 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

## aws-k8s-1.24: Kubernetes 1.24

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

## aws-k8s-1.24-nvidia: Kubernetes 1.24 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

## aws-k8s-1.25: Kubernetes 1.25

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

## aws-k8s-1.25-nvidia: Kubernetes 1.25 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

## aws-k8s-1.26: Kubernetes 1.26

* [Model](src/aws-k8s-1.26/mod.rs)
* [Default settings](src/aws-k8s-1.26/defaults.d/)

## aws-k8s-1.26-nvidia: Kubernetes 1.26 NVIDIA

* [Model](src/aws-k8s-1.26-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.26-nvidia/defaults.d/)

## aws-k8s-1.27: Kubernetes 1.27

* [Model](src/aws-k8s-1.28/mod.rs)
* [Default settings](src/aws-k8s-1.28/defaults.d/)

## aws-k8s-1.27-nvidia: Kubernetes 1.27 NVIDIA

* [Model](src/aws-k8s-1.28-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.28-nvidia/defaults.d/)

## aws-k8s-1.28: Kubernetes 1.28

* [Model](src/aws-k8s-1.30/mod.rs)
* [Default settings](src/aws-k8s-1.30/defaults.d/)

## aws-k8s-1.28-nvidia: Kubernetes 1.28 NVIDIA

* [Model](src/aws-k8s-1.30-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.30-nvidia/defaults.d/)

## aws-k8s-1.29: Kubernetes 1.29

* [Model](src/aws-k8s-1.30/mod.rs)
* [Default settings](src/aws-k8s-1.30/defaults.d/)

## aws-k8s-1.29-nvidia: Kubernetes 1.29 NVIDIA

* [Model](src/aws-k8s-1.30-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.30-nvidia/defaults.d/)

## aws-k8s-1.30: Kubernetes 1.30

* [Model](src/aws-k8s-1.30/mod.rs)
* [Default settings](src/aws-k8s-1.30/defaults.d/)

## aws-k8s-1.30-nvidia: Kubernetes 1.30 NVIDIA

* [Model](src/aws-k8s-1.30-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.30-nvidia/defaults.d/)

## aws-ecs-1: Amazon ECS

* [Model](src/aws-ecs-1/mod.rs)
* [Default settings](src/aws-ecs-1/defaults.d/)

## aws-ecs-1-nvidia: Amazon ECS NVIDIA

* [Model](src/aws-ecs-1-nvidia/mod.rs)
* [Default settings](src/aws-ecs-1-nvidia/defaults.d/)

## aws-ecs-2: Amazon ECS

* [Model](src/aws-ecs-1/mod.rs)
* [Default settings](src/aws-ecs-1/defaults.d/)

## aws-ecs-2-nvidia: Amazon ECS NVIDIA

* [Model](src/aws-ecs-1-nvidia/mod.rs)
* [Default settings](src/aws-ecs-1-nvidia/defaults.d/)

## aws-dev: AWS development build

* [Model](src/aws-dev/mod.rs)
* [Default settings](src/aws-dev/defaults.d/)

## vmware-dev: VMware development build

* [Model](src/vmware-dev/mod.rs)
* [Default settings](src/vmware-dev/defaults.d/)

## vmware-k8s-1.26: VMware Kubernetes 1.26

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

## vmware-k8s-1.27: VMware Kubernetes 1.27

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

## vmware-k8s-1.28: VMware Kubernetes 1.28

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

## vmware-k8s-1.29: VMware Kubernetes 1.29

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

## vmware-k8s-1.30: VMware Kubernetes 1.30

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

## metal-dev: Metal development build

* [Model](src/metal-dev/mod.rs)
* [Default settings](src/metal-dev/defaults.d/)

## metal-k8s-1.26: Metal Kubernetes 1.26

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

## metal-k8s-1.27: Metal Kubernetes 1.27

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

## metal-k8s-1.28: Metal Kubernetes 1.28

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

## metal-k8s-1.29: Metal Kubernetes 1.29

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

*/

// Clippy has a false positive in the presence of the Scalar macro.
#![allow(clippy::derived_hash_with_manual_eq)]

// The "de" module contains custom deserialization trait implementation for models.
mod de;

pub use modeled_types;
use modeled_types::KubernetesCPUManagerPolicyOption;
use modeled_types::KubernetesEvictionKey;
use modeled_types::KubernetesMemoryManagerPolicy;
use modeled_types::KubernetesMemoryReservation;
use modeled_types::NonNegativeInteger;

#[path="aws-dev/mod.rs"]
mod aws_dev;
#[path="aws-ecs-1/mod.rs"]
mod aws_ecs_1;
#[path="aws-ecs-1-nvidia/mod.rs"]
mod aws_ecs_1_nvidia;
#[path="aws-ecs-2/mod.rs"]
mod aws_ecs_2;
#[path="aws-ecs-2-nvidia/mod.rs"]
mod aws_ecs_2_nvidia;
#[path="aws-k8s-1.23/mod.rs"]
mod aws_k8s_123;
#[path="aws-k8s-1.23-nvidia/mod.rs"]
mod aws_k8s_123_nvidia;
#[path="aws-k8s-1.24/mod.rs"]
mod aws_k8s_124;
#[path="aws-k8s-1.24-nvidia/mod.rs"]
mod aws_k8s_124_nvidia;
#[path="aws-k8s-1.25/mod.rs"]
mod aws_k8s_125;
#[path="aws-k8s-1.25-nvidia/mod.rs"]
mod aws_k8s_125_nvidia;
#[path="aws-k8s-1.26/mod.rs"]
mod aws_k8s_126;
#[path="aws-k8s-1.26-nvidia/mod.rs"]
mod aws_k8s_126_nvidia;
#[path="aws-k8s-1.27/mod.rs"]
mod aws_k8s_127;
#[path="aws-k8s-1.27-nvidia/mod.rs"]
mod aws_k8s_127_nvidia;
#[path="aws-k8s-1.28/mod.rs"]
mod aws_k8s_128;
#[path="aws-k8s-1.28-nvidia/mod.rs"]
mod aws_k8s_128_nvidia;
#[path="aws-k8s-1.29/mod.rs"]
mod aws_k8s_129;
#[path="aws-k8s-1.29-nvidia/mod.rs"]
mod aws_k8s_129_nvidia;
#[path="aws-k8s-1.30/mod.rs"]
mod aws_k8s_130;
#[path="aws-k8s-1.30-nvidia/mod.rs"]
mod aws_k8s_130_nvidia;
#[path="metal-dev/mod.rs"]
mod metal_dev;
#[path="metal-k8s-1.25/mod.rs"]
mod metal_k8s_125;
#[path="metal-k8s-1.26/mod.rs"]
mod metal_k8s_126;
#[path="metal-k8s-1.27/mod.rs"]
mod metal_k8s_127;
#[path="metal-k8s-1.28/mod.rs"]
mod metal_k8s_128;
#[path="metal-k8s-1.29/mod.rs"]
mod metal_k8s_129;
#[path="vmware-dev/mod.rs"]
mod vmware_dev;
#[path="vmware-k8s-1.25/mod.rs"]
mod vmware_k8s_125;
#[path="vmware-k8s-1.26/mod.rs"]
mod vmware_k8s_126;
#[path="vmware-k8s-1.27/mod.rs"]
mod vmware_k8s_127;
#[path="vmware-k8s-1.28/mod.rs"]
mod vmware_k8s_128;
#[path="vmware-k8s-1.29/mod.rs"]
mod vmware_k8s_129;
#[path="vmware-k8s-1.30/mod.rs"]
mod vmware_k8s_130;

// Types used to communicate between client and server for 'apiclient exec'.
pub mod exec;

// Below, we define common structures used in the API surface; specific variants build a Settings
// structure based on these, and that's what gets exposed via the API.  (Specific variants' models
// are in submodules)

use model_derive::model;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

use crate::de::{deserialize_limit, deserialize_mirrors, deserialize_node_taints};
use modeled_types::{
    BootConfigKey, BootConfigValue, BootstrapContainerMode, CpuManagerPolicy, CredentialProvider,
    DNSDomain, ECSAgentImagePullBehavior, ECSAgentLogLevel, ECSAttributeKey, ECSAttributeValue,
    ECSDurationValue, EtcHostsEntries, FriendlyVersion, Identifier, IntegerPercent, KmodKey,
    KubernetesAuthenticationMode, KubernetesBootstrapToken, KubernetesCloudProvider,
    KubernetesClusterDnsIp, KubernetesClusterName, KubernetesDurationValue, KubernetesLabelKey,
    KubernetesLabelValue, KubernetesQuantityValue, KubernetesReservedResourceKey,
    KubernetesTaintValue, KubernetesThresholdValue, Lockdown, OciDefaultsCapability,
    OciDefaultsResourceLimitType, PemCertificateString, SingleLineString, SysctlKey,
    TopologyManagerPolicy, TopologyManagerScope, Url, ValidBase64, ValidLinuxHostname,
};

use bottlerocket_release::BottlerocketRelease;
use variant_note::get_variant_from_elf_note;

// This is the top-level model exposed by the API system. It contains the common sections for all
// variants.  This allows a single API call to retrieve everything the API system knows, which is
// useful as a check and also, for example, as a data source for templated configuration files.
#[model]
struct Model {
    settings: Settings,
    services: Services,
    configuration_files: ConfigurationFiles,
    os: BottlerocketRelease,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Settings {
    AwsDev(aws_dev::Settings),
    AwsEcs1(aws_ecs_1::Settings),
    AwsEcs1Nvidia(aws_ecs_1_nvidia::Settings),
    AwsEcs2(aws_ecs_2::Settings),
    AwsEcs2Nvidia(aws_ecs_2_nvidia::Settings),
    AwsK8s123(aws_k8s_123::Settings),
    AwsK8s123Nvidia(aws_k8s_123_nvidia::Settings),
    AwsK8s124(aws_k8s_124::Settings),
    AwsK8s124Nvidia(aws_k8s_124_nvidia::Settings),
    AwsK8s125(aws_k8s_125::Settings),
    AwsK8s125Nvidia(aws_k8s_125_nvidia::Settings),
    AwsK8s126(aws_k8s_126::Settings),
    AwsK8s126Nvidia(aws_k8s_126_nvidia::Settings),
    AwsK8s127(aws_k8s_127::Settings),
    AwsK8s127Nvidia(aws_k8s_127_nvidia::Settings),
    AwsK8s128(aws_k8s_128::Settings),
    AwsK8s128Nvidia(aws_k8s_128_nvidia::Settings),
    AwsK8s129(aws_k8s_129::Settings),
    AwsK8s129Nvidia(aws_k8s_129_nvidia::Settings),
    AwsK8s130(aws_k8s_130::Settings),
    AwsK8s130Nvidia(aws_k8s_130_nvidia::Settings),
    MetalDev(metal_dev::Settings),
    MetalK8s125(metal_k8s_125::Settings),
    MetalK8s126(metal_k8s_126::Settings),
    MetalK8s127(metal_k8s_127::Settings),
    MetalK8s128(metal_k8s_128::Settings),
    MetalK8s129(metal_k8s_129::Settings),
    VmwareDev(vmware_dev::Settings),
    VmwareK8s125(vmware_k8s_125::Settings),
    VmwareK8s126(vmware_k8s_126::Settings),
    VmwareK8s127(vmware_k8s_127::Settings),
    VmwareK8s128(vmware_k8s_128::Settings),
    VmwareK8s129(vmware_k8s_129::Settings),
    VmwareK8s130(vmware_k8s_130::Settings),
}

impl Default for Settings {
    fn default() -> Self {
        let variant = get_variant_from_elf_note();
        match variant.as_str() {
            "aws-dev" => Settings::AwsDev(aws_dev::Settings::default()),
            "aws-ecs-1" => Settings::AwsEcs1(aws_ecs_1::Settings::default()),
            "aws-ecs-1-nvidia" => Settings::AwsEcs1Nvidia(aws_ecs_1_nvidia::Settings::default()),
            "aws-ecs-2" => Settings::AwsEcs2(aws_ecs_2::Settings::default()),
            "aws-ecs-2-nvidia" => Settings::AwsEcs2Nvidia(aws_ecs_2_nvidia::Settings::default()),
            "aws-k8s-1.23" => Settings::AwsK8s123(aws_k8s_123::Settings::default()),
            "aws-k8s-1.23-nvidia" => Settings::AwsK8s123Nvidia(aws_k8s_123_nvidia::Settings::default()),
            "aws-k8s-1.24" => Settings::AwsK8s124(aws_k8s_124::Settings::default()),
            "aws-k8s-1.24-nvidia" => Settings::AwsK8s124Nvidia(aws_k8s_124_nvidia::Settings::default()),
            "aws-k8s-1.25" => Settings::AwsK8s125(aws_k8s_125::Settings::default()),
            "aws-k8s-1.25-nvidia" => Settings::AwsK8s125Nvidia(aws_k8s_125_nvidia::Settings::default()),
            "aws-k8s-1.26" => Settings::AwsK8s126(aws_k8s_126::Settings::default()),
            "aws-k8s-1.26-nvidia" => Settings::AwsK8s126Nvidia(aws_k8s_126_nvidia::Settings::default()),
            "aws-k8s-1.27" => Settings::AwsK8s127(aws_k8s_127::Settings::default()),
            "aws-k8s-1.27-nvidia" => Settings::AwsK8s127Nvidia(aws_k8s_127_nvidia::Settings::default()),
            "aws-k8s-1.28" => Settings::AwsK8s128(aws_k8s_128::Settings::default()),
            "aws-k8s-1.28-nvidia" => Settings::AwsK8s128Nvidia(aws_k8s_128_nvidia::Settings::default()),
            "aws-k8s-1.29" => Settings::AwsK8s129(aws_k8s_129::Settings::default()),
            "aws-k8s-1.29-nvidia" => Settings::AwsK8s129Nvidia(aws_k8s_129_nvidia::Settings::default()),
            "aws-k8s-1.30" => Settings::AwsK8s130(aws_k8s_130::Settings::default()),
            "aws-k8s-1.30-nvidia" => Settings::AwsK8s130Nvidia(aws_k8s_130_nvidia::Settings::default()),
            "metal-dev" => Settings::MetalDev(metal_dev::Settings::default()),
            "metal-k8s-1.25" => Settings::MetalK8s125(metal_k8s_125::Settings::default()),
            "metal-k8s-1.26" => Settings::MetalK8s126(metal_k8s_126::Settings::default()),
            "metal-k8s-1.27" => Settings::MetalK8s127(metal_k8s_127::Settings::default()),
            "metal-k8s-1.28" => Settings::MetalK8s128(metal_k8s_128::Settings::default()),
            "metal-k8s-1.29" => Settings::MetalK8s129(metal_k8s_129::Settings::default()),
            "vmware-dev" => Settings::VmwareDev(vmware_dev::Settings::default()),
            "vmware-k8s-1.25" => Settings::VmwareK8s125(vmware_k8s_125::Settings::default()),
            "vmware-k8s-1.26" => Settings::VmwareK8s126(vmware_k8s_126::Settings::default()),
            "vmware-k8s-1.27" => Settings::VmwareK8s127(vmware_k8s_127::Settings::default()),
            "vmware-k8s-1.28" => Settings::VmwareK8s128(vmware_k8s_128::Settings::default()),
            "vmware-k8s-1.29" => Settings::VmwareK8s129(vmware_k8s_129::Settings::default()),
            "vmware-k8s-1.30" => Settings::VmwareK8s130(vmware_k8s_130::Settings::default()),
            _ => unimplemented!(),
        }
    }
}

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
         where D: Deserializer<'de>
    {
        let variant = get_variant_from_elf_note();
        match variant.as_str() {
            "aws-dev" => Ok(Settings::AwsDev(aws_dev::Settings::deserialize(deserializer)?)),
            "aws-ecs-1" => Ok(Settings::AwsEcs1(aws_ecs_1::Settings::deserialize(deserializer)?)),
            "aws-ecs-1-nvidia" => Ok(Settings::AwsEcs1Nvidia(aws_ecs_1_nvidia::Settings::deserialize(deserializer)?)),
            "aws-ecs-2" => Ok(Settings::AwsEcs2(aws_ecs_2::Settings::deserialize(deserializer)?)),
            "aws-ecs-2-nvidia" => Ok(Settings::AwsEcs2Nvidia(aws_ecs_2_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.23" => Ok(Settings::AwsK8s123(aws_k8s_123::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.23-nvidia" => Ok(Settings::AwsK8s123Nvidia(aws_k8s_123_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.24" => Ok(Settings::AwsK8s124(aws_k8s_124::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.24-nvidia" => Ok(Settings::AwsK8s124Nvidia(aws_k8s_124_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.25" => Ok(Settings::AwsK8s125(aws_k8s_125::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.25-nvidia" => Ok(Settings::AwsK8s125Nvidia(aws_k8s_125_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.26" => Ok(Settings::AwsK8s126(aws_k8s_126::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.26-nvidia" => Ok(Settings::AwsK8s126Nvidia(aws_k8s_126_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.27" => Ok(Settings::AwsK8s127(aws_k8s_127::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.27-nvidia" => Ok(Settings::AwsK8s127Nvidia(aws_k8s_127_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.28" => Ok(Settings::AwsK8s128(aws_k8s_128::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.28-nvidia" => Ok(Settings::AwsK8s128Nvidia(aws_k8s_128_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.29" => Ok(Settings::AwsK8s129(aws_k8s_129::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.29-nvidia" => Ok(Settings::AwsK8s129Nvidia(aws_k8s_129_nvidia::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.30" => Ok(Settings::AwsK8s130(aws_k8s_130::Settings::deserialize(deserializer)?)),
            "aws-k8s-1.30-nvidia" => Ok(Settings::AwsK8s130Nvidia(aws_k8s_130_nvidia::Settings::deserialize(deserializer)?)),
            "metal-dev" => Ok(Settings::MetalDev(metal_dev::Settings::deserialize(deserializer)?)),
            "metal-k8s-1.25" => Ok(Settings::MetalK8s125(metal_k8s_125::Settings::deserialize(deserializer)?)),
            "metal-k8s-1.26" => Ok(Settings::MetalK8s126(metal_k8s_126::Settings::deserialize(deserializer)?)),
            "metal-k8s-1.27" => Ok(Settings::MetalK8s127(metal_k8s_127::Settings::deserialize(deserializer)?)),
            "metal-k8s-1.28" => Ok(Settings::MetalK8s128(metal_k8s_128::Settings::deserialize(deserializer)?)),
            "metal-k8s-1.29" => Ok(Settings::MetalK8s129(metal_k8s_129::Settings::deserialize(deserializer)?)),
            "vmware-dev" => Ok(Settings::VmwareDev(vmware_dev::Settings::deserialize(deserializer)?)),
            "vmware-k8s-1.25" => Ok(Settings::VmwareK8s125(vmware_k8s_125::Settings::deserialize(deserializer)?)),
            "vmware-k8s-1.26" => Ok(Settings::VmwareK8s126(vmware_k8s_126::Settings::deserialize(deserializer)?)),
            "vmware-k8s-1.27" => Ok(Settings::VmwareK8s127(vmware_k8s_127::Settings::deserialize(deserializer)?)),
            "vmware-k8s-1.28" => Ok(Settings::VmwareK8s128(vmware_k8s_128::Settings::deserialize(deserializer)?)),
            "vmware-k8s-1.29" => Ok(Settings::VmwareK8s129(vmware_k8s_129::Settings::deserialize(deserializer)?)),
            "vmware-k8s-1.30" => Ok(Settings::VmwareK8s130(vmware_k8s_130::Settings::deserialize(deserializer)?)),
            _ => unimplemented!(),
        }
    }
}

impl Settings {
    pub fn aws(&self) -> Option<settings_extension_aws::AwsSettingsV1> {
        match self {
            Settings::AwsDev(settings) => settings.aws.clone(),
            Settings::AwsEcs1(settings) => settings.aws.clone(),
            Settings::AwsEcs1Nvidia(settings) => settings.aws.clone(),
            Settings::AwsEcs2(settings) => settings.aws.clone(),
            Settings::AwsEcs2Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s123(settings) => settings.aws.clone(),
            Settings::AwsK8s123Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s124(settings) => settings.aws.clone(),
            Settings::AwsK8s124Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s125(settings) => settings.aws.clone(),
            Settings::AwsK8s125Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s126(settings) => settings.aws.clone(),
            Settings::AwsK8s126Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s127(settings) => settings.aws.clone(),
            Settings::AwsK8s127Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s128(settings) => settings.aws.clone(),
            Settings::AwsK8s128Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s129(settings) => settings.aws.clone(),
            Settings::AwsK8s129Nvidia(settings) => settings.aws.clone(),
            Settings::AwsK8s130(settings) => settings.aws.clone(),
            Settings::AwsK8s130Nvidia(settings) => settings.aws.clone(),
            Settings::MetalK8s125(settings) => settings.aws.clone(),
            Settings::MetalK8s126(settings) => settings.aws.clone(),
            Settings::MetalK8s127(settings) => settings.aws.clone(),
            Settings::MetalK8s128(settings) => settings.aws.clone(),
            Settings::MetalK8s129(settings) => settings.aws.clone(),
            Settings::VmwareK8s125(settings) => settings.aws.clone(),
            Settings::VmwareK8s126(settings) => settings.aws.clone(),
            Settings::VmwareK8s127(settings) => settings.aws.clone(),
            Settings::VmwareK8s128(settings) => settings.aws.clone(),
            Settings::VmwareK8s129(settings) => settings.aws.clone(),
            _ => None,
        }
    }
    
    pub fn kubernetes(&self) -> Option<KubernetesSettings> {
        match self {
            Settings::AwsK8s123(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s123Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s124(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s124Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s125(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s125Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s126(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s126Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s127(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s127Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s128(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s128Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s129(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s129Nvidia(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s130(settings) => settings.kubernetes.clone(),
            Settings::AwsK8s130Nvidia(settings) => settings.kubernetes.clone(),
            Settings::MetalK8s125(settings) => settings.kubernetes.clone(),
            Settings::MetalK8s126(settings) => settings.kubernetes.clone(),
            Settings::MetalK8s127(settings) => settings.kubernetes.clone(),
            Settings::MetalK8s128(settings) => settings.kubernetes.clone(),
            Settings::MetalK8s129(settings) => settings.kubernetes.clone(),
            Settings::VmwareK8s125(settings) => settings.kubernetes.clone(),
            Settings::VmwareK8s126(settings) => settings.kubernetes.clone(),
            Settings::VmwareK8s127(settings) => settings.kubernetes.clone(),
            Settings::VmwareK8s128(settings) => settings.kubernetes.clone(),
            Settings::VmwareK8s129(settings) => settings.kubernetes.clone(),
            _ => None,
        }
    }
}

// Kubernetes static pod manifest settings
#[derive(Clone)]
#[model]
struct StaticPod {
    enabled: bool,
    manifest: ValidBase64,
}

// Kubernetes related settings. The dynamic settings are retrieved from
// IMDS via Sundog's child "Pluto".
#[derive(Clone)]
#[model]
struct KubernetesSettings {
    // Settings that must be specified via user data or through API requests.  Not all settings are
    // useful for all modes. For example, in standalone mode the user does not need to specify any
    // cluster information, and the bootstrap token is only needed for TLS authentication mode.
    cluster_name: KubernetesClusterName,
    cluster_certificate: ValidBase64,
    api_server: Url,
    node_labels: HashMap<KubernetesLabelKey, KubernetesLabelValue>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_node_taints"
    )]
    node_taints: HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>>,
    static_pods: HashMap<Identifier, StaticPod>,
    authentication_mode: KubernetesAuthenticationMode,
    bootstrap_token: KubernetesBootstrapToken,
    standalone_mode: bool,
    eviction_hard: HashMap<KubernetesEvictionKey, KubernetesThresholdValue>,
    eviction_soft: HashMap<KubernetesEvictionKey, KubernetesThresholdValue>,
    eviction_soft_grace_period: HashMap<KubernetesEvictionKey, KubernetesDurationValue>,
    eviction_max_pod_grace_period: NonNegativeInteger,
    kube_reserved: HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue>,
    system_reserved: HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue>,
    allowed_unsafe_sysctls: Vec<SingleLineString>,
    server_tls_bootstrap: bool,
    cloud_provider: KubernetesCloudProvider,
    registry_qps: i32,
    registry_burst: i32,
    event_qps: i32,
    event_burst: i32,
    kube_api_qps: i32,
    kube_api_burst: i32,
    container_log_max_size: KubernetesQuantityValue,
    container_log_max_files: i32,
    cpu_cfs_quota_enforced: bool,
    cpu_manager_policy: CpuManagerPolicy,
    cpu_manager_reconcile_period: KubernetesDurationValue,
    cpu_manager_policy_options: Vec<KubernetesCPUManagerPolicyOption>,
    topology_manager_scope: TopologyManagerScope,
    topology_manager_policy: TopologyManagerPolicy,
    pod_pids_limit: i64,
    image_gc_high_threshold_percent: IntegerPercent,
    image_gc_low_threshold_percent: IntegerPercent,
    provider_id: Url,
    log_level: u8,
    credential_providers: HashMap<Identifier, CredentialProvider>,
    server_certificate: ValidBase64,
    server_key: ValidBase64,
    shutdown_grace_period: KubernetesDurationValue,
    shutdown_grace_period_for_critical_pods: KubernetesDurationValue,
    memory_manager_reserved_memory: HashMap<Identifier, KubernetesMemoryReservation>,
    memory_manager_policy: KubernetesMemoryManagerPolicy,

    // Settings where we generate a value based on the runtime environment.  The user can specify a
    // value to override the generated one, but typically would not.
    max_pods: u32,
    cluster_dns_ip: KubernetesClusterDnsIp,
    cluster_domain: DNSDomain,
    node_ip: IpAddr,
    pod_infra_container_image: SingleLineString,
    // Generated in `aws-k8s-1.26*` variants only
    hostname_override: ValidLinuxHostname,
    // Generated in `k8s-1.25+` variants only
    seccomp_default: bool,
}

// ECS settings.
#[model]
struct ECSSettings {
    cluster: String,
    instance_attributes: HashMap<ECSAttributeKey, ECSAttributeValue>,
    allow_privileged_containers: bool,
    logging_drivers: Vec<SingleLineString>,
    loglevel: ECSAgentLogLevel,
    enable_spot_instance_draining: bool,
    image_pull_behavior: ECSAgentImagePullBehavior,
    container_stop_timeout: ECSDurationValue,
    task_cleanup_wait: ECSDurationValue,
    metadata_service_rps: i64,
    metadata_service_burst: i64,
    reserved_memory: u16,
    image_cleanup_wait: ECSDurationValue,
    image_cleanup_delete_per_cycle: i64,
    image_cleanup_enabled: bool,
    image_cleanup_age: ECSDurationValue,
    backend_host: String,
    awsvpc_block_imds: bool,
    enable_container_metadata: bool,
}

#[model]
struct RegistryMirror {
    registry: SingleLineString,
    endpoint: Vec<Url>,
}

#[model]
struct RegistryCredential {
    registry: SingleLineString,
    username: SingleLineString,
    password: SingleLineString,
    // This is the base64 encoding of "username:password"
    auth: ValidBase64,
    identitytoken: SingleLineString,
}

// Image registry settings for the container runtimes.
#[model]
struct RegistrySettings {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_mirrors"
    )]
    mirrors: Vec<RegistryMirror>,
    #[serde(alias = "creds", default, skip_serializing_if = "Option::is_none")]
    credentials: Vec<RegistryCredential>,
}

// Update settings. Taken from userdata. The 'seed' setting is generated
// by the "Bork" settings generator at runtime.
#[model]
struct UpdatesSettings {
    metadata_base_url: Url,
    targets_base_url: Url,
    seed: u32,
    // Version to update to when updating via the API.
    version_lock: FriendlyVersion,
    ignore_waves: bool,
}

#[model]
struct HostContainer {
    source: Url,
    enabled: bool,
    superpowered: bool,
    user_data: ValidBase64,
}

// Network settings. These settings will affect host service components' network behavior
#[model]
struct NetworkSettings {
    hostname: ValidLinuxHostname,
    hosts: EtcHostsEntries,
    https_proxy: Url,
    // We allow some flexibility in NO_PROXY values because different services support different formats.
    no_proxy: Vec<SingleLineString>,
}

// NTP settings
#[model]
struct NtpSettings {
    time_servers: Vec<Url>,
}

// DNS Settings
#[model]
struct DnsSettings {
    name_servers: Vec<IpAddr>,
    search_list: Vec<ValidLinuxHostname>,
}

// Kernel settings
#[model]
struct KernelSettings {
    lockdown: Lockdown,
    modules: HashMap<KmodKey, KmodSetting>,
    // Values are almost always a single line and often just an integer... but not always.
    sysctl: HashMap<SysctlKey, String>,
}

// Kernel module settings
#[model]
struct KmodSetting {
    allowed: bool,
    autoload: bool,
}

// Kernel boot settings
#[model]
struct BootSettings {
    reboot_to_reconcile: bool,
    #[serde(
        alias = "kernel",
        rename(serialize = "kernel"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    kernel_parameters: HashMap<BootConfigKey, Vec<BootConfigValue>>,
    #[serde(
        alias = "init",
        rename(serialize = "init"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    init_parameters: HashMap<BootConfigKey, Vec<BootConfigValue>>,
}

// Platform-specific settings
#[model]
struct AwsSettings {
    region: SingleLineString,
    config: ValidBase64,
    credentials: ValidBase64,
    profile: SingleLineString,
}

// Metrics settings
#[model]
struct MetricsSettings {
    metrics_url: Url,
    send_metrics: bool,
    service_checks: Vec<String>,
}

// CloudFormation settings
#[model]
struct CloudFormationSettings {
    should_signal: bool,
    stack_name: SingleLineString,
    logical_resource_id: SingleLineString,
}

// AutoScaling settings
#[model]
struct AutoScalingSettings {
    should_wait: bool,
}

// Container runtime settings
#[model]
struct ContainerRuntimeSettings {
    max_container_log_line_size: i32,
    max_concurrent_downloads: i32,
    enable_unprivileged_ports: bool,
    enable_unprivileged_icmp: bool,
}

///// Internal services

// Note: Top-level objects that get returned from the API should have a "rename" attribute
// matching the struct name, but in kebab-case, e.g. ConfigurationFiles -> "configuration-files".
// This lets it match the datastore name.
// Objects that live inside those top-level objects, e.g. Service lives in Services, should have
// rename="" so they don't add an extra prefix to the datastore path that doesn't actually exist.
// This is important because we have APIs that can return those sub-structures directly.

pub type Services = HashMap<String, Service>;

#[model(add_option = false, rename = "")]
struct Service {
    configuration_files: Vec<SingleLineString>,
    restart_commands: Vec<String>,
}

pub type ConfigurationFiles = HashMap<String, ConfigurationFile>;

#[model(add_option = false, rename = "")]
struct ConfigurationFile {
    path: SingleLineString,
    template_path: SingleLineString,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<String>,
}

///// Metadata

#[model(add_option = false, rename = "metadata")]
struct Metadata {
    key: SingleLineString,
    md: SingleLineString,
    val: toml::Value,
}

///// Bootstrap Containers

#[model]
struct BootstrapContainer {
    source: Url,
    mode: BootstrapContainerMode,
    user_data: ValidBase64,
    essential: bool,
}

///// PEM Certificates
#[model]
struct PemCertificate {
    data: PemCertificateString,
    trusted: bool,
}

///// OCI hooks
#[model]
struct OciHooks {
    log4j_hotpatch_enabled: bool,
}

///// OCI defaults specifies the default values that will be used in cri-base-json.
#[model]
struct OciDefaults {
    capabilities: HashMap<OciDefaultsCapability, bool>,
    resource_limits: HashMap<OciDefaultsResourceLimitType, OciDefaultsResourceLimit>,
}

///// The hard and soft limit values for an OCI defaults resource limit.
#[model(add_option = false)]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialOrd, PartialEq)]
struct OciDefaultsResourceLimit {
    #[serde(deserialize_with = "deserialize_limit")]
    hard_limit: i64,
    #[serde(deserialize_with = "deserialize_limit")]
    soft_limit: i64,
}

#[model(add_option = false)]
struct Report {
    name: String,
    description: String,
}
