---
title: lcax_calculation API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.4.3

**Format Version:** 57

# Module `lcax_calculation`

## Modules

## Module `calculate`

```rust
pub mod calculate { /* ... */ }
```

### Functions

#### Function `calculate_project`

```rust
pub fn calculate_project(project: &mut lcax_models::project::Project, options: Option<crate::models::CalculationOptions>) -> Result<&mut lcax_models::project::Project, String> { /* ... */ }
```

#### Function `calculate_assembly`

```rust
pub fn calculate_assembly(assembly: &mut lcax_models::assembly::Assembly, options: &crate::models::CalculationOptions) -> Result<lcax_models::life_cycle_base::Impacts, String> { /* ... */ }
```

#### Function `calculate_product`

```rust
pub fn calculate_product(product: &mut lcax_models::product::Product, options: &crate::models::CalculationOptions) -> Result<lcax_models::life_cycle_base::Impacts, String> { /* ... */ }
```

## Module `models`

```rust
pub mod models { /* ... */ }
```

### Types

#### Struct `CalculationOptions`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct CalculationOptions {
    pub reference_study_period: Option<u8>,
    pub life_cycle_modules: Vec<lcax_models::life_cycle_base::LifeCycleModule>,
    pub impact_categories: Vec<lcax_models::life_cycle_base::ImpactCategoryKey>,
    pub overwrite_existing_results: bool,
}
```

##### Fields

| Name                         | Type                                                   | Documentation |
| ---------------------------- | ------------------------------------------------------ | ------------- |
| `reference_study_period`     | `Option<u8>`                                           |               |
| `life_cycle_modules`         | `Vec<lcax_models::life_cycle_base::LifeCycleModule>`   |               |
| `impact_categories`          | `Vec<lcax_models::life_cycle_base::ImpactCategoryKey>` |               |
| `overwrite_existing_results` | `bool`                                                 |               |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CalculationOptions { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **JsonSchema**
  - ```rust
    fn schema_name() -> std::string::String { /* ... */ }
    ```

  - ```rust
    fn schema_id() -> std::borrow::Cow<''static, str> { /* ... */ }
    ```

  - ```rust
    fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CalculationOptions) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **StructuralPartialEq**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**

## Module `results`

```rust
pub mod results { /* ... */ }
```

### Functions

#### Function `get_impact_total`

```rust
pub fn get_impact_total(impacts: &lcax_models::life_cycle_base::Impacts, category: &lcax_models::life_cycle_base::ImpactCategoryKey, exclude_modules: &Option<Vec<lcax_models::life_cycle_base::LifeCycleModule>>) -> f64 { /* ... */ }
```

#### Function `normalize_result`

```rust
pub fn normalize_result(result: &f64, normalizing_factor: &f64) -> f64 { /* ... */ }
```

#### Function `get_impacts_by_life_cycle_module`

```rust
pub fn get_impacts_by_life_cycle_module(impacts: &lcax_models::life_cycle_base::Impacts, category: &lcax_models::life_cycle_base::ImpactCategoryKey, exclude_modules: &Option<Vec<lcax_models::life_cycle_base::LifeCycleModule>>, normalizing_factor: Option<f64>) -> Option<lcax_models::life_cycle_base::ImpactCategory> { /* ... */ }
```
