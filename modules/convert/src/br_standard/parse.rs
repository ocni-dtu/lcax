use crate::br_standard::br18_generic_data::{
    get_district_heating_data, get_electricity_data, get_energy_data, get_lng_data,
};
use crate::br_standard::models::{BRComponent, BROperation, BRProjectInfo};
use chrono::{DateTime, Datelike, Utc};
use lcax_calculation::calculate::{calculate_project, CalculationOptions};
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_core::value::AnyValue;
use lcax_models::assembly::{Assembly, AssemblyReference, Classification};
use lcax_models::generic_impact_data::{GenericData, GenericDataReference};
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use lcax_models::product::{ImpactData, Product, ProductReference};
use lcax_models::project::{
    AreaType, BuildingType, BuildingTypology, GeneralEnergyClass, Location, Project,
    BuildingInfo, ProjectPhase, RoofType, SoftwareInfo,
};
use lcax_models::shared::{Source, Unit};
use std::collections::HashMap;
use uuid::Uuid;

/// Parse data from BR Standard Format in to a LCAx Project
///
/// # Arguments
///
/// * `project_name`: Project Name
/// * `project_info`: Project info from the "General information" tab
/// * `components`: Component info from the "BR18 vejledning" tab
/// * `operations`: Operation info from the "Drift" tab
///
/// returns: Result<LCAxProject, String>
pub fn parse_br_standard(
    project_name: &str,
    project_info: &BRProjectInfo,
    components: &Vec<BRComponent>,
    operations: &Vec<BROperation>,
) -> Result<Project, String> {
    Ok(Project::try_from_br((
        project_name,
        project_info,
        components,
        operations,
    ))?)
}

trait FromBR<T> {
    fn from_br(_: T) -> Self;
}

trait TryFromBR<T> {
    fn try_from_br(_: T) -> Result<Self, String>
    where
        Self: Sized;
}

trait FromBROperation {
    fn from_br_operation(_: &BROperation, _: &u16) -> Self;
}

impl TryFromBR<(&str, &BRProjectInfo, &Vec<BRComponent>, &Vec<BROperation>)> for Project {
    fn try_from_br(
        (project_name, project_info, components, operations): (
            &str,
            &BRProjectInfo,
            &Vec<BRComponent>,
            &Vec<BROperation>,
        ),
    ) -> Result<Self, String> {
        let mut project = Self {
            id: Uuid::new_v4().to_string(),
            name: project_name.to_string(),
            description: None,
            comment: None,
            location: Location {
                country: Country::DNK,
                address: Some(project_info.address.clone()),
                city: None,
            },
            owner: None,
            format_version: get_version(),
            lcia_method: None,
            classification_systems: Some(vec!["BR18".to_string()]),
            reference_study_period: Some(50),
            life_cycle_modules: vec![],
            impact_categories: vec![],
            assemblies: construct_assemblies(
                components,
                operations,
                &project_info.heated_area,
                &date_to_year(&project_info.building_operation_date)
                    .unwrap_or(Utc::now().year() as u16),
            )
            .iter()
            .map(|assembly| AssemblyReference::Assembly(assembly.to_owned()))
            .collect(),
            results: None,
            project_info: Some(BuildingInfo {
                    building_type: BuildingType::from_br(&project_info.building_type),
                    building_typology: vec![BuildingTypology::from_br(&project_info.typology)],
                    certifications: None,
                    building_mass: None,
                    building_height: None,
                    gross_floor_area: Some(AreaType {
                        value: project_info.gross_floor_area,
                        unit: Unit::M2,
                        definition: "".to_string(),
                    }),
                    heated_floor_area: Some(AreaType {
                        value: project_info.heated_area,
                        unit: Unit::M2,
                        definition: "".to_string(),
                    }),
                    building_footprint: None,
                    floors_above_ground: project_info.floors_above_ground,
                    floors_below_ground: Some(project_info.floors_below_ground),
                    roof_type: RoofType::UNKNOWN,
                    frame_type: None,
                    building_completion_year: date_to_year(&project_info.building_operation_date),
                    building_permit_year: date_to_year(&project_info.building_permission_date),
                    energy_demand_heating: Some(project_info.heating_no_modifiers.clone()),
                    energy_supply_heating: None,
                    energy_demand_electricity: Some(project_info.electricity_no_modifiers.clone()),
                    energy_supply_electricity: None,
                    exported_electricity: Some(project_info.electricity_production.clone()),
                    general_energy_class: GeneralEnergyClass::STANDARD,
                    local_energy_class: project_info.energy_class.clone(),
                    building_users: None,
                    building_model_scope: None,
            }),
            project_phase: ProjectPhase::TECHNICAL_DESIGN,
            software_info: SoftwareInfo {
                lca_software: project_info.lca_software.clone(),
                lca_software_version: Some(project_info.lca_version.clone()),
                goal_and_scope_definition: None,
                calculation_type: Some(project_info.building_regulation.clone()),
            },
            meta_data: Some(HashMap::from([(
                "convertedAt".to_string(),
                AnyValue::String(format!("{}", Utc::now().naive_utc().format("%Y-%m-%d"))),
            )])),
        };
        let calc_options = CalculationOptions {
            reference_study_period: Some(50),
            life_cycle_modules: vec![
                LifeCycleModule::A1A3,
                LifeCycleModule::B4,
                LifeCycleModule::B6,
                LifeCycleModule::C3,
                LifeCycleModule::C4,
            ],
            impact_categories: vec![ImpactCategoryKey::GWP],
            overwrite_existing_results: false,
        };
        calculate_project(&mut project, Some(calc_options))?;
        project.resolve_impact_categories();
        project.resolve_life_cycle_stages();
        Ok(project)
    }
}

