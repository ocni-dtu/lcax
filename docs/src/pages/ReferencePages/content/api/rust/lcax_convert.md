---
title: lcax_convert API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.4.3

**Format Version:** 57

# Module `lcax_convert`

## Modules

## Module `br_standard`

```rust
pub mod br_standard { /* ... */ }
```

### Modules

## Module `br18_generic_data`

```rust
pub mod br18_generic_data { /* ... */ }
```

### Functions

#### Function `get_electricity_data`

```rust
pub fn get_electricity_data() -> std::collections::HashMap<u16, lcax_models::generic_impact_data::GenericData> { /* ... */ }
```

#### Function `get_district_heating_data`

```rust
pub fn get_district_heating_data() -> std::collections::HashMap<u16, lcax_models::generic_impact_data::GenericData> { /* ... */ }
```

#### Function `get_lng_data`

```rust
pub fn get_lng_data() -> std::collections::HashMap<u16, lcax_models::generic_impact_data::GenericData> { /* ... */ }
```

#### Function `get_energy_data`

```rust
pub fn get_energy_data(year: &u16, data: std::collections::HashMap<u16, lcax_models::generic_impact_data::GenericData>) -> Vec<lcax_models::generic_impact_data::GenericData> { /* ... */ }
```

## Module `parse`

```rust
pub mod parse { /* ... */ }
```

### Functions

#### Function `parse_br_standard`

Parse data from BR Standard Format in to a LCAx Project

# Arguments

- `project_name`: Project Name
- `project_info`: Project info from the "General information" tab
- `components`: Component info from the "BR18 vejledning" tab
- `operations`: Operation info from the "Drift" tab

returns: Result<LCAxProject, String>

```rust
pub fn parse_br_standard(project_name: &str, project_info: &crate::br_standard::models::BRProjectInfo, components: &Vec<crate::br_standard::models::BRComponent>, operations: &Vec<crate::br_standard::models::BROperation>) -> Result<lcax_models::project::Project, String> { /* ... */ }
```

## Module `xlsx`

```rust
pub mod xlsx { /* ... */ }
```

### Functions

#### Function `br_standard_from_file`

```rust
pub fn br_standard_from_file(file_path: std::path::PathBuf) -> Result<lcax_models::project::Project, String> { /* ... */ }
```

#### Function `read_br_standard`

```rust
pub fn read_br_standard<T>(workbook: &mut calamine::Xlsx<T>) -> Result<(crate::br_standard::models::BRProjectInfo, Vec<crate::br_standard::models::BRComponent>, Vec<crate::br_standard::models::BROperation>), calamine::Error>
where
    T: Seek + std::io::Read { /* ... */ }
```

#### Function `read_br_standard_from_file`

```rust
pub fn read_br_standard_from_file(file_path: &std::path::PathBuf) -> Result<(crate::br_standard::models::BRProjectInfo, Vec<crate::br_standard::models::BRComponent>, Vec<crate::br_standard::models::BROperation>), calamine::Error> { /* ... */ }
```

#### Function `read_br_standard_from_bytes`

```rust
pub fn read_br_standard_from_bytes(file: Vec<u8>) -> Result<(crate::br_standard::models::BRProjectInfo, Vec<crate::br_standard::models::BRComponent>, Vec<crate::br_standard::models::BROperation>), calamine::Error> { /* ... */ }
```

## Module `ilcd`

```rust
pub mod ilcd { /* ... */ }
```

### Modules

## Module `ilcd`

```rust
pub mod ilcd { /* ... */ }
```

### Types

