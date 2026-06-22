---
title: lcax_models API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.4.3

**Format Version:** 57

# Module `lcax_models`

## Modules

## Module `assembly`

```rust
pub mod assembly { /* ... */ }
```

### Types

#### Struct `Assembly`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Assembly {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub quantity: f64,
    pub unit: crate::shared::Unit,
    pub classification: Option<Vec<Classification>>,
    pub products: Vec<crate::product::ProductReference>,
    pub results: Option<crate::life_cycle_base::Impacts>,
    pub meta_data: Option<crate::shared::MetaData>,
}
```

##### Fields

| Name             | Type                                      | Documentation |
| ---------------- | ----------------------------------------- | ------------- |
| `id`             | `String`                                  |               |
| `name`           | `String`                                  |               |
| `description`    | `Option<String>`                          |               |
| `comment`        | `Option<String>`                          |               |
| `quantity`       | `f64`                                     |               |
| `unit`           | `crate::shared::Unit`                     |               |
| `classification` | `Option<Vec<Classification>>`             |               |
| `products`       | `Vec<crate::product::ProductReference>`   |               |
| `results`        | `Option<crate::life_cycle_base::Impacts>` |               |
| `meta_data`      | `Option<crate::shared::MetaData>`         |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: Option<String>, name: String, description: Option<String>, comment: Option<String>, quantity: f64, unit: Unit, classification: Option<Vec<Classification>>, products: Vec<ProductReference>, results: Option<Impacts>, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> Assembly { /* ... */ }
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
    fn eq(self: &Self, other: &Assembly) -> bool { /* ... */ }
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

#### Struct `Classification`

**Attributes:**

- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Classification {
    pub system: String,
    pub code: String,
    pub name: String,
}
```

##### Fields

| Name     | Type     | Documentation |
| -------- | -------- | ------------- |
| `system` | `String` |               |
| `code`   | `String` |               |
| `name`   | `String` |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(system: String, code: String, name: String) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> Classification { /* ... */ }
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
    fn eq(self: &Self, other: &Classification) -> bool { /* ... */ }
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

#### Enum `AssemblyReference`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[serde(tag = \"type\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum AssemblyReference {
    Assembly(Assembly),
    Reference(crate::shared::Reference),
}
```

##### Variants

###### `Assembly`

Fields:

| Index | Type       | Documentation |
| ----- | ---------- | ------------- |
| 0     | `Assembly` |               |

###### `Reference`

Fields:

| Index | Type                       | Documentation |
| ----- | -------------------------- | ------------- |
| 0     | `crate::shared::Reference` |               |

##### Implementations

###### Methods

- ```rust
  pub fn resolve(self: &Self) -> Result<Assembly, String> { /* ... */ }
  ```

- ```rust
  pub fn resolve_mut(self: &mut Self) -> Result<&mut Assembly, String> { /* ... */ }
  ```

- ```rust
  pub fn new(_type: &str, id: Option<String>, name: String, description: Option<String>, comment: Option<String>, quantity: f64, unit: Unit, classification: Option<Vec<Classification>>, products: Vec<ProductReference>, results: Option<Impacts>, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> AssemblyReference { /* ... */ }
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
    fn eq(self: &Self, other: &AssemblyReference) -> bool { /* ... */ }
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

## Module `epd`

```rust
pub mod epd { /* ... */ }
```

### Types

#### Struct `EPD`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct EPD {
    pub id: String,
    pub name: String,
    pub declared_unit: crate::shared::Unit,
    pub version: String,
    pub published_date: chrono::NaiveDate,
    pub valid_until: chrono::NaiveDate,
    pub source: Option<crate::shared::Source>,
    pub reference_service_life: Option<u32>,
    pub standard: Standard,
    pub comment: Option<String>,
    pub location: lcax_core::country::Country,
    pub subtype: SubType,
    pub conversions: Option<Vec<crate::shared::Conversion>>,
    pub impacts: crate::life_cycle_base::Impacts,
    pub meta_data: Option<crate::shared::MetaData>,
}
```

##### Fields

| Name                     | Type                                     | Documentation |
| ------------------------ | ---------------------------------------- | ------------- |
| `id`                     | `String`                                 |               |
| `name`                   | `String`                                 |               |
| `declared_unit`          | `crate::shared::Unit`                    |               |
| `version`                | `String`                                 |               |
| `published_date`         | `chrono::NaiveDate`                      |               |
| `valid_until`            | `chrono::NaiveDate`                      |               |
| `source`                 | `Option<crate::shared::Source>`          |               |
| `reference_service_life` | `Option<u32>`                            |               |
| `standard`               | `Standard`                               |               |
| `comment`                | `Option<String>`                         |               |
| `location`               | `lcax_core::country::Country`            |               |
| `subtype`                | `SubType`                                |               |
| `conversions`            | `Option<Vec<crate::shared::Conversion>>` |               |
| `impacts`                | `crate::life_cycle_base::Impacts`        |               |
| `meta_data`              | `Option<crate::shared::MetaData>`        |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: Option<String>, name: String, declared_unit: Unit, version: String, published_date: NaiveDate, valid_until: NaiveDate, source: Option<Source>, reference_service_life: Option<u32>, standard: Standard, comment: Option<String>, location: Country, subtype: SubType, conversions: Option<Vec<Conversion>>, impacts: Impacts, meta_data: Option<MetaData>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn loads(value: &str) -> Result<EPD, serde_json::Error> { /* ... */ }
  ```

