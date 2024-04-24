# models

Current version: 0.1.0

## API models

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

### aws-k8s-1.23: Kubernetes 1.23

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

### aws-k8s-1.23-nvidia: Kubernetes 1.23 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

### aws-k8s-1.24: Kubernetes 1.24

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

### aws-k8s-1.24-nvidia: Kubernetes 1.24 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

### aws-k8s-1.25: Kubernetes 1.25

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

### aws-k8s-1.25-nvidia: Kubernetes 1.25 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

### aws-k8s-1.26: Kubernetes 1.26

* [Model](src/aws-k8s-1.26/mod.rs)
* [Default settings](src/aws-k8s-1.26/defaults.d/)

### aws-k8s-1.26-nvidia: Kubernetes 1.26 NVIDIA

* [Model](src/aws-k8s-1.26-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.26-nvidia/defaults.d/)

### aws-k8s-1.27: Kubernetes 1.27

* [Model](src/aws-k8s-1.28/mod.rs)
* [Default settings](src/aws-k8s-1.28/defaults.d/)

### aws-k8s-1.27-nvidia: Kubernetes 1.27 NVIDIA

* [Model](src/aws-k8s-1.28-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.28-nvidia/defaults.d/)

### aws-k8s-1.28: Kubernetes 1.28

* [Model](src/aws-k8s-1.30/mod.rs)
* [Default settings](src/aws-k8s-1.30/defaults.d/)

### aws-k8s-1.28-nvidia: Kubernetes 1.28 NVIDIA

* [Model](src/aws-k8s-1.30-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.30-nvidia/defaults.d/)

### aws-k8s-1.29: Kubernetes 1.29

* [Model](src/aws-k8s-1.30/mod.rs)
* [Default settings](src/aws-k8s-1.30/defaults.d/)

### aws-k8s-1.29-nvidia: Kubernetes 1.29 NVIDIA

* [Model](src/aws-k8s-1.30-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.30-nvidia/defaults.d/)

### aws-k8s-1.30: Kubernetes 1.30

* [Model](src/aws-k8s-1.30/mod.rs)
* [Default settings](src/aws-k8s-1.30/defaults.d/)

### aws-k8s-1.30-nvidia: Kubernetes 1.30 NVIDIA

* [Model](src/aws-k8s-1.30-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.30-nvidia/defaults.d/)

### aws-ecs-1: Amazon ECS

* [Model](src/aws-ecs-1/mod.rs)
* [Default settings](src/aws-ecs-1/defaults.d/)

### aws-ecs-1-nvidia: Amazon ECS NVIDIA

* [Model](src/aws-ecs-1-nvidia/mod.rs)
* [Default settings](src/aws-ecs-1-nvidia/defaults.d/)

### aws-ecs-2: Amazon ECS

* [Model](src/aws-ecs-1/mod.rs)
* [Default settings](src/aws-ecs-1/defaults.d/)

### aws-ecs-2-nvidia: Amazon ECS NVIDIA

* [Model](src/aws-ecs-1-nvidia/mod.rs)
* [Default settings](src/aws-ecs-1-nvidia/defaults.d/)

### aws-dev: AWS development build

* [Model](src/aws-dev/mod.rs)
* [Default settings](src/aws-dev/defaults.d/)

### vmware-dev: VMware development build

* [Model](src/vmware-dev/mod.rs)
* [Default settings](src/vmware-dev/defaults.d/)

### vmware-k8s-1.26: VMware Kubernetes 1.26

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

### vmware-k8s-1.27: VMware Kubernetes 1.27

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

### vmware-k8s-1.28: VMware Kubernetes 1.28

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

### vmware-k8s-1.29: VMware Kubernetes 1.29

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

### vmware-k8s-1.30: VMware Kubernetes 1.30

* [Model](src/vmware-k8s-1.30/mod.rs)
* [Default settings](src/vmware-k8s-1.30/defaults.d/)

### metal-dev: Metal development build

* [Model](src/metal-dev/mod.rs)
* [Default settings](src/metal-dev/defaults.d/)

### metal-k8s-1.26: Metal Kubernetes 1.26

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

### metal-k8s-1.27: Metal Kubernetes 1.27

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

### metal-k8s-1.28: Metal Kubernetes 1.28

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)

### metal-k8s-1.29: Metal Kubernetes 1.29

* [Model](src/metal-k8s-1.29/mod.rs)
* [Default settings](src/metal-k8s-1.29/defaults.d/)


## Colophon

This text was generated from `README.tpl` using [cargo-readme](https://crates.io/crates/cargo-readme), and includes the rustdoc from `src/lib.rs`.
