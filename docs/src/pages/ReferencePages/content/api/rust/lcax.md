---
title: lcax API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.4.3

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

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"default\"), span: modules/lcax/src/rust.rs:5:7: 5:26 (#0) }])]")`

```rust
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> Result<lcax_convert::lcabyg::parse::LCABygResult, String> { /* ... */ }
```

#### Function `convert_ilcd`

**Attributes:**

- `Other("#[attr = CfgTrace([NameValue { name: \"feature\", value: Some(\"default\"), span: modules/lcax/src/rust.rs:13:7: 13:26 (#0) }])]")`

```rust
pub fn convert_ilcd(data: String) -> Result<lcax_models::epd::EPD, String> { /* ... */ }
```