- ```rust
  pub fn dumps(self: &Self) -> Result<String, serde_json::Error> { /* ... */ }
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
    fn clone(self: &Self) -> EPD { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
    fn eq(self: &Self, other: &EPD) -> bool { /* ... */ }
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

#### Enum `Standard`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum Standard {
    EN15804A1,
    EN15804A2,
    UNKNOWN,
}
```

##### Variants

###### `EN15804A1`

###### `EN15804A2`

###### `UNKNOWN`

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
    fn clone(self: &Self) -> Standard { /* ... */ }
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

  - ```rust
    fn from(value: &String) -> Self { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &Standard) -> bool { /* ... */ }
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

#### Enum `SubType`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum SubType {
    GENERIC,
    SPECIFIC,
    INDUSTRY,
    REPRESENTATIVE,
}
```

##### Variants

###### `GENERIC`

###### `SPECIFIC`

###### `INDUSTRY`

###### `REPRESENTATIVE`

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
    fn clone(self: &Self) -> SubType { /* ... */ }
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

  - ```rust
    fn from(value: &Option<String>) -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```

    Calls `U::from(self)`.

  - ```rust
    fn into(self: Self) -> String { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &SubType) -> bool { /* ... */ }
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

#### Enum `EPDReference`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[serde(tag = \"type\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum EPDReference {
    EPD(EPD),
    Reference(crate::shared::Reference),
}
```

##### Variants

###### `EPD`

Fields:

| Index | Type  | Documentation |
| ----- | ----- | ------------- |
| 0     | `EPD` |               |

###### `Reference`

Fields:

| Index | Type                       | Documentation |
| ----- | -------------------------- | ------------- |
| 0     | `crate::shared::Reference` |               |

##### Implementations

###### Methods

- ```rust
  pub fn resolve(self: &Self) -> Result<&EPD, String> { /* ... */ }
  ```

- ```rust
  pub fn new(_type: &str, id: Option<String>, name: String, declared_unit: Unit, version: String, published_date: NaiveDate, valid_until: NaiveDate, source: Option<Source>, reference_service_life: Option<u32>, standard: Standard, comment: Option<String>, location: Country, subtype: SubType, conversions: Option<Vec<Conversion>>, impacts: Impacts, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> EPDReference { /* ... */ }
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
    fn eq(self: &Self, other: &EPDReference) -> bool { /* ... */ }
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

## Module `generic_impact_data`

```rust
pub mod generic_impact_data { /* ... */ }
```

### Types

#### Struct `GenericData`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct GenericData {
    pub id: String,
    pub name: String,
    pub declared_unit: crate::shared::Unit,
    pub source: Option<crate::shared::Source>,
    pub comment: Option<String>,
    pub conversions: Option<Vec<crate::shared::Conversion>>,
    pub impacts: crate::life_cycle_base::Impacts,
    pub meta_data: Option<crate::shared::MetaData>,
}
```

##### Fields

