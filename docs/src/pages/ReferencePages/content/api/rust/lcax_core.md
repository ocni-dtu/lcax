---
title: lcax_core API Reference
description: Rust - API Reference
---

# Crate Documentation

**Version:** 3.4.3

**Format Version:** 57

# Module `lcax_core`

## Modules

## Module `country`

```rust
pub mod country { /* ... */ }
```

### Types

#### Enum `Country`

**Attributes:**

- `Other("#[serde(rename_all = \"lowercase\")]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum Country {
    UNKNOWN,
    AFG,
    ALA,
    ALB,
    DZA,
    ASM,
    AND,
    AGO,
    AIA,
    ATA,
    ATG,
    ARG,
    ARM,
    ABW,
    AUS,
    AUT,
    AZE,
    BHS,
    BHR,
    BGD,
    BRB,
    BLR,
    BEL,
    BLZ,
    BEN,
    BMU,
    BTN,
    BOL,
    BES,
    BIH,
    BWA,
    BVT,
    BRA,
    IOT,
    BRN,
    BGR,
    BFA,
    BDI,
    CPV,
    KHM,
    CMR,
    CAN,
    CYM,
    CAF,
    TCD,
    CHL,
    CHN,
    CXR,
    CCK,
    COL,
    COM,
    COG,
    COD,
    COK,
    CRI,
    CIV,
    HRV,
    CUB,
    CUW,
    CYP,
    CZE,
    DNK,
    DJI,
    DMA,
    DOM,
    ECU,
    EGY,
    SLV,
    GNQ,
    ERI,
    EST,
    SWZ,
    ETH,
    FLK,
    FRO,
    FJI,
    FIN,
    FRA,
    GUF,
    PYF,
    ATF,
    GAB,
    GMB,
    GEO,
    DEU,
    GHA,
    GIB,
    GRC,
    GRL,
    GRD,
    GLP,
    GUM,
    GTM,
    GGY,
    GIN,
    GNB,
    GUY,
    HTI,
    HMD,
    VAT,
    HND,
    HKG,
    HUN,
    ISL,
    IND,
    IDN,
    IRN,
    IRQ,
    IRL,
    IMN,
    ISR,
    ITA,
    JAM,
    JPN,
    JEY,
    JOR,
    KAZ,
    KEN,
    KIR,
    PRK,
    KOR,
    KWT,
    KGZ,
    LAO,
    LVA,
    LBN,
    LSO,
    LBR,
    LBY,
    LIE,
    LTU,
    LUX,
    MAC,
    MDG,
    MWI,
    MYS,
    MDV,
    MLI,
    MLT,
    MHL,
    MTQ,
    MRT,
    MUS,
    MYT,
    MEX,
    FSM,
    MDA,
    MCO,
    MNG,
    MNE,
    MSR,
    MAR,
    MOZ,
    MMR,
    NAM,
    NRU,
    NPL,
    NLD,
    NCL,
    NZL,
    NIC,
    NER,
    NGA,
    NIU,
    NFK,
    MKD,
    MNP,
    NOR,
    OMN,
    PAK,
    PLW,
    PSE,
    PAN,
    PNG,
    PRY,
    PER,
    PHL,
    PCN,
    POL,
    PRT,
    PRI,
    QAT,
    REU,
    ROU,
    RUS,
    RWA,
    BLM,
    SHN,
    KNA,
    LCA,
    MAF,
    SPM,
    VCT,
    WSM,
    SMR,
    STP,
    SAU,
    SEN,
    SRB,
    SYC,
    SLE,
    SGP,
    SXM,
    SVK,
    SVN,
    SLB,
    SOM,
    ZAF,
    SGS,
    SSD,
    ESP,
    LKA,
    SDN,
    SUR,
    SJM,
    SWE,
    CHE,
    SYR,
    TWN,
    TJK,
    TZA,
    THA,
    TLS,
    TGO,
    TKL,
    TON,
    TTO,
    TUN,
    TUR,
    TKM,
    TCA,
    TUV,
    UGA,
    UKR,
    ARE,
    GBR,
    USA,
    UMI,
    URY,
    UZB,
    VUT,
    VEN,
    VNM,
    VGB,
    VIR,
    WLF,
    ESH,
    YEM,
    ZMB,
    ZWE,
}
```