#### Struct `ILCD`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ILCD {
    pub process_information: ProcessInformation,
    pub modelling_and_validation: ModellingAndValidation,
    pub exchanges: Exchanges,
    pub lcia_results: LCIAResults,
    pub version: String,
}
```

##### Fields

| Name                       | Type                     | Documentation |
| -------------------------- | ------------------------ | ------------- |
| `process_information`      | `ProcessInformation`     |               |
| `modelling_and_validation` | `ModellingAndValidation` |               |
| `exchanges`                | `Exchanges`              |               |
| `lcia_results`             | `LCIAResults`            |               |
| `version`                  | `String`                 |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Exchanges`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Exchanges {
    pub exchange: Vec<Exchange>,
}
```

##### Fields

| Name       | Type            | Documentation |
| ---------- | --------------- | ------------- |
| `exchange` | `Vec<Exchange>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Exchange`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Exchange {
    pub reference_to_flow_data_set: ReferenceToDescription,
    pub mean_amount: Option<f64>,
    pub resulting_amount: Option<f64>,
    pub data_set_internal_id: Option<u32>,
    pub reference_flow: Option<bool>,
    pub resulting_flow_amount: Option<f64>,
    pub flow_properties: Option<Vec<FlowProperty>>,
    pub material_properties: Option<Vec<MaterialProperty>>,
    pub exchange_direction: Option<String>,
    pub other: Option<LCIAAnies>,
}
```

##### Fields

| Name                         | Type                            | Documentation |
| ---------------------------- | ------------------------------- | ------------- |
| `reference_to_flow_data_set` | `ReferenceToDescription`        |               |
| `mean_amount`                | `Option<f64>`                   |               |
| `resulting_amount`           | `Option<f64>`                   |               |
| `data_set_internal_id`       | `Option<u32>`                   |               |
| `reference_flow`             | `Option<bool>`                  |               |
| `resulting_flow_amount`      | `Option<f64>`                   |               |
| `flow_properties`            | `Option<Vec<FlowProperty>>`     |               |
| `material_properties`        | `Option<Vec<MaterialProperty>>` |               |
| `exchange_direction`         | `Option<String>`                |               |
| `other`                      | `Option<LCIAAnies>`             |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `FlowProperty`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct FlowProperty {
    pub name: Vec<ValueLang>,
    pub uuid: String,
    pub mean_value: f64,
    pub reference_flow_property: Option<bool>,
    pub reference_unit: Option<String>,
    pub unit_group_uuid: Option<String>,
}
```

##### Fields

| Name                      | Type             | Documentation |
| ------------------------- | ---------------- | ------------- |
| `name`                    | `Vec<ValueLang>` |               |
| `uuid`                    | `String`         |               |
| `mean_value`              | `f64`            |               |
| `reference_flow_property` | `Option<bool>`   |               |
| `reference_unit`          | `Option<String>` |               |
| `unit_group_uuid`         | `Option<String>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `MaterialProperty`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct MaterialProperty {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub unit_description: Option<String>,
}
```

##### Fields

| Name               | Type             | Documentation |
| ------------------ | ---------------- | ------------- |
| `name`             | `String`         |               |
| `value`            | `String`         |               |
| `unit`             | `String`         |               |
| `unit_description` | `Option<String>` |               |

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
    fn clone(self: &Self) -> MaterialProperty { /* ... */ }
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

  - ```rust
    fn into(self: Self) -> HashMap<String, Option<AnyValue>> { /* ... */ }
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

#### Struct `ModellingAndValidation`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ModellingAndValidation {
    pub lci_method_and_allocation: LCIMethodAndAllocation,
    pub compliance_declarations: ComplianceDeclarations,
}
```

##### Fields

| Name                        | Type                     | Documentation |
| --------------------------- | ------------------------ | ------------- |
| `lci_method_and_allocation` | `LCIMethodAndAllocation` |               |
| `compliance_declarations`   | `ComplianceDeclarations` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `ComplianceDeclarations`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ComplianceDeclarations {
    pub compliance: Vec<Compliance>,
}
```

##### Fields