impl FromBR<&str> for BuildingTypology {
    fn from_br(typology: &str) -> Self {
        if typology.is_empty() {
            return Self::UNKNOWN;
        }
        let _slice = &typology[0..3];
        match _slice {
            "110" | "120" | "121" | "122" | "130" | "131" | "132" | "140" | "150" | "160"
            | "185" | "190" => Self::RESIDENTIAL,
            "210" | "211" | "212" | "213" | "214" | "215" | "216" | "217" | "218" | "219" => {
                Self::AGRICULTURAL
            }
            "220" | "221" | "222" | "223" | "229" | "290" => Self::INDUSTRIAL,
            "230" | "231" | "232" | "233" | "234" | "239" => Self::INFRASTRUCTURE,
            "310" | "311" | "312" | "313" | "314" | "315" | "319" => Self::INFRASTRUCTURE,
            "320" | "321" => Self::OFFICE,
            "322" | "323" | "324" | "325" | "329" | "330" | "331" | "332" | "333" | "334"
            | "339" => Self::COMMERCIAL,
            "410" | "411" | "412" | "413" | "414" | "415" | "416" | "419" => Self::PUBLIC,
            "420" | "421" | "422" | "429" => Self::EDUCATIONAL,
            "430" | "431" | "432" | "433" | "439" => Self::HEALTH,
            "440" | "441" | "442" | "443" | "444" | "449" => Self::PUBLIC,
            "530" | "531" | "532" | "533" | "534" | "535" | "539" => Self::OTHER,
            "510" | "520" | "521" | "522" | "523" | "529" | "540" | "585" | "590" => Self::OTHER,
            "910" | "920" | "930" | "940" | "950" | "960" | "970" | "990" => Self::OTHER,
            "999" => Self::UNKNOWN,
            _ => panic!("Unknown typology: {typology}"),
        }
    }
}

impl FromBR<&str> for BuildingType {
    fn from_br(_type: &str) -> Self {
        match _type {
            "Nybyggeri" => Self::NEW_CONSTRUCTION_WORKS,
            "Tilbygning" => Self::EXTENSION_WORKS,
            "Transformation" => Self::RETROFIT_WORKS,
            "Renovering" => Self::RETROFIT_AND_EXTENSION_WORKS,
            "" => Self::UNKNOWN,
            _ => panic!("Unknown type: {_type}"),
        }
    }
}

fn date_to_year(date: &Option<String>) -> Option<u16> {
    match date {
        Some(date) if !date.is_empty() => {
            if date == "dd-mm-åååå" {
                return None;
            }
            let time = DateTime::parse_from_str(date, "%Y-%m-%d");
            Some(time.unwrap().year() as u16)
        }
        _ => None,
    }
}

fn construct_assemblies(
    components: &Vec<BRComponent>,
    operations: &Vec<BROperation>,
    heated_floor_area: &f64,
    project_completion_year: &u16,
) -> Vec<Assembly> {
    let mut assemblies = vec![];

    assemblies.push(Assembly {
        id: Uuid::new_v4().to_string(),
        name: "Drift".to_string(),
        description: None,
        comment: None,
        unit: Unit::M2,
        quantity: heated_floor_area.clone(),
        classification: Some(vec![Classification {
            system: "BR18".to_string(),
            code: "".to_string(),
            name: "Drift".to_string(),
        }]),
        products: construct_operation_products(operations, project_completion_year)
            .iter()
            .map(|product| ProductReference::Product(product.to_owned()))
            .collect(),
        results: None,
        meta_data: None,
    });
    add_components(components, &mut assemblies);
    assemblies
}

fn construct_operation_products(
    operations: &Vec<BROperation>,
    project_completion_year: &u16,
) -> Vec<Product> {
    let mut products = vec![];
    for operation in operations {
        products.push(Product::from_br_operation(
            operation,
            project_completion_year,
        ))
    }
    products
}

