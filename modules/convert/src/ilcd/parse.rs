use crate::ilcd::ilcd::{Anie, DataSetName, Exchange, LCIAResult, ModuleAnie, ILCD};
use chrono::NaiveDate;
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_models::epd::{Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::shared::{Conversion, Unit};
use serde_json::Error;

/// Parse a ILCD formatted EPD in an EPDx struct
///
/// # Arguments
///
/// * `json`: JSON formatted string containing the "full" EPD on ILCD format
///
/// returns: EPD
pub fn parse_ilcd(data: &str) -> Result<EPD, Error> {
    match serde_json::from_str(data) {
        Ok(ilcd_epd) => match epd_from_ilcd(ilcd_epd) {
            Ok(lcabyg_project) => Ok(lcabyg_project),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn epd_from_ilcd(ilcd_epd: ILCD) -> Result<EPD, Error> {
    let generic_anie = Anie {
        value: Some("Generic".to_string()),
        name: "subType".to_string(),
    };
    let subtype = match &ilcd_epd
        .modelling_and_validation
        .lci_method_and_allocation
        .other
    {
        Some(other) => match other.anies.iter().find(|&anie| anie.name == "subType") {
            Some(subtype) => subtype.clone(),
            None => generic_anie,
        },
        None => generic_anie,
    };

    let comment = collect_comment(&ilcd_epd);

    let mut impacts = collect_from_lcia_result(&ilcd_epd.lcia_results.lcia_result);

    let (declared_unit, conversions) =
        collect_from_exchanges(&ilcd_epd.exchanges.exchange, &mut impacts);

    Ok(EPD {
        id: ilcd_epd
            .process_information
            .data_set_information
            .uuid
            .clone(),
        name: get_name(&ilcd_epd.process_information.data_set_information.name),
        version: ilcd_epd.version.clone(),
        format_version: get_version(),
        declared_unit,
        reference_service_life: None,
        conversions: Some(conversions),
        standard: get_ilcd_standard(&ilcd_epd),
        comment,
        meta_data: None,
        source: None,
        published_date: get_date(&ilcd_epd.process_information.time.reference_year),
        valid_until: get_date(&ilcd_epd.process_information.time.data_set_valid_until),
        location: Country::from(
            ilcd_epd
                .process_information
                .geography
                .location_of_operation_supply_or_production
                .location
                .as_str(),
        ),
        subtype: SubType::from(&subtype.value),
        impacts,
    })
}

fn get_date(year: &i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(*year, 1, 1).unwrap()
}

fn impact_category_from_anies(anies: &Vec<ModuleAnie>) -> ImpactCategory {
    let mut category = ImpactCategory::new();
    let mut composite_a1a3: Option<f64> = None;

    for anie in anies {
        match (&anie.module, &anie.value) {
            (Some(module), Some(value))
                if vec!["a1", "a2", "a3"].contains(&module.to_lowercase().as_str()) =>
            {
                if composite_a1a3.is_some() {
                    match f64::try_from(value) {
                        Ok(value) => composite_a1a3 = Some(composite_a1a3.unwrap() + value),
                        Err(_) => {}
                    }
                } else {
                    match f64::try_from(value) {
                        Ok(value) => composite_a1a3 = Some(value),
                        Err(_) => {}
                    }
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("a1-a3") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::A1A3, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("a4") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::A4, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("a5") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::A5, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b1") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B1, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b2") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B2, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b3") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B3, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b4") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B4, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b5") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B5, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b6") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B6, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("b7") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::B7, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("c1") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::C1, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("c2") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::C2, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("c3") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::C3, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("c4") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::C4, Some(value));
                    }
                    Err(_) => {}
                }
            }
            (Some(module), Some(value)) if module.to_lowercase() == String::from("d") => {
                match f64::try_from(value) {
                    Ok(value) => {
                        category.insert(LifeCycleModule::D, Some(value));
                    }
                    Err(_) => {}
                }
            }
            _ => continue,
        }
    }
    if composite_a1a3.is_some() && category.get(&LifeCycleModule::A1A3).is_none() {
        category.insert(LifeCycleModule::A1A3, composite_a1a3);
    }
    category
}

fn get_ilcd_standard(helper: &ILCD) -> Standard {
    for compliance in &helper
        .modelling_and_validation
        .compliance_declarations
        .compliance
    {
        for description in &compliance.reference_to_compliance_system.short_description {
            match Standard::from(&description.value.clone().unwrap()) {
                Standard::UNKNOWN => continue,
                standard => return standard,
            }
        }
    }

    return Standard::UNKNOWN;
}

