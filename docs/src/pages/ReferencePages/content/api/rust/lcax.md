---
title: lcax API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.7.0

**Format Version:** 57

# Module `lcax`

## Modules

## Module `rust`

```rust
pub mod rust { /* ... */ }
```

### Functions

#### Function `convert_lcabyg`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"default\"), span: modules/lcax/src/rust.rs:6:7: 6:26 (#0) }])]")`

```rust
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> Result<lcax_convert::lcabyg::parse::LCABygResult, String> { /* ... */ }
```

#### Function `convert_ilcd`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"default\"), span: modules/lcax/src/rust.rs:14:7: 14:26 (#0) }])]")`

```rust
pub fn convert_ilcd(data: String) -> Result<lcax_models::epd::EPD, String> { /* ... */ }
```

#### Function `get_energy_assemblies`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"default\"), span: modules/lcax/src/rust.rs:26:7: 26:26 (#0) }])]")`

Get default energy assemblies for a given standard.

Currently supported standards:

- `"BR18"`: Danish Building Regulations BR18 generic milestone data for Grid Electricity, District Heating, and Natural Gas.

```rust
pub fn get_energy_assemblies(standard: &str) -> Result<Vec<lcax_models::assembly::Assembly>, String> { /* ... */ }
```

## Re-exports

### Re-export `rust::*`

```rust
pub use rust::*;
```
