use core::any::TypeId;
use field_access::FieldAccess;
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_models::assembly::{Assembly, Classification};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, LifeCycleStage};
use lcax_models::product::{ImpactDataSource, Product};
use lcax_models::project::{Location, Project as LCAxProject, SoftwareInfo};
use lcax_models::shared::Unit;
use lcax_models::techflow::TechFlow;
use parquet::record::{Row, RowAccessor};
use std::collections::HashMap;
use uuid;

#[derive(Default, FieldAccess)]
pub struct SLiCEElement {
    stock_region_name: String,
    stock_region_code: String,
    building_use_subtype_name: String,
    building_use_subtype_code: String,
    stock_activity_type_name: String,
    stock_activity_type_code: String,
    building_energy_performance_name: String,
    building_energy_performance_code: String,
    building_archetype_code: String,
    element_class_generic_name: String,
    element_class_sfb: String,
    worksection_class_sfb: String,
    techflow_name_mmg: String,
    material_name_mmg: String,
    //material_name_JRC_CDW: String,
    material_name_jrc_cdw: String,
    //material_category_Sturm: String,
    material_category_sturm: String,
    //material_category_Sturm_upd: String,
    material_category_sturm_upd: String,
    amount_material_kg_per_building: f64,
    activity_in_out: String,
    activity_year: i64,
    //LCS_EN15978: String,
    lcs_en15978: String,
    //ind_GWP_Tot: f64,
    ind_gwp_tot: f64,
    //ind_GWP_Fos: f64,
    ind_gwp_fos: f64,
    //ind_GWP_Bio: f64,
    ind_gwp_bio: f64,
    //ind_GWP_LuLuc: f64,
    ind_gwp_luluc: f64,
    // ind_ODP: f64,
    ind_odp: f64,
    // ind_AP: f64,
    ind_ap: f64,
    // ind_EP_Fw: f64,
    ind_ep_fw: f64,
    // ind_EP_Mar: f64,
    ind_ep_mar: f64,
    // ind_EP_Ter: f64,
    ind_ep_ter: f64,
    // ind_PCOP: f64,
    ind_pcop: f64,
    // ind_ADP_MiMe: f64,
    ind_adp_mime: f64,
    // ind_ADP_Fos: f64,
    ind_adp_fos: f64,
    // ind_WDP: f64,
    ind_wdp: f64,
    // ind_PM: f64,
    ind_pm: f64,
    // ind_IRP: f64,
    ind_irp: f64,
    // ind_ETP_Fw: f64,
    ind_etp_fw: f64,
    // ind_HTP_c: f64,
    ind_htp_c: f64,
    // ind_HTP_nc: f64,
    ind_htp_nc: f64,
    // ind_SQP: f64,
    ind_sqp: f64,
    // ind_GWP_EN15804+A1: f64
    ind_gwp_en15804_a1: f64,
}

impl From<(&Row, &Vec<&str>)> for SLiCEElement {
    fn from(element: (&Row, &Vec<&str>)) -> Self {
        let (row, fields) = element;
        let mut slice = SLiCEElement::default();
        let mut field_iter_count = 0;
        for _field in fields {
            let mut field = slice.field_mut(&_field.to_lowercase()).unwrap();
            if field.type_id() == TypeId::of::<String>() {
                match row.get_string(field_iter_count) {
                    Ok(value) => {
                        field.replace(value.clone());
                    }
                    Err(_) => continue, //Err(_) => println!("Error getting string value for {_field}"),
                };
            } else if field.type_id() == TypeId::of::<f64>() {
                match row.get_double(field_iter_count) {
                    Ok(value) => {
                        field.replace(value.clone());
                    }
                    Err(_) => continue, //Err(_) => println!("Error getting f64 value for {_field}"),
                };
            } else if field.type_id() == TypeId::of::<i64>() {
                match row.get_long(field_iter_count) {
                    Ok(value) => {
                        field.replace(value.clone());
                    }
                    Err(_) => continue, //Err(_) => println!("Error getting i64 value for {_field}"),
                };
            }
            field_iter_count += 1;
        }
        slice
    }
}