| Name         | Type              | Documentation |
| ------------ | ----------------- | ------------- |
| `compliance` | `Vec<Compliance>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Compliance`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Compliance {
    pub reference_to_compliance_system: ReferenceToDescription,
}
```

##### Fields

| Name                             | Type                     | Documentation |
| -------------------------------- | ------------------------ | ------------- |
| `reference_to_compliance_system` | `ReferenceToDescription` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `ReferenceToDescription`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ReferenceToDescription {
    pub short_description: Vec<ValueLang>,
    pub _type: String,
    pub version: Option<String>,
}
```

##### Fields

| Name                | Type             | Documentation |
| ------------------- | ---------------- | ------------- |
| `short_description` | `Vec<ValueLang>` |               |
| `_type`             | `String`         |               |
| `version`           | `Option<String>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `LCIAResults`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct LCIAResults {
    pub lcia_result: Vec<LCIAResult>,
}
```

##### Fields

| Name          | Type              | Documentation |
| ------------- | ----------------- | ------------- |
| `lcia_result` | `Vec<LCIAResult>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `LCIAResult`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct LCIAResult {
    pub reference_to_lcia_method_dataset: ReferenceToLCIAMethodDataSet,
    pub other: LCIAAnies,
}
```

##### Fields

| Name                               | Type                           | Documentation |
| ---------------------------------- | ------------------------------ | ------------- |
| `reference_to_lcia_method_dataset` | `ReferenceToLCIAMethodDataSet` |               |
| `other`                            | `LCIAAnies`                    |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `LCIAAnies`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct LCIAAnies {
    pub anies: Vec<ModuleAnie>,
}
```

##### Fields

| Name    | Type              | Documentation |
| ------- | ----------------- | ------------- |
| `anies` | `Vec<ModuleAnie>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `ModuleAnie`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ModuleAnie {
    pub module: Option<String>,
    pub value: Option<AnieValue>,
}
```

##### Fields

| Name     | Type                | Documentation |
| -------- | ------------------- | ------------- |
| `module` | `Option<String>`    |               |
| `value`  | `Option<AnieValue>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Enum `AnieValue`

**Attributes:**

- `Other("#[serde(untagged)]")`

```rust
pub enum AnieValue {
    ValueString(String),
    ValueObject(ValueObject),
}
```

##### Variants

###### `ValueString`

Fields:

| Index | Type     | Documentation |
| ----- | -------- | ------------- |
| 0     | `String` |               |

###### `ValueObject`

Fields:

| Index | Type          | Documentation |
| ----- | ------------- | ------------- |
| 0     | `ValueObject` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &AnieValue) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**

#### Struct `ValueObject`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ValueObject {
    // Some fields omitted
}
```

##### Fields

| Name             | Type | Documentation                   |
| ---------------- | ---- | ------------------------------- |
| _private fields_ | ...  | _Some fields have been omitted_ |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Enum `ModuleValue`

```rust
pub enum ModuleValue {
    Value(String),
    Name(ModuleMap),
}
```

##### Variants

###### `Value`

Fields:

| Index | Type     | Documentation |
| ----- | -------- | ------------- |
| 0     | `String` |               |

###### `Name`

Fields:

| Index | Type        | Documentation |
| ----- | ----------- | ------------- |
| 0     | `ModuleMap` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `ModuleMap`

```rust
pub struct ModuleMap {
    // Some fields omitted
}
```

##### Fields

| Name             | Type | Documentation                   |
| ---------------- | ---- | ------------------------------- |
| _private fields_ | ...  | _Some fields have been omitted_ |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `ReferenceToLCIAMethodDataSet`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ReferenceToLCIAMethodDataSet {
    pub short_description: Vec<ValueLang>,
}
```

##### Fields

| Name                | Type             | Documentation |
| ------------------- | ---------------- | ------------- |
| `short_description` | `Vec<ValueLang>` |               |

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `LCIMethodAndAllocation`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct LCIMethodAndAllocation {
    pub other: Option<Anies>,
}
```

##### Fields

| Name    | Type            | Documentation |
| ------- | --------------- | ------------- |
| `other` | `Option<Anies>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Anies`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Anies {
    pub anies: Vec<Anie>,
}
```

