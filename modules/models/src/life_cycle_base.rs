use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum LifeCycleStage {
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
    C1,
    C2,
    C3,
    C4,
    D,
}

impl fmt::Display for LifeCycleStage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LifeCycleStage::A1A3 => write!(f, "A1A3"),
            LifeCycleStage::A4 => write!(f, "A4"),
            LifeCycleStage::A5 => write!(f, "A5"),
            LifeCycleStage::B1 => write!(f, "B1"),
            LifeCycleStage::B2 => write!(f, "B2"),
            LifeCycleStage::B3 => write!(f, "B3"),
            LifeCycleStage::B4 => write!(f, "B4"),
            LifeCycleStage::B5 => write!(f, "B5"),
            LifeCycleStage::B6 => write!(f, "B6"),
            LifeCycleStage::B7 => write!(f, "B7"),
            LifeCycleStage::C1 => write!(f, "C1"),
            LifeCycleStage::C2 => write!(f, "C2"),
            LifeCycleStage::C3 => write!(f, "C3"),
            LifeCycleStage::C4 => write!(f, "C4"),
            LifeCycleStage::D => write!(f, "D"),
        }
    }
}

impl TryFrom<&str> for LifeCycleStage {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "a1to3" => Ok(LifeCycleStage::A1A3),
            "a1a3" => Ok(LifeCycleStage::A1A3),
            "a1-3" => Ok(LifeCycleStage::A1A3),
            "a4" => Ok(LifeCycleStage::A4),
            "a5" => Ok(LifeCycleStage::A5),
            "b1" => Ok(LifeCycleStage::B1),
            "b2" => Ok(LifeCycleStage::B2),
            "b3" => Ok(LifeCycleStage::B3),
            "b4" => Ok(LifeCycleStage::B4),
            "b5" => Ok(LifeCycleStage::B5),
            "b6" => Ok(LifeCycleStage::B6),
            "b7" => Ok(LifeCycleStage::B7),
            "c1" => Ok(LifeCycleStage::C1),
            "c2" => Ok(LifeCycleStage::C2),
            "c3" => Ok(LifeCycleStage::C3),
            "c4" => Ok(LifeCycleStage::C4),
            "d" => Ok(LifeCycleStage::D),
            value => Err(format!("Unknown lifecycle stage: '{}'", value)),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ImpactCategoryKey {
    GWP,
    #[allow(non_camel_case_types)]
    GWP_FOS,
    #[allow(non_camel_case_types)]
    GWP_BIO,
    #[allow(non_camel_case_types)]
    GWP_LUL,
    ODP,
    AP,
    EP,
    #[allow(non_camel_case_types)]
    EP_FW,
    #[allow(non_camel_case_types)]
    EP_MAR,
    #[allow(non_camel_case_types)]
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
    #[allow(non_camel_case_types)]
    ETP_FW,
    #[allow(non_camel_case_types)]
    HTP_C,
    #[allow(non_camel_case_types)]
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

impl fmt::Display for ImpactCategoryKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImpactCategoryKey::GWP => write!(f, "GWP"),
            ImpactCategoryKey::ODP => write!(f, "ODP"),
            ImpactCategoryKey::AP => write!(f, "AP"),
            ImpactCategoryKey::EP => write!(f, "EP"),
            ImpactCategoryKey::POCP => write!(f, "POCP"),
            ImpactCategoryKey::ADPE => write!(f, "ADPE"),
            ImpactCategoryKey::ADPF => write!(f, "ADPF"),
            ImpactCategoryKey::PERE => write!(f, "PERE"),
            ImpactCategoryKey::PERM => write!(f, "PERM"),
            ImpactCategoryKey::PERT => write!(f, "PERT"),
            ImpactCategoryKey::PENRT => write!(f, "PENRT"),
            ImpactCategoryKey::PENRM => write!(f, "PENRM"),
            ImpactCategoryKey::SM => write!(f, "SM"),
            ImpactCategoryKey::RSF => write!(f, "RSF"),
            ImpactCategoryKey::NRSF => write!(f, "NRSF"),
            ImpactCategoryKey::FW => write!(f, "FW"),
            ImpactCategoryKey::HWD => write!(f, "HWD"),
            ImpactCategoryKey::NHWD => write!(f, "NHWD"),
            ImpactCategoryKey::RWD => write!(f, "RWD"),
            ImpactCategoryKey::CRU => write!(f, "CRU"),
            ImpactCategoryKey::MRF => write!(f, "MRF"),
            ImpactCategoryKey::MER => write!(f, "MER"),
            ImpactCategoryKey::EEE => write!(f, "EEE"),
            ImpactCategoryKey::EET => write!(f, "EET"),
            ImpactCategoryKey::PENRE => write!(f, "PENRE"),
            ImpactCategoryKey::GWP_FOS => write!(f, "GWP_FOS"),
            ImpactCategoryKey::GWP_BIO => write!(f, "GWP_BIO"),
            ImpactCategoryKey::GWP_LUL => write!(f, "GWP_LUL"),
            ImpactCategoryKey::EP_FW => write!(f, "EP_FW"),
            ImpactCategoryKey::EP_MAR => write!(f, "EP_MAR"),
            ImpactCategoryKey::EP_TER => write!(f, "EP_TER"),
            ImpactCategoryKey::PM => write!(f, "PM"),
            ImpactCategoryKey::WDP => write!(f, "WDP"),
            ImpactCategoryKey::IRP => write!(f, "IRP"),
            ImpactCategoryKey::ETP_FW => write!(f, "ETP_FW"),
            ImpactCategoryKey::HTP_C => write!(f, "HTP_C"),
            ImpactCategoryKey::HTP_NC => write!(f, "HTP_NC"),
            ImpactCategoryKey::SQP => write!(f, "SQP"),
        }
    }
}

pub type ImpactCategory = HashMap<LifeCycleStage, Option<f64>>;

pub type Results = Option<HashMap<ImpactCategoryKey, ImpactCategory>>;