pub fn add_project_data(project: &mut LCAxProject, element: &SLiCEElement) {
    project.id = uuid::Uuid::new_v4().to_string();
    project.name = element.building_archetype_code.clone();

    let stock_region_name = element.stock_region_name.clone();
    let building_use_subtype_name = element.building_use_subtype_name.clone();
    let stock_activity_type_name = element.stock_activity_type_name.clone();
    let building_energy_performance_name = element.building_energy_performance_name.clone();
    project.description = Some(format!("{stock_region_name}-{building_use_subtype_name}-{stock_activity_type_name}-{building_energy_performance_name}"));

    project.location = Location {
        country: get_country_from_region(&element.stock_region_code),
        city: None,
        address: None,
    };
    project.impact_categories = vec![
        ImpactCategoryKey::AP,
        ImpactCategoryKey::ADPE,
        ImpactCategoryKey::ADPF,
        ImpactCategoryKey::EP,
        ImpactCategoryKey::POCP,
        ImpactCategoryKey::ODP,
        ImpactCategoryKey::GWP,
        ImpactCategoryKey::PENRE,
        ImpactCategoryKey::PERE,
    ];
    project.life_cycle_stages = vec![
        LifeCycleStage::A1A3,
        LifeCycleStage::A4,
        LifeCycleStage::A5,
        LifeCycleStage::B2,
        LifeCycleStage::B4,
        LifeCycleStage::B5,
        LifeCycleStage::B6,
        LifeCycleStage::C1,
        LifeCycleStage::C2,
        LifeCycleStage::C3,
        LifeCycleStage::C4,
    ];
    project.owner = None;
    project.format_version = get_version();
    project.classification_system = Some(String::from("SfB"));
    project.software_info = SoftwareInfo {
        goal_and_scope_definition: None,
        lca_software: String::from("SLiCE"),
        calculation_type: None,
    }
}

fn get_country_from_region(region: &str) -> Country {
    match region.to_lowercase().as_str() {
        "continental" => Country::DEU,
        "mediterranean" => Country::ITA,
        "nordic" => Country::SWE,
        "oceanic" => Country::GBR,
        _ => Country::DEU,
    }
}

pub fn add_slice_element(project: &mut LCAxProject, element: &SLiCEElement) {
    let assembly_uuid = get_uuid(&element.element_class_sfb);
    let worksection_class_sfb = element.worksection_class_sfb.clone();
    let techflow_name_mmg = element.techflow_name_mmg.clone();
    let product_uuid = get_uuid(&format!("{worksection_class_sfb}-{techflow_name_mmg}"));

    if !project.assemblies.contains_key(&assembly_uuid) {
        let mut assembly = assembly_from_slice(assembly_uuid.as_str(), element);
        let product = product_from_slice(product_uuid.as_str(), element);
        assembly.products.insert(product_uuid.clone(), product);
        project.assemblies.insert(assembly_uuid.clone(), assembly);
    } else {
        let assembly = project.assemblies.get_mut(&assembly_uuid).unwrap();
        if !assembly.products.contains_key(&product_uuid) {
            let product = product_from_slice(product_uuid.as_str(), element);
            assembly.products.insert(product_uuid.clone(), product);
        } else {
            let product = assembly.products.get_mut(&product_uuid).unwrap();
            match product.impact_data {
                ImpactDataSource::TechFlow(ref mut tech_flow) => {
                    add_impact_data(tech_flow, element);
                }
                _ => {}
            }
        }
    }
}

fn get_uuid(name: &str) -> String {
    let uid = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, name.as_bytes());
    uid.to_string()
}

fn assembly_from_slice(uid: &str, element: &SLiCEElement) -> Assembly {
    Assembly {
        id: uid.to_string(),
        name: element.element_class_sfb.clone(),
        description: element.element_class_generic_name.clone(),
        comment: None,
        quantity: 1.0,
        unit: Unit::KG,
        category: None,
        classification: Some(vec![Classification {
            system: "SfB".to_string(),
            code: element.element_class_sfb.clone(),
            name: element.element_class_generic_name.clone(),
        }]),
        products: Default::default(),
        results: None,
        meta_data: None,
    }
}

fn product_from_slice(uid: &str, element: &SLiCEElement) -> Product {
    Product {
        id: uid.to_string(),
        name: element.worksection_class_sfb.clone(),
        description: "".to_string(),
        reference_service_life: 50,
        impact_data: ImpactDataSource::TechFlow(create_tech_flow(element)),
        quantity: element.amount_material_kg_per_building,
        unit: Unit::KG,
        transport: None,
        results: None,
        meta_data: None,
    }
}