##### Fields

| Name    | Type        | Documentation |
| ------- | ----------- | ------------- |
| `anies` | `Vec<Anie>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Anie`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Anie {
    pub name: String,
    pub value: Option<String>,
}
```

##### Fields

| Name    | Type             | Documentation |
| ------- | ---------------- | ------------- |
| `name`  | `String`         |               |
| `value` | `Option<String>` |               |

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
    fn clone(self: &Self) -> Anie { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
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

#### Struct `ProcessInformation`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct ProcessInformation {
    pub data_set_information: DataSetInformation,
    pub time: TimeData,
    pub geography: Geography,
    pub technology: Option<Technology>,
}
```

##### Fields

| Name                   | Type                 | Documentation |
| ---------------------- | -------------------- | ------------- |
| `data_set_information` | `DataSetInformation` |               |
| `time`                 | `TimeData`           |               |
| `geography`            | `Geography`          |               |
| `technology`           | `Option<Technology>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Technology`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Technology {
    pub description: Vec<ValueLang>,
}
```

##### Fields

| Name          | Type             | Documentation |
| ------------- | ---------------- | ------------- |
| `description` | `Vec<ValueLang>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `Geography`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct Geography {
    pub location_of_operation_supply_or_production: LocationOfOperationSupplyOrProduction,
}
```

##### Fields

| Name                                         | Type                                    | Documentation |
| -------------------------------------------- | --------------------------------------- | ------------- |
| `location_of_operation_supply_or_production` | `LocationOfOperationSupplyOrProduction` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `LocationOfOperationSupplyOrProduction`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct LocationOfOperationSupplyOrProduction {
    pub location: String,
}
```

##### Fields

| Name       | Type     | Documentation |
| ---------- | -------- | ------------- |
| `location` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `TimeData`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct TimeData {
    pub reference_year: i32,
    pub data_set_valid_until: i32,
}
```

##### Fields

| Name                   | Type  | Documentation |
| ---------------------- | ----- | ------------- |
| `reference_year`       | `i32` |               |
| `data_set_valid_until` | `i32` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `DataSetInformation`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct DataSetInformation {
    pub uuid: String,
    pub name: DataSetName,
    pub general_comment: Option<Vec<ValueLang>>,
}
```

##### Fields

| Name              | Type                     | Documentation |
| ----------------- | ------------------------ | ------------- |
| `uuid`            | `String`                 |               |
| `name`            | `DataSetName`            |               |
| `general_comment` | `Option<Vec<ValueLang>>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `DataSetName`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`

```rust
pub struct DataSetName {
    pub base_name: Vec<ValueLang>,
}
```

##### Fields