fn get_converted_unit(unit_value: &String) -> Unit {
    let value = unit_value
        .split("/")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string();
    Unit::from(&value)
}

fn get_ilcd_conversion(exchange: &Exchange) -> Vec<Conversion> {
    let mut conversions: Vec<Conversion> = vec![];

    match &exchange.material_properties {
        Some(material_properties) => {
            for material_property in material_properties {
                let value = material_property.value.parse().unwrap_or_else(|_| 1.0);
                conversions.push(Conversion {
                    value,
                    to: get_converted_unit(&material_property.unit),
                    meta_data: Some((*material_property).clone().into()),
                })
            }
        }
        _ => return conversions,
    }

    conversions
}

fn get_ilcd_declared_unit(exchange: &Exchange) -> Unit {
    for flow_property in exchange.flow_properties.as_ref().unwrap() {
        match (
            flow_property.reference_flow_property,
            &flow_property.reference_unit,
        ) {
            (Some(reference_flow), Some(reference_unit)) if reference_flow == true => {
                return Unit::from(reference_unit);
            }
            _ => continue,
        }
    }

    Unit::UNKNOWN
}

fn collect_from_lcia_result(lcia_result: &Vec<LCIAResult>) -> Impacts {
    let mut impacts = Impacts::new();

    for lcia_result in lcia_result {
        for description in &lcia_result
            .reference_to_lcia_method_dataset
            .short_description
        {
            let impact_value = impact_category_from_anies(&lcia_result.other.anies);
            match &description.value {
                Some(value) if value.contains("(GWP)") || value.contains("(GWP-total)") => {
                    impacts.insert(ImpactCategoryKey::GWP, impact_value);
                }
                Some(value) if value.contains("(GWP-fossil)") => {
                    impacts.insert(ImpactCategoryKey::GWP_FOS, impact_value);
                }
                Some(value) if value.contains("(GWP-biogenic)") => {
                    impacts.insert(ImpactCategoryKey::GWP_BIO, impact_value);
                }
                Some(value) if value.contains("(GWP-luluc)") => {
                    impacts.insert(ImpactCategoryKey::GWP_LUL, impact_value);
                }
                Some(value) if value.contains("(ODP)") => {
                    impacts.insert(ImpactCategoryKey::ODP, impact_value);
                }
                Some(value) if value.contains("(AP)") => {
                    impacts.insert(ImpactCategoryKey::AP, impact_value);
                }
                Some(value) if value.contains("(EP)") => {
                    impacts.insert(ImpactCategoryKey::EP, impact_value);
                }
                Some(value) if value.contains("(EP-freshwater)") => {
                    impacts.insert(ImpactCategoryKey::EP_FW, impact_value);
                }
                Some(value) if value.contains("(EP-marine)") => {
                    impacts.insert(ImpactCategoryKey::EP_MAR, impact_value);
                }
                Some(value) if value.contains("(EP-terrestrial)") => {
                    impacts.insert(ImpactCategoryKey::EP_TER, impact_value);
                }
                Some(value) if value.contains("(POCP)") => {
                    impacts.insert(ImpactCategoryKey::POCP, impact_value);
                }
                Some(value) if value.contains("(WDP)") => {
                    impacts.insert(ImpactCategoryKey::WDP, impact_value);
                }
                Some(value) if value.contains("(PM)") => {
                    impacts.insert(ImpactCategoryKey::PM, impact_value);
                }
                Some(value) if value.contains("(IRP)") || value.contains("(IR)") => {
                    impacts.insert(ImpactCategoryKey::IRP, impact_value);
                }
                Some(value) if value.contains("(ADPE)") => {
                    impacts.insert(ImpactCategoryKey::ADPE, impact_value);
                }
                Some(value) if value.contains("(ADPF)") => {
                    impacts.insert(ImpactCategoryKey::ADPF, impact_value);
                }
                Some(value) if value.contains("(ETP-fw)") => {
                    impacts.insert(ImpactCategoryKey::ETP_FW, impact_value);
                }
                Some(value) if value.contains("(HTP-c)") => {
                    impacts.insert(ImpactCategoryKey::HTP_C, impact_value);
                }
                Some(value) if value.contains("(HTP-nc)") => {
                    impacts.insert(ImpactCategoryKey::HTP_NC, impact_value);
                }
                Some(value) if value.contains("(SQP)") => {
                    impacts.insert(ImpactCategoryKey::SQP, impact_value);
                }
                _ => continue,
            };
        }
    }

    impacts
}