| Name            | Type                                     | Documentation |
| --------------- | ---------------------------------------- | ------------- |
| `id`            | `String`                                 |               |
| `name`          | `String`                                 |               |
| `declared_unit` | `crate::shared::Unit`                    |               |
| `source`        | `Option<crate::shared::Source>`          |               |
| `comment`       | `Option<String>`                         |               |
| `conversions`   | `Option<Vec<crate::shared::Conversion>>` |               |
| `impacts`       | `crate::life_cycle_base::Impacts`        |               |
| `meta_data`     | `Option<crate::shared::MetaData>`        |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: Option<String>, name: String, declared_unit: Unit, source: Option<Source>, comment: Option<String>, conversions: Option<Vec<Conversion>>, impacts: Impacts, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> GenericData { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
    fn eq(self: &Self, other: &GenericData) -> bool { /* ... */ }
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

#### Enum `GenericDataReference`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[serde(tag = \"type\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum GenericDataReference {
    GenericData(GenericData),
    Reference(crate::shared::Reference),
}
```

##### Variants

###### `GenericData`

Fields:

| Index | Type          | Documentation |
| ----- | ------------- | ------------- |
| 0     | `GenericData` |               |

###### `Reference`

Fields:

| Index | Type                       | Documentation |
| ----- | -------------------------- | ------------- |
| 0     | `crate::shared::Reference` |               |

##### Implementations

###### Methods

- ```rust
  pub fn resolve(self: &Self) -> Result<GenericData, String> { /* ... */ }
  ```

- ```rust
  pub fn new(_type: &str, id: Option<String>, name: String, declared_unit: Unit, source: Option<Source>, comment: Option<String>, conversions: Option<Vec<Conversion>>, impacts: Impacts, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> GenericDataReference { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> GenericDataReference { /* ... */ }
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
    fn eq(self: &Self, other: &GenericDataReference) -> bool { /* ... */ }
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

## Module `life_cycle_base`

```rust
pub mod life_cycle_base { /* ... */ }
```

### Types

#### Enum `LifeCycleModule`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum LifeCycleModule {
    A0,
    A1A3,
    A4,
    A5,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    D,
}
```

##### Variants

###### `A0`

###### `A1A3`

###### `A4`

###### `A5`

###### `B1`

###### `B2`

###### `B3`

###### `B4`

###### `B5`

###### `B6`

###### `B7`

###### `B8`

###### `C1`

###### `C2`

###### `C3`

###### `C4`

###### `D`

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
    fn clone(self: &Self) -> LifeCycleModule { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &LifeCycleModule) -> bool { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &str) -> Result<Self, <Self as >::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**

#### Enum `ImpactCategoryKey`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum ImpactCategoryKey {
    GWP,
    GWP_FOS,
    GWP_BIO,
    GWP_LUL,
    ODP,
    AP,
    EP,
    EP_FW,
    EP_MAR,
    EP_TER,
    POCP,
    ADPE,
    ADPF,
    PENRE,
    PERE,
    PERM,
    PERT,
    PENRT,
    PENRM,
    SM,
    PM,
    WDP,
    IRP,
    ETP_FW,
    HTP_C,
    HTP_NC,
    SQP,
    RSF,
    NRSF,
    FW,
    HWD,
    NHWD,
    RWD,
    CRU,
    MRF,
    MER,
    EEE,
    EET,
}
```

##### Variants

###### `GWP`

###### `GWP_FOS`

###### `GWP_BIO`

###### `GWP_LUL`

###### `ODP`

###### `AP`

###### `EP`

###### `EP_FW`

###### `EP_MAR`

###### `EP_TER`

###### `POCP`

###### `ADPE`

###### `ADPF`

###### `PENRE`

###### `PERE`

###### `PERM`

###### `PERT`

###### `PENRT`

###### `PENRM`

###### `SM`

###### `PM`

###### `WDP`

###### `IRP`

###### `ETP_FW`

###### `HTP_C`

###### `HTP_NC`

###### `SQP`

###### `RSF`

###### `NRSF`

###### `FW`

###### `HWD`

###### `NHWD`

###### `RWD`

###### `CRU`

###### `MRF`

###### `MER`

###### `EEE`

###### `EET`

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
    fn clone(self: &Self) -> ImpactCategoryKey { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DynClone**
  - ```rust
    fn __clone_box(self: &Self, _: Private) -> *mut () { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &ImpactCategoryKey) -> bool { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

#### Struct `ImpactCategory`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct ImpactCategory(/* private field */);
```

##### Fields

| Index | Type      | Documentation   |
| ----- | --------- | --------------- |
| 0     | `private` | _Private field_ |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, key: LifeCycleModule, value: Option<f64>) -> Option<Option<f64>> { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = (&LifeCycleModule, &Option<f64>)> { /* ... */ }
  ```

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, LifeCycleModule, Option<f64>> { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self, key: &LifeCycleModule) -> Option<&Option<f64>> { /* ... */ }
  ```

- ```rust
  pub fn get_mut(self: &mut Self, key: &LifeCycleModule) -> Option<&mut Option<f64>> { /* ... */ }
  ```

- ```rust
  pub fn remove(self: &mut Self, key: &LifeCycleModule) -> Option<Option<f64>> { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
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
    fn clone(self: &Self) -> ImpactCategory { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
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

  - ```rust
    fn from(value: [(LifeCycleModule, Option<f64>); N]) -> Self { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &ImpactCategory) -> bool { /* ... */ }
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

#### Struct `Impacts`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Impacts(/* private field */);
```

##### Fields

| Index | Type      | Documentation   |
| ----- | --------- | --------------- |
| 0     | `private` | _Private field_ |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, key: ImpactCategoryKey, value: ImpactCategory) -> Option<ImpactCategory> { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = (&ImpactCategoryKey, &ImpactCategory)> { /* ... */ }
  ```

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, ImpactCategoryKey, ImpactCategory> { /* ... */ }
  ```

- ```rust
  pub fn get(self: &Self, key: &ImpactCategoryKey) -> Option<&ImpactCategory> { /* ... */ }
  ```

- ```rust
  pub fn entry(self: &mut Self, key: ImpactCategoryKey) -> Entry<''_, ImpactCategoryKey, ImpactCategory> { /* ... */ }
  ```

- ```rust
  pub fn get_mut(self: &mut Self, key: &ImpactCategoryKey) -> Option<&mut ImpactCategory> { /* ... */ }
  ```

- ```rust
  pub fn remove(self: &mut Self, key: &ImpactCategoryKey) -> Option<ImpactCategory> { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
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
    fn clone(self: &Self) -> Impacts { /* ... */ }
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

  - ```rust
    fn from(value: [(ImpactCategoryKey, ImpactCategory); N]) -> Self { /* ... */ }
    ```

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

- **NewResults**
  - ```rust
    fn new_results(impact_categories: &Vec<ImpactCategoryKey>, life_cycle_stage: &Vec<LifeCycleModule>) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Impacts) -> bool { /* ... */ }
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

### Traits

#### Trait `NewResults`

```rust
pub trait NewResults {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `new_results`

##### Implementations

This trait is implemented for the following types:

- `Impacts`

## Module `product`

```rust
pub mod product { /* ... */ }
```

### Types

#### Struct `Product`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub reference_service_life: u32,
    pub impact_data: Vec<ImpactData>,
    pub quantity: f64,
    pub unit: crate::shared::Unit,
    pub transport: Option<Vec<Transport>>,
    pub results: Option<crate::life_cycle_base::Impacts>,
    pub meta_data: Option<crate::shared::MetaData>,
}
```

##### Fields

| Name                     | Type                                      | Documentation |
| ------------------------ | ----------------------------------------- | ------------- |
| `id`                     | `String`                                  |               |
| `name`                   | `String`                                  |               |
| `description`            | `Option<String>`                          |               |
| `reference_service_life` | `u32`                                     |               |
| `impact_data`            | `Vec<ImpactData>`                         |               |
| `quantity`               | `f64`                                     |               |
| `unit`                   | `crate::shared::Unit`                     |               |
| `transport`              | `Option<Vec<Transport>>`                  |               |
| `results`                | `Option<crate::life_cycle_base::Impacts>` |               |
| `meta_data`              | `Option<crate::shared::MetaData>`         |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: Option<String>, name: &str, description: Option<String>, reference_service_life: u32, impact_data: Vec<ImpactData>, quantity: f64, unit: Unit, transport: Option<Vec<Transport>>, results: Option<Impacts>, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn eq(self: &Self, other: &Product) -> bool { /* ... */ }
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

#### Enum `ProductReference`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[serde(tag = \"type\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum ProductReference {
    Product(Product),
    Reference(crate::shared::Reference),
}
```

##### Variants

###### `Product`

Fields:

| Index | Type      | Documentation |
| ----- | --------- | ------------- |
| 0     | `Product` |               |

###### `Reference`

Fields:

| Index | Type                       | Documentation |
| ----- | -------------------------- | ------------- |
| 0     | `crate::shared::Reference` |               |

##### Implementations

###### Methods

- ```rust
  pub fn resolve(self: &Self) -> Result<&Product, String> { /* ... */ }
  ```

- ```rust
  pub fn resolve_mut(self: &mut Self) -> Result<&mut Product, String> { /* ... */ }
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
    fn clone(self: &Self) -> ProductReference { /* ... */ }
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
    fn eq(self: &Self, other: &ProductReference) -> bool { /* ... */ }
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

#### Struct `Transport`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Transport {
    pub id: String,
    pub name: String,
    pub life_cycle_modules: Vec<crate::life_cycle_base::LifeCycleModule>,
    pub distance: f64,
    pub distance_unit: crate::shared::Unit,
    pub impact_data: ImpactData,
}
```

##### Fields

| Name                 | Type                                           | Documentation |
| -------------------- | ---------------------------------------------- | ------------- |
| `id`                 | `String`                                       |               |
| `name`               | `String`                                       |               |
| `life_cycle_modules` | `Vec<crate::life_cycle_base::LifeCycleModule>` |               |
| `distance`           | `f64`                                          |               |
| `distance_unit`      | `crate::shared::Unit`                          |               |
| `impact_data`        | `ImpactData`                                   |               |

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
    fn clone(self: &Self) -> Transport { /* ... */ }
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
    fn eq(self: &Self, other: &Transport) -> bool { /* ... */ }
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

#### Enum `ImpactData`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[serde(untagged)]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum ImpactData {
    EPD(crate::epd::EPDReference),
    GenericData(crate::generic_impact_data::GenericDataReference),
}
```

##### Variants

###### `EPD`

Fields:

| Index | Type                       | Documentation |
| ----- | -------------------------- | ------------- |
| 0     | `crate::epd::EPDReference` |               |

###### `GenericData`

Fields:

| Index | Type                                               | Documentation |
| ----- | -------------------------------------------------- | ------------- |
| 0     | `crate::generic_impact_data::GenericDataReference` |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(_type: &str, id: Option<String>, name: String, declared_unit: Unit, version: String, published_date: NaiveDate, valid_until: NaiveDate, source: Option<Source>, reference_service_life: Option<u32>, standard: Standard, comment: Option<String>, location: Country, subtype: SubType, conversions: Option<Vec<Conversion>>, impacts: Impacts, meta_data: Option<MetaData>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> ImpactData { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ImpactData { /* ... */ }
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
    fn eq(self: &Self, other: &ImpactData) -> bool { /* ... */ }
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

## Module `project`

```rust
pub mod project { /* ... */ }
```

### Types

#### Struct `Project`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub location: Location,
    pub owner: Option<String>,
    pub format_version: String,
    pub lcia_method: Option<String>,
    pub classification_systems: Option<Vec<String>>,
    pub reference_study_period: Option<u8>,
    pub life_cycle_modules: Vec<crate::life_cycle_base::LifeCycleModule>,
    pub impact_categories: Vec<crate::life_cycle_base::ImpactCategoryKey>,
    pub assemblies: Vec<crate::assembly::AssemblyReference>,
    pub results: Option<crate::life_cycle_base::Impacts>,
    pub project_info: Option<BuildingInfo>,
    pub project_phase: ProjectPhase,
    pub software_info: SoftwareInfo,
    pub meta_data: Option<crate::shared::MetaData>,
}
```

##### Fields

| Name                     | Type                                             | Documentation |
| ------------------------ | ------------------------------------------------ | ------------- |
| `id`                     | `String`                                         |               |
| `name`                   | `String`                                         |               |
| `description`            | `Option<String>`                                 |               |
| `comment`                | `Option<String>`                                 |               |
| `location`               | `Location`                                       |               |
| `owner`                  | `Option<String>`                                 |               |
| `format_version`         | `String`                                         |               |
| `lcia_method`            | `Option<String>`                                 |               |
| `classification_systems` | `Option<Vec<String>>`                            |               |
| `reference_study_period` | `Option<u8>`                                     |               |
| `life_cycle_modules`     | `Vec<crate::life_cycle_base::LifeCycleModule>`   |               |
| `impact_categories`      | `Vec<crate::life_cycle_base::ImpactCategoryKey>` |               |
| `assemblies`             | `Vec<crate::assembly::AssemblyReference>`        |               |
| `results`                | `Option<crate::life_cycle_base::Impacts>`        |               |
| `project_info`           | `Option<BuildingInfo>`                           |               |
| `project_phase`          | `ProjectPhase`                                   |               |
| `software_info`          | `SoftwareInfo`                                   |               |
| `meta_data`              | `Option<crate::shared::MetaData>`                |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: Option<String>, name: &str, description: Option<String>, comment: Option<String>, location: Location, owner: Option<String>, format_version: Option<String>, lcia_method: Option<String>, classification_systems: Option<Vec<String>>, reference_study_period: Option<u8>, life_cycle_modules: Vec<LifeCycleModule>, impact_categories: Vec<ImpactCategoryKey>, assemblies: Vec<AssemblyReference>, results: Option<Impacts>, project_info: Option<BuildingInfo>, project_phase: ProjectPhase, software_info: SoftwareInfo, meta_data: Option<MetaData>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn loads(value: &str) -> Result<Project, serde_json::Error> { /* ... */ }
  ```

- ```rust
  pub fn dumps(self: &Self) -> Result<String, serde_json::Error> { /* ... */ }
  ```

- ```rust
  pub fn resolve_impact_categories(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn resolve_life_cycle_stages(self: &mut Self) { /* ... */ }
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
    fn clone(self: &Self) -> Project { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Project { /* ... */ }
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
    fn eq(self: &Self, other: &Project) -> bool { /* ... */ }
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

#### Struct `SoftwareInfo`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct SoftwareInfo {
    pub lca_software: String,
    pub lca_software_version: Option<String>,
    pub goal_and_scope_definition: Option<String>,
    pub calculation_type: Option<String>,
}
```

##### Fields

| Name                        | Type             | Documentation |
| --------------------------- | ---------------- | ------------- |
| `lca_software`              | `String`         |               |
| `lca_software_version`      | `Option<String>` |               |
| `goal_and_scope_definition` | `Option<String>` |               |
| `calculation_type`          | `Option<String>` |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(lca_software: String, lca_software_version: Option<String>, goal_and_scope_definition: Option<String>, calculation_type: Option<String>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> SoftwareInfo { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SoftwareInfo { /* ... */ }
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
    fn eq(self: &Self, other: &SoftwareInfo) -> bool { /* ... */ }
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

#### Enum `ProjectPhase`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum ProjectPhase {
    STRATEGIC_DESIGN,
    CONCEPT_DESIGN,
    TECHNICAL_DESIGN,
    CONSTRUCTION,
    POST_COMPLETION,
    IN_USE,
    OTHER,
}
```

##### Variants

###### `STRATEGIC_DESIGN`

###### `CONCEPT_DESIGN`

###### `TECHNICAL_DESIGN`

###### `CONSTRUCTION`

###### `POST_COMPLETION`

###### `IN_USE`

###### `OTHER`

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
    fn clone(self: &Self) -> ProjectPhase { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ProjectPhase { /* ... */ }
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
    fn eq(self: &Self, other: &ProjectPhase) -> bool { /* ... */ }
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

#### Struct `Location`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Location {
    pub country: lcax_core::country::Country,
    pub city: Option<String>,
    pub address: Option<String>,
}
```

##### Fields

| Name      | Type                          | Documentation |
| --------- | ----------------------------- | ------------- |
| `country` | `lcax_core::country::Country` |               |
| `city`    | `Option<String>`              |               |
| `address` | `Option<String>`              |               |

##### Implementations

###### Methods

- ```rust
  pub fn new(country: Country, city: Option<String>, address: Option<String>) -> Self { /* ... */ }
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
    fn clone(self: &Self) -> Location { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Location { /* ... */ }
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
    fn eq(self: &Self, other: &Location) -> bool { /* ... */ }
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

#### Struct `BuildingInfo`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct BuildingInfo {
    pub building_type: BuildingType,
    pub building_typology: Vec<BuildingTypology>,
    pub certifications: Option<Vec<String>>,
    pub building_mass: Option<ValueUnit>,
    pub building_height: Option<ValueUnit>,
    pub gross_floor_area: Option<AreaType>,
    pub heated_floor_area: Option<AreaType>,
    pub building_footprint: Option<ValueUnit>,
    pub floors_above_ground: u16,
    pub floors_below_ground: Option<u16>,
    pub roof_type: Option<RoofType>,
    pub frame_type: Option<String>,
    pub building_completion_year: Option<u16>,
    pub building_permit_year: Option<u16>,
    pub energy_demand_heating: Option<f64>,
    pub energy_supply_heating: Option<f64>,
    pub energy_demand_electricity: Option<f64>,
    pub energy_supply_electricity: Option<f64>,
    pub exported_electricity: Option<f64>,
    pub general_energy_class: GeneralEnergyClass,
    pub local_energy_class: Option<String>,
    pub building_users: Option<u32>,
    pub building_model_scope: Option<Vec<BuildingModelScope>>,
}
```

##### Fields

| Name                        | Type                              | Documentation |
| --------------------------- | --------------------------------- | ------------- |
| `building_type`             | `BuildingType`                    |               |
| `building_typology`         | `Vec<BuildingTypology>`           |               |
| `certifications`            | `Option<Vec<String>>`             |               |
| `building_mass`             | `Option<ValueUnit>`               |               |
| `building_height`           | `Option<ValueUnit>`               |               |
| `gross_floor_area`          | `Option<AreaType>`                |               |
| `heated_floor_area`         | `Option<AreaType>`                |               |
| `building_footprint`        | `Option<ValueUnit>`               |               |
| `floors_above_ground`       | `u16`                             |               |
| `floors_below_ground`       | `Option<u16>`                     |               |
| `roof_type`                 | `Option<RoofType>`                |               |
| `frame_type`                | `Option<String>`                  |               |
| `building_completion_year`  | `Option<u16>`                     |               |
| `building_permit_year`      | `Option<u16>`                     |               |
| `energy_demand_heating`     | `Option<f64>`                     |               |
| `energy_supply_heating`     | `Option<f64>`                     |               |
| `energy_demand_electricity` | `Option<f64>`                     |               |
| `energy_supply_electricity` | `Option<f64>`                     |               |
| `exported_electricity`      | `Option<f64>`                     |               |
| `general_energy_class`      | `GeneralEnergyClass`              |               |
| `local_energy_class`        | `Option<String>`                  |               |
| `building_users`            | `Option<u32>`                     |               |
| `building_model_scope`      | `Option<Vec<BuildingModelScope>>` |               |

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
    fn clone(self: &Self) -> BuildingInfo { /* ... */ }
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
    fn eq(self: &Self, other: &BuildingInfo) -> bool { /* ... */ }
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

#### Struct `AreaType`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct AreaType {
    pub value: f64,
    pub unit: crate::shared::Unit,
    pub definition: String,
}
```

##### Fields

| Name         | Type                  | Documentation |
| ------------ | --------------------- | ------------- |
| `value`      | `f64`                 |               |
| `unit`       | `crate::shared::Unit` |               |
| `definition` | `String`              |               |

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
    fn clone(self: &Self) -> AreaType { /* ... */ }
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
    fn eq(self: &Self, other: &AreaType) -> bool { /* ... */ }
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

#### Struct `ValueUnit`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct ValueUnit {
    pub value: f64,
    pub unit: crate::shared::Unit,
}
```

##### Fields

| Name    | Type                  | Documentation |
| ------- | --------------------- | ------------- |
| `value` | `f64`                 |               |
| `unit`  | `crate::shared::Unit` |               |

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
    fn clone(self: &Self) -> ValueUnit { /* ... */ }
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
    fn eq(self: &Self, other: &ValueUnit) -> bool { /* ... */ }
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

#### Enum `RoofType`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum RoofType {
    FLAT,
    PITCHED,
    SADDLE,
    PYRAMID,
    UNKNOWN,
    OTHER,
}
```

##### Variants

###### `FLAT`

###### `PITCHED`

###### `SADDLE`

###### `PYRAMID`

###### `UNKNOWN`

###### `OTHER`

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
    fn clone(self: &Self) -> RoofType { /* ... */ }
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
    fn eq(self: &Self, other: &RoofType) -> bool { /* ... */ }
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

#### Enum `GeneralEnergyClass`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum GeneralEnergyClass {
    EXISTING,
    STANDARD,
    ADVANCED,
    UNKNOWN,
}
```

##### Variants

###### `EXISTING`

###### `STANDARD`

###### `ADVANCED`

###### `UNKNOWN`

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
    fn clone(self: &Self) -> GeneralEnergyClass { /* ... */ }
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

  - ```rust
    fn from(class: &String) -> Self { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &GeneralEnergyClass) -> bool { /* ... */ }
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

#### Enum `BuildingModelScope`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum BuildingModelScope {
    FACILITATING_WORKS,
    SUBSTRUCTURE,
    SUPERSTRUCTURE_FRAME,
    SUPERSTRUCTURE_ENVELOPE,
    SUPERSTRUCTURE_INTERNAL_ELEMENTS,
    FINISHES,
    BUILDING_SERVICES,
    EXTERNAL_WORKS,
    FF_E,
}
```

##### Variants

###### `FACILITATING_WORKS`

###### `SUBSTRUCTURE`

###### `SUPERSTRUCTURE_FRAME`

###### `SUPERSTRUCTURE_ENVELOPE`

###### `SUPERSTRUCTURE_INTERNAL_ELEMENTS`

###### `FINISHES`

###### `BUILDING_SERVICES`

###### `EXTERNAL_WORKS`

###### `FF_E`

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
    fn clone(self: &Self) -> BuildingModelScope { /* ... */ }
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
    fn eq(self: &Self, other: &BuildingModelScope) -> bool { /* ... */ }
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

#### Enum `BuildingType`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum BuildingType {
    NEW_CONSTRUCTION_WORKS,
    DEMOLITION,
    DECONSTRUCTION_AND_NEW_CONSTRUCTION_WORKS,
    RETROFIT_WORKS,
    EXTENSION_WORKS,
    RETROFIT_AND_EXTENSION_WORKS,
    FIT_OUT_WORKS,
    OPERATIONS,
    UNKNOWN,
    OTHER,
}
```