| Name        | Type             | Documentation |
| ----------- | ---------------- | ------------- |
| `base_name` | `Vec<ValueLang>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

#### Struct `ValueLang`

```rust
pub struct ValueLang {
    pub value: Option<String>,
    pub lang: String,
}
```

##### Fields

| Name    | Type             | Documentation |
| ------- | ---------------- | ------------- |
| `value` | `Option<String>` |               |
| `lang`  | `String`         |               |

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Sync**
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

## Module `parse`

```rust
pub mod parse { /* ... */ }
```

### Functions

#### Function `parse_ilcd`

Parse a ILCD formatted EPD in an EPDx struct

# Arguments

- `json`: JSON formatted string containing the "full" EPD on ILCD format

returns: EPD

```rust
pub fn parse_ilcd(data: &str) -> Result<lcax_models::epd::EPD, serde_json::Error> { /* ... */ }
```

## Module `lcabyg`

```rust
pub mod lcabyg { /* ... */ }
```

### Modules

## Module `nodes`

```rust
pub mod nodes { /* ... */ }
```

### Types

#### Enum `Node`

**Attributes:**

- `Other("#[serde(rename_all = \"PascalCase\")]")`

```rust
pub enum Node {
    ConstructionProcess(ConstructionProcess),
    Product(Product),
    Construction(Construction),
    ElementModel(ElementModel),
    ConstructionInstallation(ConstructionInstallation),
    FuelConsumption(FuelConsumption),
    DGNBOperationReference(DGNBOperationReference),
    Element(Element),
    EmbodiedRoot(EmbodiedRoot),
    Building(Building),
    Stage(Stage),
    Operation(Operation),
    ProductTransportRoot(ProductTransportRoot),
    Project(Project),
    Transport(Transport),
}
```

##### Variants

###### `ConstructionProcess`

Fields:

| Index | Type                  | Documentation |
| ----- | --------------------- | ------------- |
| 0     | `ConstructionProcess` |               |

###### `Product`

Fields:

| Index | Type      | Documentation |
| ----- | --------- | ------------- |
| 0     | `Product` |               |

###### `Construction`

Fields:

| Index | Type           | Documentation |
| ----- | -------------- | ------------- |
| 0     | `Construction` |               |

###### `ElementModel`

Fields:

| Index | Type           | Documentation |
| ----- | -------------- | ------------- |
| 0     | `ElementModel` |               |

###### `ConstructionInstallation`

Fields:

| Index | Type                       | Documentation |
| ----- | -------------------------- | ------------- |
| 0     | `ConstructionInstallation` |               |

###### `FuelConsumption`

Fields:

| Index | Type              | Documentation |
| ----- | ----------------- | ------------- |
| 0     | `FuelConsumption` |               |

###### `DGNBOperationReference`

Fields:

| Index | Type                     | Documentation |
| ----- | ------------------------ | ------------- |
| 0     | `DGNBOperationReference` |               |

###### `Element`

Fields:

| Index | Type      | Documentation |
| ----- | --------- | ------------- |
| 0     | `Element` |               |

###### `EmbodiedRoot`

Fields:

| Index | Type           | Documentation |
| ----- | -------------- | ------------- |
| 0     | `EmbodiedRoot` |               |

###### `Building`

Fields:

| Index | Type       | Documentation |
| ----- | ---------- | ------------- |
| 0     | `Building` |               |

###### `Stage`

Fields:

| Index | Type    | Documentation |
| ----- | ------- | ------------- |
| 0     | `Stage` |               |

###### `Operation`

Fields:

| Index | Type        | Documentation |
| ----- | ----------- | ------------- |
| 0     | `Operation` |               |

###### `ProductTransportRoot`

Fields:

| Index | Type                   | Documentation |
| ----- | ---------------------- | ------------- |
| 0     | `ProductTransportRoot` |               |

###### `Project`

Fields:

| Index | Type      | Documentation |
| ----- | --------- | ------------- |
| 0     | `Project` |               |

###### `Transport`

Fields:

| Index | Type        | Documentation |
| ----- | ----------- | ------------- |
| 0     | `Transport` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Transport`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Transport {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Element`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Element {
    pub id: String,
    pub name: Languages,
    pub source: String,
    pub comment: Languages,
    pub enabled: bool,
    pub excluded_scenarios: Vec<String>,
}
```

##### Fields

| Name                 | Type          | Documentation |
| -------------------- | ------------- | ------------- |
| `id`                 | `String`      |               |
| `name`               | `Languages`   |               |
| `source`             | `String`      |               |
| `comment`            | `Languages`   |               |
| `enabled`            | `bool`        |               |
| `excluded_scenarios` | `Vec<String>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Construction`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Construction {
    pub id: String,
    pub name: Languages,
    pub unit: String,
    pub source: String,
    pub comment: Languages,
    pub locked: bool,
}
```

##### Fields

| Name      | Type        | Documentation |
| --------- | ----------- | ------------- |
| `id`      | `String`    |               |
| `name`    | `Languages` |               |
| `unit`    | `String`    |               |
| `source`  | `String`    |               |
| `comment` | `Languages` |               |
| `locked`  | `bool`      |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `DGNBOperationReference`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct DGNBOperationReference {
    pub id: String,
    pub heat_supplement: f64,
    pub electricity_supplement: f64,
}
```

##### Fields

| Name                     | Type     | Documentation |
| ------------------------ | -------- | ------------- |
| `id`                     | `String` |               |
| `heat_supplement`        | `f64`    |               |
| `electricity_supplement` | `f64`    |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Building`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Building {
    pub id: String,
    pub scenario_name: String,
    pub locked: String,
    pub description: Languages,
    pub building_type: String,
    pub heated_floor_area: f64,
    pub gross_area: f64,
    pub integrated_garage: f64,
    pub external_area: f64,
    pub gross_area_above_ground: f64,
    pub person_count: f64,
    pub storeys_above_ground: u16,
    pub storeys_below_ground: u16,
    pub storey_height: f64,
    pub initial_year: u16,
    pub calculation_timespan: u16,
    pub calculation_mode: String,
    pub outside_area: f64,
    pub plot_area: f64,
    pub energy_class: String,
    pub project_type: Option<String>,
}
```

##### Fields

| Name                      | Type             | Documentation |
| ------------------------- | ---------------- | ------------- |
| `id`                      | `String`         |               |
| `scenario_name`           | `String`         |               |
| `locked`                  | `String`         |               |
| `description`             | `Languages`      |               |
| `building_type`           | `String`         |               |
| `heated_floor_area`       | `f64`            |               |
| `gross_area`              | `f64`            |               |
| `integrated_garage`       | `f64`            |               |
| `external_area`           | `f64`            |               |
| `gross_area_above_ground` | `f64`            |               |
| `person_count`            | `f64`            |               |
| `storeys_above_ground`    | `u16`            |               |
| `storeys_below_ground`    | `u16`            |               |
| `storey_height`           | `f64`            |               |
| `initial_year`            | `u16`            |               |
| `calculation_timespan`    | `u16`            |               |
| `calculation_mode`        | `String`         |               |
| `outside_area`            | `f64`            |               |
| `plot_area`               | `f64`            |               |
| `energy_class`            | `String`         |               |
| `project_type`            | `Option<String>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `StageIndicators`

**Attributes:**

- `Other("#[serde(rename_all = \"UPPERCASE\")]")`

```rust
pub struct StageIndicators {
    pub ser: f64,
    pub ep: f64,
    pub odp: f64,
    pub pocp: f64,
    pub per: f64,
    pub adpe: f64,
    pub ap: f64,
    pub gwp: f64,
    pub adpf: f64,
    pub penr: f64,
    pub senr: f64,
}
```

##### Fields

| Name   | Type  | Documentation |
| ------ | ----- | ------------- |
| `ser`  | `f64` |               |
| `ep`   | `f64` |               |
| `odp`  | `f64` |               |
| `pocp` | `f64` |               |
| `per`  | `f64` |               |
| `adpe` | `f64` |               |
| `ap`   | `f64` |               |
| `gwp`  | `f64` |               |
| `adpf` | `f64` |               |
| `penr` | `f64` |               |
| `senr` | `f64` |               |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn update(self: &mut Self, key: &str, value: &f64) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AnyFieldAccess**
  - ```rust
    fn field_as_any(self: &Self, field: &str) -> ::core::option::Option<&dyn ::core::any::Any> { /* ... */ }
    ```

  - ```rust
    fn field_as_any_mut(self: &mut Self, field: &str) -> ::core::option::Option<&mut dyn ::core::any::Any> { /* ... */ }
    ```

  - ```rust
    fn field_names(self: &Self) -> &''static [&''static str] { /* ... */ }
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
    fn clone(self: &Self) -> StageIndicators { /* ... */ }
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

- **FieldAccess**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

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

#### Struct `ProductTransportRoot`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct ProductTransportRoot {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Project`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Project {
    pub id: String,
    pub name: Languages,
    pub owner: String,
    pub address: String,
    pub lca_advisor: String,
    pub building_regulation_version: String,
}
```

##### Fields

| Name                          | Type        | Documentation |
| ----------------------------- | ----------- | ------------- |
| `id`                          | `String`    |               |
| `name`                        | `Languages` |               |
| `owner`                       | `String`    |               |
| `address`                     | `String`    |               |
| `lca_advisor`                 | `String`    |               |
| `building_regulation_version` | `String`    |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Operation`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Operation {
    pub id: String,
    pub electricity_usage: f64,
    pub heat_usage: f64,
    pub electricity_production: f64,
}
```

##### Fields

| Name                     | Type     | Documentation |
| ------------------------ | -------- | ------------- |
| `id`                     | `String` |               |
| `electricity_usage`      | `f64`    |               |
| `heat_usage`             | `f64`    |               |
| `electricity_production` | `f64`    |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `EmbodiedRoot`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct EmbodiedRoot {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `FuelConsumption`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct FuelConsumption {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `ConstructionInstallation`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct ConstructionInstallation {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `ElementModel`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct ElementModel {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `ConstructionProcess`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct ConstructionProcess {
    pub id: String,
}
```