##### Variants

###### `UNKNOWN`

###### `AFG`

###### `ALA`

###### `ALB`

###### `DZA`

###### `ASM`

###### `AND`

###### `AGO`

###### `AIA`

###### `ATA`

###### `ATG`

###### `ARG`

###### `ARM`

###### `ABW`

###### `AUS`

###### `AUT`

###### `AZE`

###### `BHS`

###### `BHR`

###### `BGD`

###### `BRB`

###### `BLR`

###### `BEL`

###### `BLZ`

###### `BEN`

###### `BMU`

###### `BTN`

###### `BOL`

###### `BES`

###### `BIH`

###### `BWA`

###### `BVT`

###### `BRA`

###### `IOT`

###### `BRN`

###### `BGR`

###### `BFA`

###### `BDI`

###### `CPV`

###### `KHM`

###### `CMR`

###### `CAN`

###### `CYM`

###### `CAF`

###### `TCD`

###### `CHL`

###### `CHN`

###### `CXR`

###### `CCK`

###### `COL`

###### `COM`

###### `COG`

###### `COD`

###### `COK`

###### `CRI`

###### `CIV`

###### `HRV`

###### `CUB`

###### `CUW`

###### `CYP`

###### `CZE`

###### `DNK`

###### `DJI`

###### `DMA`

###### `DOM`

###### `ECU`

###### `EGY`

###### `SLV`

###### `GNQ`

###### `ERI`

###### `EST`

###### `SWZ`

###### `ETH`

###### `FLK`

###### `FRO`

###### `FJI`

###### `FIN`

###### `FRA`

###### `GUF`

###### `PYF`

###### `ATF`

###### `GAB`

###### `GMB`

###### `GEO`

###### `DEU`

###### `GHA`

###### `GIB`

###### `GRC`

###### `GRL`

###### `GRD`

###### `GLP`

###### `GUM`

###### `GTM`

###### `GGY`

###### `GIN`

###### `GNB`

###### `GUY`

###### `HTI`

###### `HMD`

###### `VAT`

###### `HND`

###### `HKG`

###### `HUN`

###### `ISL`

###### `IND`

###### `IDN`

###### `IRN`

###### `IRQ`

###### `IRL`

###### `IMN`

###### `ISR`

###### `ITA`

###### `JAM`

###### `JPN`

###### `JEY`

###### `JOR`

###### `KAZ`

###### `KEN`

###### `KIR`

###### `PRK`

###### `KOR`

###### `KWT`

###### `KGZ`

###### `LAO`

###### `LVA`

###### `LBN`

###### `LSO`

###### `LBR`

###### `LBY`

###### `LIE`

###### `LTU`

###### `LUX`

###### `MAC`

###### `MDG`

###### `MWI`

###### `MYS`

###### `MDV`

###### `MLI`

###### `MLT`

###### `MHL`

###### `MTQ`

###### `MRT`

###### `MUS`

###### `MYT`

###### `MEX`

###### `FSM`

###### `MDA`

###### `MCO`

###### `MNG`

###### `MNE`

###### `MSR`

###### `MAR`

###### `MOZ`

###### `MMR`

###### `NAM`

###### `NRU`

###### `NPL`

###### `NLD`

###### `NCL`

###### `NZL`

###### `NIC`

###### `NER`

###### `NGA`

###### `NIU`

###### `NFK`

###### `MKD`

###### `MNP`

###### `NOR`

###### `OMN`

###### `PAK`

###### `PLW`

###### `PSE`

###### `PAN`

###### `PNG`

###### `PRY`

###### `PER`

###### `PHL`

###### `PCN`

###### `POL`

###### `PRT`

###### `PRI`

###### `QAT`

###### `REU`

###### `ROU`

###### `RUS`

###### `RWA`

###### `BLM`

###### `SHN`

###### `KNA`

###### `LCA`

###### `MAF`

###### `SPM`

