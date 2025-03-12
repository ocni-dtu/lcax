use std::collections::HashMap;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use crate::br_standard::models::{BRComponents, BROperation, BRProjectInfo};
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_core::value::AnyValue;
use lcax_models::assembly::{Assembly, AssemblyReference};
use lcax_models::project::{AreaType, BuildingInfo, BuildingType, BuildingTypology, GeneralEnergyClass, Location, Project, ProjectInfo, ProjectPhase, RoofType, SoftwareInfo};
use lcax_models::shared::Unit;

/// Parse an LCAByg project exported as json files.
///
/// # Arguments
///
/// * `project_data`: JSON formatted string containing the LCAbyg project data
/// * `result_data`: Optional JSON formatted string containing the result data from the LCAByg project
///
/// returns: LCAxProject
pub fn parse_br_standard(
    project_info: &BRProjectInfo,
    components: &Vec<BRComponents>,
    operations: &Vec<BROperation>,
) -> Project {
    Project::from_br((project_info, components, operations))
}

trait FromBR<T> {
    fn from_br(_: T) -> Self;
}

impl FromBR<(&BRProjectInfo, &Vec<BRComponents>, &Vec<BROperation>)> for Project {
    fn from_br(
        (project_info, components, operations): (
            &BRProjectInfo,
            &Vec<BRComponents>,
            &Vec<BROperation>,
        ),
    ) -> Self {
        let mut project = Self {
            id: "".to_string(),
            name: "".to_string(),
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
            life_cycle_stages: vec![],
            impact_categories: vec![],
            assemblies: construct_assemblies(
                components,
                operations,
            )
                .iter()
                .map(|assembly| AssemblyReference::Assembly(assembly.to_owned()))
                .collect(),
            results: None,
            project_info: Some(ProjectInfo::BuildingInfo { 0: BuildingInfo {
                building_type: BuildingType::from_br(&project_info.building_type),
                building_typology: vec![BuildingTypology::from_br(&project_info.typology)],
                certifications: None,
                building_mass: None,
                building_height: None,
                gross_floor_area: Some(AreaType {
                  value: project_info.gross_floor_area,
                    unit: Unit::M2,
                    definition: "".to_string()
                }),
                heated_floor_area: Some(AreaType {
                    value: project_info.heated_area,
                    unit: Unit::M2,
                    definition: "".to_string()
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
            } }),
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
        project.resolve_impact_categories();
        project.resolve_life_cycle_stages();
        project
    }
}

impl FromBR<&str> for BuildingTypology {
    fn from_br(typology: &str) -> Self {
        unimplemented!();
    }
}

impl FromBR<&str> for BuildingType {
    fn from_br(_type: &str) -> Self {
        unimplemented!();
    }
}

fn date_to_year(date: &Option<String>) -> Option<u16> {
    match date {
        None => None,
        Some(date) => {
            let time = DateTime::parse_from_str(date, "%Y-%m-%d");
            Some(time.unwrap().year() as u16)
        }
    }
}

fn construct_assemblies(components: &Vec<BRComponents>, operations: &Vec<BROperation>,) -> Vec<Assembly> {
    unimplemented!();
}