##### Fields

| Name | Type     | Documentation |
| ---- | -------- | ------------- |
| `id` | `String` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Struct `Product`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Product {
    pub id: String,
    pub name: Languages,
    pub source: String,
    pub comment: Languages,
    pub uncertainty_factor: f64,
}
```

##### Fields

| Name                 | Type        | Documentation |
| -------------------- | ----------- | ------------- |
| `id`                 | `String`    |               |
| `name`               | `Languages` |               |
| `source`             | `String`    |               |
| `comment`            | `Languages` |               |
| `uncertainty_factor` | `f64`       |               |

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
    fn clone(self: &Self) -> Product { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

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

#### Struct `Languages`

**Attributes:**

- `Other("#[serde(rename_all = \"PascalCase\")]")`

```rust
pub struct Languages {
    pub english: Option<String>,
    pub german: Option<String>,
    pub norwegian: Option<String>,
    pub danish: Option<String>,
}
```

##### Fields

| Name        | Type             | Documentation |
| ----------- | ---------------- | ------------- |
| `english`   | `Option<String>` |               |
| `german`    | `Option<String>` |               |
| `norwegian` | `Option<String>` |               |
| `danish`    | `Option<String>` |               |

##### Implementations

###### Methods

- ```rust
  pub fn get(self: &Self) -> String { /* ... */ }
  ```

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
    fn clone(self: &Self) -> Languages { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

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

#### Struct `Stage`

**Attributes:**

- `Other("#[serde(rename_all = \"snake_case\")]")`

```rust
pub struct Stage {
    pub id: String,
    pub name: Languages,
    pub comment: Languages,
    pub source: String,
    pub valid_to: Option<String>,
    pub stage: String,
    pub stage_unit: String,
    pub stage_factor: f64,
    pub mass_factor: f64,
    pub scale_factor: f64,
    pub external_source: String,
    pub external_id: String,
    pub external_version: String,
    pub external_url: String,
    pub compliance: String,
    pub data_type: String,
    pub indicators: StageIndicators,
}
```

##### Fields

| Name               | Type              | Documentation |
| ------------------ | ----------------- | ------------- |
| `id`               | `String`          |               |
| `name`             | `Languages`       |               |
| `comment`          | `Languages`       |               |
| `source`           | `String`          |               |
| `valid_to`         | `Option<String>`  |               |
| `stage`            | `String`          |               |
| `stage_unit`       | `String`          |               |
| `stage_factor`     | `f64`             |               |
| `mass_factor`      | `f64`             |               |
| `scale_factor`     | `f64`             |               |
| `external_source`  | `String`          |               |
| `external_id`      | `String`          |               |
| `external_version` | `String`          |               |
| `external_url`     | `String`          |               |
| `compliance`       | `String`          |               |
| `data_type`        | `String`          |               |
| `indicators`       | `StageIndicators` |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(epd: &EPD, stage_name: &str, impact_key: &str, value: &f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn update_indicator(self: &mut Self, key: &str, value: &f64) { /* ... */ }
  ```

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
    fn clone(self: &Self) -> Stage { /* ... */ }
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

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

