---
title: lcax_validation API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.4.3

**Format Version:** 57

# Module `lcax_validation`

## Modules

## Module `model`

```rust
pub mod model { /* ... */ }
```

### Types

#### Enum `Level`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum Level {
    Project,
    Assembly,
    Product,
    ImpactData,
}
```

##### Variants

###### `Project`

###### `Assembly`

###### `Product`

###### `ImpactData`

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
    fn clone(self: &Self) -> Level { /* ... */ }
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
- **Validatable**
  - ```rust
    fn validate(self: &Self, validator: ValidPhrase<''v>) -> Result<(), InnerValidatorError<FieldNames, String>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: ValidPhrase<''v>) -> Result<T, InnerValidatorError<FieldNames, String>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<(), InnerValidatorError<FieldNames, M>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<T, InnerValidatorError<FieldNames, M>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, ()>) -> Result<(), InnerValidatorError<FieldNames, M2>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, ()>) -> Result<T, InnerValidatorError<FieldNames, M2>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```
    ````

#### Struct `ValidationSchema`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct ValidationSchema {
    pub level: Level,
    pub field: String,
    pub rule: ValidationRule,
}
```

##### Fields

| Name    | Type             | Documentation |
| ------- | ---------------- | ------------- |
| `level` | `Level`          |               |
| `field` | `String`         |               |
| `rule`  | `ValidationRule` |               |

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
    fn clone(self: &Self) -> ValidationSchema { /* ... */ }
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
- **Validatable**
  - ```rust
    fn validate(self: &Self, validator: ValidPhrase<''v>) -> Result<(), InnerValidatorError<FieldNames, String>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: ValidPhrase<''v>) -> Result<T, InnerValidatorError<FieldNames, String>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<(), InnerValidatorError<FieldNames, M>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<T, InnerValidatorError<FieldNames, M>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, ()>) -> Result<(), InnerValidatorError<FieldNames, M2>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, ()>) -> Result<T, InnerValidatorError<FieldNames, M2>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```
    ````

#### Struct `ValidationRule`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct ValidationRule {
    pub range: Option<[f64; 2]>,
    pub includes: Option<String>,
    pub required: Option<bool>,
    pub equal: Option<String>,
    pub greater: Option<f64>,
    pub less: Option<f64>,
    pub one_of: Option<Vec<String>>,
}
```

##### Fields

| Name       | Type                  | Documentation |
| ---------- | --------------------- | ------------- |
| `range`    | `Option<[f64; 2]>`    |               |
| `includes` | `Option<String>`      |               |
| `required` | `Option<bool>`        |               |
| `equal`    | `Option<String>`      |               |
| `greater`  | `Option<f64>`         |               |
| `less`     | `Option<f64>`         |               |
| `one_of`   | `Option<Vec<String>>` |               |

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
    fn clone(self: &Self) -> ValidationRule { /* ... */ }
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
- **Validatable**
  - ```rust
    fn validate(self: &Self, validator: ValidPhrase<''v>) -> Result<(), InnerValidatorError<FieldNames, String>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: ValidPhrase<''v>) -> Result<T, InnerValidatorError<FieldNames, String>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<(), InnerValidatorError<FieldNames, M>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<T, InnerValidatorError<FieldNames, M>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, ()>) -> Result<(), InnerValidatorError<FieldNames, M2>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, ()>) -> Result<T, InnerValidatorError<FieldNames, M2>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```
    ````

#### Struct `ValidationResult`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub struct ValidationResult {
    pub field: String,
    pub message: String,
}
```

##### Fields