##### Variants

###### `NEW_CONSTRUCTION_WORKS`

###### `DEMOLITION`

###### `DECONSTRUCTION_AND_NEW_CONSTRUCTION_WORKS`

###### `RETROFIT_WORKS`

###### `EXTENSION_WORKS`

###### `RETROFIT_AND_EXTENSION_WORKS`

###### `FIT_OUT_WORKS`

###### `OPERATIONS`

###### `UNKNOWN`

###### `OTHER`

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
    fn clone(self: &Self) -> BuildingType { /* ... */ }
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
    fn eq(self: &Self, other: &BuildingType) -> bool { /* ... */ }
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

#### Enum `BuildingTypology`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum BuildingTypology {
    OFFICE,
    RESIDENTIAL,
    PUBLIC,
    COMMERCIAL,
    INDUSTRIAL,
    INFRASTRUCTURE,
    AGRICULTURAL,
    EDUCATIONAL,
    HEALTH,
    SCIENCE,
    CULTURE,
    PARKING_LOGISTIC,
    UNKNOWN,
    OTHER,
}
```

##### Variants

###### `OFFICE`

###### `RESIDENTIAL`

###### `PUBLIC`

###### `COMMERCIAL`

###### `INDUSTRIAL`

###### `INFRASTRUCTURE`

###### `AGRICULTURAL`

###### `EDUCATIONAL`

###### `HEALTH`

###### `SCIENCE`

###### `CULTURE`

###### `PARKING_LOGISTIC`

###### `UNKNOWN`

###### `OTHER`

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
    fn clone(self: &Self) -> BuildingTypology { /* ... */ }
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

  - ```rust
    fn from(unit: &String) -> Self { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &BuildingTypology) -> bool { /* ... */ }
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