### Functions

#### Function `epd_from_lcabyg_stages`

```rust
pub fn epd_from_lcabyg_stages(stages: &Vec<Stage>) -> lcax_models::epd::EPD { /* ... */ }
```

## Module `parse`

```rust
pub mod parse { /* ... */ }
```

### Types

#### Enum `NodesAndEdges`

```rust
pub enum NodesAndEdges {
    Node(crate::lcabyg::nodes::Node),
    Edge((crate::lcabyg::edges::EdgeType, String, String)),
    Secret(Vec<i32>),
}
```

##### Variants

###### `Node`

Fields:

| Index | Type                         | Documentation |
| ----- | ---------------------------- | ------------- |
| 0     | `crate::lcabyg::nodes::Node` |               |

###### `Edge`

Fields:

| Index | Type                                               | Documentation |
| ----- | -------------------------------------------------- | ------------- |
| 0     | `(crate::lcabyg::edges::EdgeType, String, String)` |               |

###### `Secret`

Fields:

| Index | Type       | Documentation |
| ----- | ---------- | ------------- |
| 0     | `Vec<i32>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **RefUnwindSafe**
- **Send**
- **Serialize**
  - ````rust
        fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>
    where
        __S: _serde::Serializer { /* ... */ }
        ```
    ````

- **Sync**
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