impl FromBROperation for Product {
    fn from_br_operation(operation: &BROperation, project_completion_year: &u16) -> Self {
        match operation.product.as_str() {
            "El" | "El - Fremskrivning" => Self {
                id: Uuid::new_v4().to_string(),
                name: "Electricity".to_string(),
                description: Some("Impact data should be linearly interpolated".to_string()),
                reference_service_life: 50,
                impact_data: get_energy_data(project_completion_year, get_electricity_data())
                    .iter()
                    .map(|impact| {
                        ImpactData::GenericData(GenericDataReference::GenericData(
                            impact.to_owned(),
                        ))
                    })
                    .collect(),
                quantity: operation.quantity,
                unit: Unit::KWH,
                transport: None,
                results: Some(operation.results.clone()),
                meta_data: None,
            },
            "Fjernvarme" | "Fjernvarme - Fremskrivning" => Self {
                id: Uuid::new_v4().to_string(),
                name: "Heating".to_string(),
                description: Some("Impact data should be linearly interpolated".to_string()),
                reference_service_life: 50,
                impact_data: get_energy_data(project_completion_year, get_district_heating_data())
                    .iter()
                    .map(|impact| {
                        ImpactData::GenericData(GenericDataReference::GenericData(
                            impact.to_owned(),
                        ))
                    })
                    .collect(),
                quantity: operation.quantity,
                unit: Unit::KWH,
                transport: None,
                results: Some(operation.results.clone()),
                meta_data: None,
            },
            "Ledningsgas" => Self {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Gas".to_string(),
                description: Some("Impact data should be linearly interpolated".to_string()),
                reference_service_life: 50,
                impact_data: get_energy_data(project_completion_year, get_lng_data())
                    .iter()
                    .map(|impact| {
                        ImpactData::GenericData(GenericDataReference::GenericData(
                            impact.to_owned(),
                        ))
                    })
                    .collect(),
                quantity: operation.quantity,
                unit: Unit::KWH,
                transport: None,
                results: Some(operation.results.clone()),
                meta_data: None,
            },
            "Andet energi" => {
                todo!()
            }
            _else => panic!("Unknown operation type: {_else}"),
        }
    }
}

fn add_components(components: &Vec<BRComponent>, assemblies: &mut Vec<Assembly>) {
    let mut map_assemblies: HashMap<(String, String), Assembly> = HashMap::new();

    for component in components {
        match map_assemblies.get_mut(&(
            component.building_part_sub.to_owned(),
            component.construction.to_owned(),
        )) {
            Some(assembly) => {
                assembly
                    .products
                    .push(ProductReference::Product(Product::from_br(component)));
            }
            None => {
                map_assemblies.insert(
                    (
                        component.building_part_sub.to_owned(),
                        component.construction.to_owned(),
                    ),
                    Assembly::from_br(&component),
                );
            }
        }
    }
    map_assemblies
        .values()
        .for_each(|assembly| assemblies.push(assembly.to_owned()))
}

impl FromBR<&BRComponent> for Assembly {
    fn from_br(component: &BRComponent) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: component.building_part_sub.to_owned(),
            description: None,
            comment: None,
            quantity: 1.0,
            unit: Unit::PCS,
            classification: Some(vec![Classification {
                system: "BR18".to_string(),
                code: "".to_string(),
                name: component.building_part_sub.to_owned(),
            }]),
            products: vec![ProductReference::Product(Product::from_br(component))],
            results: None,
            meta_data: None,
        }
    }
}

impl FromBR<&BRComponent> for Product {
    fn from_br(component: &BRComponent) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: component.construction.to_owned(),
            description: None,
            reference_service_life: component.reference_service_life as u32,
            impact_data: vec![ImpactData::GenericData(GenericDataReference::GenericData(
                GenericData::from_br(component),
            ))],
            quantity: component.computed_quantity,
            unit: Unit::from(&component.computed_unit),
            transport: None,
            results: Some(component.results.clone()),
            meta_data: None,
        }
    }
}

impl FromBR<&BRComponent> for GenericData {
    fn from_br(component: &BRComponent) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: component.product.to_owned(),
            declared_unit: Unit::from(&component.computed_unit),
            format_version: get_version(),
            source: Some(Source {
                name: component
                    .environmental_data
                    .link
                    .replace("http://", "")
                    .replace("https://", "")
                    .replace("www.", "")
                    .split(".")
                    .collect::<Vec<&str>>()[0]
                    .to_string(),
                url: Some(component.environmental_data.link.to_owned()),
            }),
            comment: None,
            conversions: None,
            impacts: Default::default(),
            meta_data: None,
        }
    }
}