## Module `shared`

```rust
pub mod shared { /* ... */ }
```

### Types

#### Enum `Unit`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum Unit {
    M,
    M2,
    M3,
    KG,
    TONES,
    PCS,
    KWH,
    L,
    M2R1,
    KM,
    TONES_KM,
    KGM3,
    UNKNOWN,
}
```

##### Variants

###### `M`

###### `M2`

###### `M3`

###### `KG`

###### `TONES`

###### `PCS`

###### `KWH`

###### `L`

###### `M2R1`

###### `KM`

###### `TONES_KM`

###### `KGM3`

###### `UNKNOWN`

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
    fn clone(self: &Self) -> Unit { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Unit { /* ... */ }
    ```

- **Deserialize**
  - ````rust
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>
    where
        __D: _serde::Deserializer<''de> { /* ... */ }
        ```
    ````

- **DeserializeOwned**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

  - ```rust
    fn from(unit: &String) -> Self { /* ... */ }
    ```

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
    fn eq(self: &Self, other: &Unit) -> bool { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

#### Struct `Conversion`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Conversion {
    pub value: f64,
    pub to: Unit,
    pub meta_data: Option<MetaData>,
}
```

##### Fields

| Name        | Type               | Documentation |
| ----------- | ------------------ | ------------- |
| `value`     | `f64`              |               |
| `to`        | `Unit`             |               |
| `meta_data` | `Option<MetaData>` |               |

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
    fn clone(self: &Self) -> Conversion { /* ... */ }
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
    fn eq(self: &Self, other: &Conversion) -> bool { /* ... */ }
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