| Name      | Type     | Documentation |
| --------- | -------- | ------------- |
| `field`   | `String` |               |
| `message` | `String` |               |

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
    fn clone(self: &Self) -> ValidationResult { /* ... */ }
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
- **Validatable**
  - ```rust
    fn validate(self: &Self, validator: ValidPhrase<''v>) -> Result<(), InnerValidatorError<FieldNames, String>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: ValidPhrase<''v>) -> Result<T, InnerValidatorError<FieldNames, String>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<(), InnerValidatorError<FieldNames, M>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, HashMap<MessageKey<''_>, M>>) -> Result<T, InnerValidatorError<FieldNames, M>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```

    ````

  - ```rust
    fn validate(self: &Self, validator: InnerValidator<M, ()>) -> Result<(), InnerValidatorError<FieldNames, M2>> { /* ... */ }
    ```

  - ````rust
        fn validate_mut<''de>(self: Self, validator: InnerValidator<M, ()>) -> Result<T, InnerValidatorError<FieldNames, M2>>
    where
        T: Deserialize<''de> { /* ... */ }
        ```
    ````

## Module `rules`

```rust
pub mod rules { /* ... */ }
```

### Modules

## Module `equal`

```rust
pub mod equal { /* ... */ }
```

### Types

#### Struct `Equal`

```rust
pub struct Equal<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
| ----- | ---- | ------------- |
| 0     | `T`  |               |

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
    fn clone(self: &Self) -> Equal<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CoreRule**
  - ```rust
    fn call(self: &mut Self, data: &mut ValueMap) -> Result<(), <T as CoreRule<ValueMap, ()>>::Message> { /* ... */ }
    ```
    Rule specific implementation, data is gived type all field's value, and current field index.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoRuleList**
  - ```rust
    fn into_list(self: Self) -> RuleList<ValueMap, M> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Rule**
  - ```rust
    fn message(self: &Self) -> <Self as >::Message { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, data: &mut Value) -> bool { /* ... */ }
    ```

- **RuleExt**
  - ````rust
        fn and<R2>(self: Self, other: R2) -> RuleList<Input, Msg>
    where
        R2: CoreRule<Input, (), Message = Msg> { /* ... */ }
        ```

    ````

  - ````rust
        fn custom<F, V>(self: Self, other: F) -> RuleList<Input, Msg>
    where
        F: for<''a> FnOnce(&''a mut V) -> Result<(), Msg> + CoreRule<Input, V, Message = Msg>,
        V: FromValue + ''static { /* ... */ }
        ```
    ````

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

## Module `greater`

```rust
pub mod greater { /* ... */ }
```

### Types

#### Struct `Greater`

```rust
pub struct Greater<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
| ----- | ---- | ------------- |
| 0     | `T`  |               |

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
    fn clone(self: &Self) -> Greater<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CoreRule**
  - ```rust
    fn call(self: &mut Self, data: &mut ValueMap) -> Result<(), <T as CoreRule<ValueMap, ()>>::Message> { /* ... */ }
    ```
    Rule specific implementation, data is gived type all field's value, and current field index.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoRuleList**
  - ```rust
    fn into_list(self: Self) -> RuleList<ValueMap, M> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Rule**
  - ```rust
    fn message(self: &Self) -> <Self as >::Message { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, data: &mut Value) -> bool { /* ... */ }
    ```

- **RuleExt**
  - ````rust
        fn and<R2>(self: Self, other: R2) -> RuleList<Input, Msg>
    where
        R2: CoreRule<Input, (), Message = Msg> { /* ... */ }
        ```

    ````

  - ````rust
        fn custom<F, V>(self: Self, other: F) -> RuleList<Input, Msg>
    where
        F: for<''a> FnOnce(&''a mut V) -> Result<(), Msg> + CoreRule<Input, V, Message = Msg>,
        V: FromValue + ''static { /* ... */ }
        ```
    ````

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

## Module `includes`

```rust
pub mod includes { /* ... */ }
```

### Types

#### Struct `Includes`

```rust
pub struct Includes<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
| ----- | ---- | ------------- |
| 0     | `T`  |               |

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
    fn clone(self: &Self) -> Includes<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CoreRule**
  - ```rust
    fn call(self: &mut Self, data: &mut ValueMap) -> Result<(), <T as CoreRule<ValueMap, ()>>::Message> { /* ... */ }
    ```
    Rule specific implementation, data is gived type all field's value, and current field index.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoRuleList**
  - ```rust
    fn into_list(self: Self) -> RuleList<ValueMap, M> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Rule**
  - ```rust
    fn message(self: &Self) -> <Self as >::Message { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, data: &mut Value) -> bool { /* ... */ }
    ```

- **RuleExt**
  - ````rust
        fn and<R2>(self: Self, other: R2) -> RuleList<Input, Msg>
    where
        R2: CoreRule<Input, (), Message = Msg> { /* ... */ }
        ```

    ````

  - ````rust
        fn custom<F, V>(self: Self, other: F) -> RuleList<Input, Msg>
    where
        F: for<''a> FnOnce(&''a mut V) -> Result<(), Msg> + CoreRule<Input, V, Message = Msg>,
        V: FromValue + ''static { /* ... */ }
        ```
    ````

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

## Module `less`

```rust
pub mod less { /* ... */ }
```

### Types

#### Struct `Less`

```rust
pub struct Less<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
| ----- | ---- | ------------- |
| 0     | `T`  |               |

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
    fn clone(self: &Self) -> Less<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CoreRule**
  - ```rust
    fn call(self: &mut Self, data: &mut ValueMap) -> Result<(), <T as CoreRule<ValueMap, ()>>::Message> { /* ... */ }
    ```
    Rule specific implementation, data is gived type all field's value, and current field index.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoRuleList**
  - ```rust
    fn into_list(self: Self) -> RuleList<ValueMap, M> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Rule**
  - ```rust
    fn message(self: &Self) -> <Self as >::Message { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, data: &mut Value) -> bool { /* ... */ }
    ```

- **RuleExt**
  - ````rust
        fn and<R2>(self: Self, other: R2) -> RuleList<Input, Msg>
    where
        R2: CoreRule<Input, (), Message = Msg> { /* ... */ }
        ```

    ````

  - ````rust
        fn custom<F, V>(self: Self, other: F) -> RuleList<Input, Msg>
    where
        F: for<''a> FnOnce(&''a mut V) -> Result<(), Msg> + CoreRule<Input, V, Message = Msg>,
        V: FromValue + ''static { /* ... */ }
        ```
    ````

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

## Module `one_of`

```rust
pub mod one_of { /* ... */ }
```

### Types

#### Struct `OneOf`

```rust
pub struct OneOf<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
| ----- | ---- | ------------- |
| 0     | `T`  |               |

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
    fn clone(self: &Self) -> OneOf<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **CoreRule**
  - ```rust
    fn call(self: &mut Self, data: &mut ValueMap) -> Result<(), <T as CoreRule<ValueMap, ()>>::Message> { /* ... */ }
    ```
    Rule specific implementation, data is gived type all field's value, and current field index.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoRuleList**
  - ```rust
    fn into_list(self: Self) -> RuleList<ValueMap, M> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Rule**
  - ```rust
    fn message(self: &Self) -> <Self as >::Message { /* ... */ }
    ```

  - ```rust
    fn call(self: &mut Self, data: &mut Value) -> bool { /* ... */ }
    ```

- **RuleExt**
  - ````rust
        fn and<R2>(self: Self, other: R2) -> RuleList<Input, Msg>
    where
        R2: CoreRule<Input, (), Message = Msg> { /* ... */ }
        ```

    ````

  - ````rust
        fn custom<F, V>(self: Self, other: F) -> RuleList<Input, Msg>
    where
        F: for<''a> FnOnce(&''a mut V) -> Result<(), Msg> + CoreRule<Input, V, Message = Msg>,
        V: FromValue + ''static { /* ... */ }
        ```
    ````

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

### Re-exports

#### Re-export `Equal`

```rust
pub use equal::Equal;
```

#### Re-export `Greater`

```rust
pub use greater::Greater;
```

#### Re-export `Includes`

```rust
pub use includes::Includes;
```

#### Re-export `Less`

```rust
pub use less::Less;
```

#### Re-export `OneOf`

```rust
pub use one_of::OneOf;
```

## Module `validate`

```rust
pub mod validate { /* ... */ }
```

### Functions

#### Function `validate`

```rust
pub fn validate(project: &lcax_models::project::Project, validation_schema: &Vec<crate::model::ValidationSchema>) -> Vec<crate::model::ValidationResult> { /* ... */ }
```

## Re-exports

### Re-export `ValidationSchema`

```rust
pub use model::ValidationSchema;
```

### Re-export `validate`

```rust
pub use validate::validate;
```
