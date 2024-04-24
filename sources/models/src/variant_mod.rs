// This is linked into place at variant/mod.rs because the build system mounts a temporary
// directory at variant/ - see README.md.

use crate::{ConfigurationFiles, Services, Settings};
use bottlerocket_release::BottlerocketRelease;
use model_derive::model;

// We expose anything defined by the current variant.
mod current;

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