fn create_tech_flow(element: &SLiCEElement) -> TechFlow {
    let impacts = match LifeCycleStage::try_from(element.lcs_en15978.as_str()) {
        Err(_) => HashMap::new(),
        Ok(stage) => HashMap::from([
            (
                ImpactCategoryKey::GWP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_gwp_tot))]),
            ),
            (
                ImpactCategoryKey::GWP_BIO,
                ImpactCategory::from([(stage.clone(), Some(element.ind_gwp_bio))]),
            ),
            (
                ImpactCategoryKey::GWP_LUL,
                ImpactCategory::from([(stage.clone(), Some(element.ind_gwp_luluc))]),
            ),
            (
                ImpactCategoryKey::ODP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_odp))]),
            ),
            (
                ImpactCategoryKey::AP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_ap))]),
            ),
            (
                ImpactCategoryKey::EP_FW,
                ImpactCategory::from([(stage.clone(), Some(element.ind_ep_fw))]),
            ),
            (
                ImpactCategoryKey::EP_MAR,
                ImpactCategory::from([(stage.clone(), Some(element.ind_ep_mar))]),
            ),
            (
                ImpactCategoryKey::EP_TER,
                ImpactCategory::from([(stage.clone(), Some(element.ind_ep_ter))]),
            ),
            (
                ImpactCategoryKey::POCP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_pcop))]),
            ),
            (
                ImpactCategoryKey::WDP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_wdp))]),
            ),
            (
                ImpactCategoryKey::PM,
                ImpactCategory::from([(stage.clone(), Some(element.ind_pm))]),
            ),
            (
                ImpactCategoryKey::IRP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_irp))]),
            ),
            (
                ImpactCategoryKey::ETP_FW,
                ImpactCategory::from([(stage.clone(), Some(element.ind_etp_fw))]),
            ),
            (
                ImpactCategoryKey::HTP_C,
                ImpactCategory::from([(stage.clone(), Some(element.ind_htp_c))]),
            ),
            (
                ImpactCategoryKey::HTP_NC,
                ImpactCategory::from([(stage.clone(), Some(element.ind_htp_nc))]),
            ),
            (
                ImpactCategoryKey::SQP,
                ImpactCategory::from([(stage.clone(), Some(element.ind_sqp))]),
            ),
        ]),
    };
    TechFlow {
        id: get_uuid(&element.techflow_name_mmg),
        name: element.techflow_name_mmg.clone(),
        declared_unit: Unit::KG,
        format_version: get_version(),
        source: None,
        comment: None,
        location: Country::UNKNOWN,
        conversions: None,
        impacts,
        meta_data: None,
    }
}

fn add_impact_data(tech_flow: &mut TechFlow, element: &SLiCEElement) {
    match LifeCycleStage::try_from(element.lcs_en15978.as_str()) {
        Err(stage) => println!("{stage} not a valid life cycle stage."),
        Ok(stage) => {
            for (category_key, category_value) in tech_flow.impacts.iter_mut() {
                let impact_value = match category_key {
                    ImpactCategoryKey::GWP => element.ind_gwp_tot,
                    ImpactCategoryKey::GWP_BIO => element.ind_gwp_bio,
                    ImpactCategoryKey::GWP_LUL => element.ind_gwp_luluc,
                    ImpactCategoryKey::ODP => element.ind_odp,
                    ImpactCategoryKey::EP_FW => element.ind_ep_fw,
                    ImpactCategoryKey::EP_MAR => element.ind_ep_mar,
                    ImpactCategoryKey::EP_TER => element.ind_ep_ter,
                    ImpactCategoryKey::POCP => element.ind_pcop,
                    ImpactCategoryKey::WDP => element.ind_wdp,
                    ImpactCategoryKey::PM => element.ind_pm,
                    ImpactCategoryKey::IRP => element.ind_irp,
                    ImpactCategoryKey::ETP_FW => element.ind_etp_fw,
                    ImpactCategoryKey::HTP_C => element.ind_htp_c,
                    ImpactCategoryKey::HTP_NC => element.ind_htp_nc,
                    ImpactCategoryKey::SQP => element.ind_sqp,
                    _ => continue,
                };
                match category_value.get(&stage) {
                    None => {
                        category_value.insert(stage.clone(), Some(impact_value));
                    }
                    Some(impact_category_value) => {
                        category_value.insert(
                            stage.clone(),
                            Some(impact_category_value.unwrap() + impact_value),
                        );
                    }
                }
            }
        }
    }
}