###### `VCT`

###### `WSM`

###### `SMR`

###### `STP`

###### `SAU`

###### `SEN`

###### `SRB`

###### `SYC`

###### `SLE`

###### `SGP`

###### `SXM`

###### `SVK`

###### `SVN`

###### `SLB`

###### `SOM`

###### `ZAF`

###### `SGS`

###### `SSD`

###### `ESP`

###### `LKA`

###### `SDN`

###### `SUR`

###### `SJM`

###### `SWE`

###### `CHE`

###### `SYR`

###### `TWN`

###### `TJK`

###### `TZA`

###### `THA`

###### `TLS`

###### `TGO`

###### `TKL`

###### `TON`

###### `TTO`

###### `TUN`

###### `TUR`

###### `TKM`

###### `TCA`

###### `TUV`

###### `UGA`

###### `UKR`

###### `ARE`

###### `GBR`

###### `USA`

###### `UMI`

###### `URY`

###### `UZB`

###### `VUT`

###### `VEN`

###### `VNM`

###### `VGB`

###### `VIR`

###### `WLF`

###### `ESH`

###### `YEM`

###### `ZMB`

###### `ZWE`

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
    fn clone(self: &Self) -> Country { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Country { /* ... */ }
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
    fn from(value: &str) -> Self { /* ... */ }
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
    fn eq(self: &Self, other: &Country) -> bool { /* ... */ }
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

## Module `dates`

```rust
pub mod dates { /* ... */ }
```

### Functions

#### Function `serialize_yyyy_mm_dd`

```rust
pub fn serialize_yyyy_mm_dd<S>(date: &chrono::NaiveDate, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
```

#### Function `deserialize_yyyy_mm_dd`

```rust
pub fn deserialize_yyyy_mm_dd<''de, D>(deserializer: D) -> Result<chrono::NaiveDate, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
```

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

### Functions

#### Function `get_version`

```rust
pub fn get_version() -> String { /* ... */ }
```

## Module `value`

```rust
pub mod value { /* ... */ }
```

### Types

#### Enum `AnyValue`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\", untagged)]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum AnyValue {
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<AnyValue>),
    Object(std::collections::HashMap<String, AnyValue>),
}
```

##### Variants

###### `Bool`

Fields:

| Index | Type   | Documentation |
| ----- | ------ | ------------- |
| 0     | `bool` |               |

###### `Number`

Fields:

| Index | Type     | Documentation |
| ----- | -------- | ------------- |
| 0     | `Number` |               |

###### `String`

Fields:

| Index | Type     | Documentation |
| ----- | -------- | ------------- |
| 0     | `String` |               |

###### `Array`

Fields:

| Index | Type            | Documentation |
| ----- | --------------- | ------------- |
| 0     | `Vec<AnyValue>` |               |

###### `Object`

Fields:

| Index | Type                                          | Documentation |
| ----- | --------------------------------------------- | ------------- |
| 0     | `std::collections::HashMap<String, AnyValue>` |               |

##### Implementations

###### Methods

- ```rust
  pub fn as_f64(self: &Self) -> Option<f64> { /* ... */ }
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
    fn clone(self: &Self) -> AnyValue { /* ... */ }
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
    fn from(value: String) -> AnyValue { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<String>) -> AnyValue { /* ... */ }
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
    fn eq(self: &Self, other: &AnyValue) -> bool { /* ... */ }
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

#### Enum `Number`

**Attributes:**

- `Other("#[serde(rename_all = \"camelCase\", untagged)]")`
- `Other("#[attr = CfgAttrTrace]")`

```rust
pub enum Number {
    Int(i64),
    Float(f64),
}
```

##### Variants

###### `Int`

Fields:

| Index | Type  | Documentation |
| ----- | ----- | ------------- |
| 0     | `i64` |               |

###### `Float`

Fields:

| Index | Type  | Documentation |
| ----- | ----- | ------------- |
| 0     | `f64` |               |

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
    fn clone(self: &Self) -> Number { /* ... */ }
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
    fn eq(self: &Self, other: &Number) -> bool { /* ... */ }
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