fn collect_from_exchanges(
    exchanges: &Vec<Exchange>,
    impacts: &mut Impacts,
) -> (Unit, Vec<Conversion>) {
    let mut declared_unit = Unit::UNKNOWN;
    let mut conversions: Vec<Conversion> = vec![];

    for exchange in exchanges {
        match exchange.reference_flow {
            Some(flow) if flow == true => {
                declared_unit = get_ilcd_declared_unit(exchange);
                conversions = get_ilcd_conversion(exchange);
            }
            _ => {
                for description in &exchange.reference_to_flow_data_set.short_description {
                    let impact_value = match &exchange.other {
                        Some(_anies) => impact_category_from_anies(&_anies.anies),
                        _ => continue,
                    };
                    match &description.value {
                        Some(_description) if _description.contains("(PERE)") => {
                            impacts.insert(ImpactCategoryKey::PERE, impact_value);
                        }
                        Some(_description) if _description.contains("(PERM)") => {
                            impacts.insert(ImpactCategoryKey::PERM, impact_value);
                        }
                        Some(_description) if _description.contains("(PERT)") => {
                            impacts.insert(ImpactCategoryKey::PERT, impact_value);
                        }
                        Some(_description) if _description.contains("(PENRE)") => {
                            impacts.insert(ImpactCategoryKey::PENRE, impact_value);
                        }
                        Some(_description) if _description.contains("(PENRM)") => {
                            impacts.insert(ImpactCategoryKey::PENRM, impact_value);
                        }
                        Some(_description) if _description.contains("(PENRT)") => {
                            impacts.insert(ImpactCategoryKey::PENRT, impact_value);
                        }
                        Some(_description) if _description.contains("(SM)") => {
                            impacts.insert(ImpactCategoryKey::SM, impact_value);
                        }
                        Some(_description) if _description.contains("(RSF)") => {
                            impacts.insert(ImpactCategoryKey::RSF, impact_value);
                        }
                        Some(_description) if _description.contains("(NRSF)") => {
                            impacts.insert(ImpactCategoryKey::NRSF, impact_value);
                        }
                        Some(_description) if _description.contains("(FW)") => {
                            impacts.insert(ImpactCategoryKey::FW, impact_value);
                        }
                        Some(_description) if _description.contains("(HWD)") => {
                            impacts.insert(ImpactCategoryKey::HWD, impact_value);
                        }
                        Some(_description) if _description.contains("(NHWD)") => {
                            impacts.insert(ImpactCategoryKey::NHWD, impact_value);
                        }
                        Some(_description) if _description.contains("(RWD)") => {
                            impacts.insert(ImpactCategoryKey::RWD, impact_value);
                        }
                        Some(_description) if _description.contains("(CRU)") => {
                            impacts.insert(ImpactCategoryKey::CRU, impact_value);
                        }
                        Some(_description) if _description.contains("(MFR)") => {
                            impacts.insert(ImpactCategoryKey::MRF, impact_value);
                        }
                        Some(_description) if _description.contains("(MER)") => {
                            impacts.insert(ImpactCategoryKey::MER, impact_value);
                        }
                        Some(_description) if _description.contains("(EEE)") => {
                            impacts.insert(ImpactCategoryKey::EEE, impact_value);
                        }
                        Some(_description) if _description.contains("(EET)") => {
                            impacts.insert(ImpactCategoryKey::EET, impact_value);
                        }
                        _ => continue,
                    }
                }
            }
        };
    }

    (declared_unit, conversions)
}

fn get_name(base_name: &DataSetName) -> String {
    match base_name.base_name.first() {
        Some(name) if name.value.is_some() => name.value.clone().unwrap(),
        _ => "".to_string(),
    }
}

fn collect_comment(epd: &ILCD) -> Option<String> {
    let mut comment = None;
    if let Some(general_comments) = &epd.process_information.data_set_information.general_comment {
        comment = match general_comments
            .iter()
            .find(|comment| comment.lang == "en".to_string())
        {
            Some(comment) => comment.value.clone(),
            None if general_comments.len() > 0 => general_comments[0].value.clone(),
            None => None,
        };
    }
    if comment.is_none() {
        if let Some(technology) = &epd.process_information.technology {
            comment = match technology
                .description
                .iter()
                .find(|comment| comment.lang == "en".to_string())
            {
                Some(comment) => comment.value.clone(),
                None if technology.description.len() > 0 => technology.description[0].value.clone(),
                None => None,
            };
        }
    }

    comment
}