#### Struct `Source`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}
```

##### Fields

| Name   | Type             | Documentation |
| ------ | ---------------- | ------------- |
| `name` | `String`         |               |
| `url`  | `Option<String>` |               |

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
    fn clone(self: &Self) -> Source { /* ... */ }
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
    fn eq(self: &Self, other: &Source) -> bool { /* ... */ }
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

#### Struct `Reference`

**Attributes:**

- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct Reference {
    pub uri: String,
    pub format: Option<String>,
    pub version: Option<String>,
    pub overrides: Option<std::collections::HashMap<String, Option<lcax_core::value::AnyValue>>>,
}
```

##### Fields

| Name        | Type                                                                            | Documentation |
| ----------- | ------------------------------------------------------------------------------- | ------------- |
| `uri`       | `String`                                                                        |               |
| `format`    | `Option<String>`                                                                |               |
| `version`   | `Option<String>`                                                                |               |
| `overrides` | `Option<std::collections::HashMap<String, Option<lcax_core::value::AnyValue>>>` |               |

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
    fn clone(self: &Self) -> Reference { /* ... */ }
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
    fn eq(self: &Self, other: &Reference) -> bool { /* ... */ }
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

#### Type Alias `MetaData`

**Attributes:**

- `Other("#[attr = CfgAttrTrace]")`

```rust
pub type MetaData = std::collections::HashMap<String, Option<lcax_core::value::AnyValue>>;
```