#### Enum `LCABygResult`

**Attributes:**

- `Other("#[serde(untagged)]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum LCABygResult {
    Project(lcax_models::project::Project),
    Assemblies(Vec<lcax_models::assembly::Assembly>),
    Products(Vec<lcax_models::product::Product>),
    EPDs(Vec<lcax_models::epd::EPD>),
}
```

##### Variants

###### `Project`

Fields:

| Index | Type                            | Documentation |
| ----- | ------------------------------- | ------------- |
| 0     | `lcax_models::project::Project` |               |

###### `Assemblies`

Fields:

| Index | Type                                   | Documentation |
| ----- | -------------------------------------- | ------------- |
| 0     | `Vec<lcax_models::assembly::Assembly>` |               |

###### `Products`

Fields:

| Index | Type                                 | Documentation |
| ----- | ------------------------------------ | ------------- |
| 0     | `Vec<lcax_models::product::Product>` |               |

###### `EPDs`

Fields:

| Index | Type                         | Documentation |
| ----- | ---------------------------- | ------------- |
| 0     | `Vec<lcax_models::epd::EPD>` |               |

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

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LCABygResult) -> bool { /* ... */ }
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

### Functions

#### Function `parse_lcabyg`

Parse an LCAByg project exported as json files.

# Arguments

- `project_data`: JSON formatted string containing the LCAbyg project data
- `result_data`: Optional JSON formatted string containing the result data from the LCAByg project

returns: LCAxProject

```rust
pub fn parse_lcabyg(project_data: &str, result_data: Option<&str>) -> Result<LCABygResult, serde_json::Error> { /* ... */ }
```

## Module `serialize`

```rust
pub mod serialize { /* ... */ }
```

### Functions

#### Function `to_lcabyg`

```rust
pub fn to_lcabyg(objects: &crate::lcabyg::parse::LCABygResult) -> serde_json::Result<String> { /* ... */ }
```

#### Function `serialize_epds`

```rust
pub fn serialize_epds(epds: &Vec<lcax_models::epd::EPD>) -> serde_json::Result<String> { /* ... */ }
```
