use crate::br_standard::models::{BRComponent, BREnvironmentalData, BROperation, BRProjectInfo};
use crate::br_standard::parse::parse_br_standard;
use crate::br_standard::translations::GENERAL_INFORMATION_TRANSLATION;
use calamine::{open_workbook, open_workbook_from_rs, Data, DataType, Error, Reader, Xlsx};
use lcax_models::life_cycle_base::{ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::project::Project;
use std::collections::HashMap;
use std::io::{Cursor, Seek};
use std::path::PathBuf;

pub fn br_standard_from_file(file_path: PathBuf) -> Result<Project, String> {
    let (project_info, components, operations) = read_br_standard_from_file(&file_path).unwrap();
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    Ok(parse_br_standard(
        file_name,
        &project_info,
        &components,
        &operations,
    )?)
}

pub fn read_br_standard<T>(
    workbook: &mut Xlsx<T>,
) -> Result<(BRProjectInfo, Vec<BRComponent>, Vec<BROperation>), Error>
where
    T: Seek,
    T: std::io::Read,
{
    let project_info = read_project_info(workbook)?;
    let components = read_components(workbook, &project_info.gross_floor_area, &50.0)?;
    let operations = read_operations(workbook, &50.0)?;
    Ok((project_info, components, operations))
}

pub fn read_br_standard_from_file(
    file_path: &PathBuf,
) -> Result<(BRProjectInfo, Vec<BRComponent>, Vec<BROperation>), Error> {
    let mut workbook: Xlsx<_> = open_workbook(file_path)?;
    Ok(read_br_standard(&mut workbook)?)
}

pub fn read_br_standard_from_bytes(
    file: Vec<u8>,
) -> Result<(BRProjectInfo, Vec<BRComponent>, Vec<BROperation>), Error> {
    let buffer: Cursor<Vec<u8>> = Cursor::new(file);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(buffer)?;
    Ok(read_br_standard(&mut workbook)?)
}

fn read_project_info<T>(workbook: &mut Xlsx<T>) -> Result<BRProjectInfo, Error>
where
    T: Seek,
    T: std::io::Read,
{
    let mut project_info_map: HashMap<&str, Data> = HashMap::new();

    for name in vec!["General information", "Generel information"] {
        match workbook.worksheet_range(name) {
            Ok(range) => {
                for row in range.rows() {
                    match &row[0] {
                        Data::String(name) => {
                            match GENERAL_INFORMATION_TRANSLATION.get(&name) {
                                Some(key) => project_info_map.insert(*key, row[1].clone()),
                                None => continue,
                            };
                        }
                        _ => continue,
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(BRProjectInfo {
        typology: project_info_map
            .get("typology")
            .unwrap_or(&Data::String("unknown".to_string()))
            .to_string(),
        building_type: project_info_map
            .get("building_type")
            .unwrap_or(&Data::String("unknown".to_string()))
            .to_string(),
        floors_above_ground: project_info_map
            .get("floors_above_ground")
            .unwrap()
            .as_f64()
            .unwrap() as u16,
        floors_below_ground: project_info_map
            .get("floors_below_ground")
            .unwrap()
            .as_f64()
            .unwrap() as u16,
        address: project_info_map
            .get("address")
            .unwrap_or(&Data::String("unknown".to_string()))
            .to_string(),
        building_permission_date: Some(
            project_info_map
                .get("building_permission_date")
                .unwrap()
                .to_string(),
        ),
        building_operation_date: Some(
            project_info_map
                .get("building_operation_date")
                .unwrap()
                .to_string(),
        ),
        building_regulation: project_info_map
            .get("building_regulation")
            .unwrap()
            .to_string(),
        threshold_new_constructions: project_info_map
            .get("threshold_new_constructions")
            .unwrap()
            .as_f64()
            .unwrap(),
        threshold_low_emission: project_info_map
            .get("threshold_low_emission")
            .unwrap()
            .as_f64()
            .unwrap(),
        total_emissions: project_info_map
            .get("total_emissions")
            .unwrap()
            .as_f64()
            .unwrap(),
        emission_modifiers: project_info_map
            .get("emission_modifiers")
            .unwrap()
            .as_f64()
            .unwrap(),
        emission_all_modules: project_info_map
            .get("emission_all_modules")
            .unwrap()
            .as_f64()
            .unwrap(),
        emission_module_d: project_info_map
            .get("emission_module_d")
            .unwrap()
            .as_f64()
            .unwrap(),
        gross_floor_area: project_info_map
            .get("gross_floor_area")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        basement_area: project_info_map.get("basement_area").unwrap().as_f64(),
        staircase_area: project_info_map
            .get("staircase_area")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        garage_area: project_info_map
            .get("garage_area")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        carport_area: project_info_map.get("carport_area").unwrap().as_f64(),
        walk_on_ceilings: project_info_map.get("walk_on_ceilings").unwrap().as_f64(),
        total_area: project_info_map
            .get("total_area")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        exterior_area: project_info_map.get("exterior_area").unwrap().as_f64(),
        heated_area: project_info_map
            .get("heated_area")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        pv_area: project_info_map.get("pv_area").unwrap().as_f64(),
        energy_class: Some(project_info_map.get("energy_class").unwrap().to_string()),
        heating_no_modifiers: project_info_map
            .get("heating_no_modifiers")
            .unwrap()
            .as_f64()
            .unwrap(),
        electricity_no_modifiers: project_info_map
            .get("electricity_no_modifiers")
            .unwrap()
            .as_f64()
            .unwrap(),
        heating_with_modifiers: project_info_map
            .get("heating_with_modifiers")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        electricity_with_modifiers: project_info_map
            .get("electricity_with_modifiers")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        electricity_production: project_info_map
            .get("electricity_production")
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
        bbr_code: Some(project_info_map.get("bbr_code").unwrap().to_string()),
        lca_software: project_info_map.get("lca_software").unwrap().to_string(),
        lca_version: project_info_map.get("lca_version").unwrap().to_string(),
    })
}

fn read_operations<T>(
    workbook: &mut Xlsx<T>,
    reference_service_life: &f64,
) -> Result<Vec<BROperation>, Error>
where
    T: Seek,
    T: std::io::Read,
{
    let mut operations = vec![];

    if let Ok(r) = workbook.worksheet_range("Drift") {
        for (index, row) in r.rows().enumerate() {
            if index < 2 {
                continue;
            } else if row[1] == Data::Empty || row[1] == "Type" {
                continue;
            }

            operations.push(BROperation {
                name: row[0].to_string(),
                category: row[1].to_string(),
                _type: row[2].to_string(),
                product: row[3].to_string(),
                included: row[4].to_string() == "Ja".to_string(),
                quantity: row[5].as_f64().unwrap_or(0.0),
                unit: row[6].to_string(),
                environmental_data: BREnvironmentalData {
                    link: row[8].to_string(),
                    epd_number: row[9].to_string(),
                    expiration_data: row[10].to_string(),
                    standard: row[11].to_string(),
                },
                results: HashMap::from([(
                    ImpactCategoryKey::GWP,
                    HashMap::from([(
                        LifeCycleModule::B6,
                        Some(row[13].as_f64().unwrap_or(0.0) * reference_service_life),
                    )]),
                )]),
            })
        }
    }
    Ok(operations)
}

trait FromBR<T> {
    fn from_br(_: T) -> Self;
}

impl FromBR<(&[Data], i32, &f64, &f64)> for Impacts {
    fn from_br(
        (row, start_index, gross_floor_area, reference_service_life): (&[Data], i32, &f64, &f64),
    ) -> Self {
        let categories = [
            ImpactCategoryKey::GWP,
            ImpactCategoryKey::GWP_FOS,
            ImpactCategoryKey::GWP_BIO,
            ImpactCategoryKey::GWP_LUL,
            ImpactCategoryKey::ODP,
            ImpactCategoryKey::AP,
            ImpactCategoryKey::EP_FW,
            ImpactCategoryKey::EP_MAR,
            ImpactCategoryKey::EP_TER,
            ImpactCategoryKey::POCP,
            ImpactCategoryKey::ADPE,
            ImpactCategoryKey::ADPF,
            ImpactCategoryKey::WDP,
        ];
        let stages = [
            LifeCycleModule::A1A3,
            LifeCycleModule::A4,
            LifeCycleModule::A5,
            LifeCycleModule::B1,
            LifeCycleModule::B2,
            LifeCycleModule::B3,
            LifeCycleModule::B4,
            LifeCycleModule::B5,
            LifeCycleModule::B6,
            LifeCycleModule::B7,
            LifeCycleModule::C1,
            LifeCycleModule::C2,
            LifeCycleModule::C3,
            LifeCycleModule::C4,
            LifeCycleModule::D,
        ];
        let mut results = HashMap::new();
        for (c_index, category) in categories.iter().enumerate() {
            let mut result_category = HashMap::new();
            for (s_index, stage) in stages.iter().enumerate() {
                let index = start_index + (c_index as i32 * 15) + s_index as i32;
                let data = &row[index as usize];
                if !data.is_empty() {
                    result_category.insert(
                        stage.clone(),
                        Some(data.as_f64().unwrap() * gross_floor_area * reference_service_life),
                    );
                }
            }
            results.insert(category.clone(), result_category);
        }
        results
    }
}

fn read_components<T>(
    workbook: &mut Xlsx<T>,
    gross_floor_area: &f64,
    reference_service_life: &f64,
) -> Result<Vec<BRComponent>, Error>
where
    T: Seek,
    T: std::io::Read,
{
    let mut components = vec![];

    for name in vec!["Bygningsdele", "BR18 vejledning"] {
        match workbook.worksheet_range(name) {
            Ok(range) => {
                for (index, row) in range.rows().enumerate() {
                    if index < 4 {
                        continue;
                    } else if row[1] == Data::Empty {
                        continue;
                    }

                    components.push(BRComponent {
                        category: row[0].to_string(),
                        _type: row[1].to_string(),
                        building_part: row[2].to_string(),
                        building_part_main: row[3].to_string(),
                        building_part_sub: row[4].to_string(),
                        construction: row[5].to_string(),
                        product: row[6].to_string(),
                        included: row[7].to_string() == "Ja".to_string(),
                        recycled: row[8].to_string() == "Ja".to_string(),
                        structural: row[9].to_string() == "Ja".to_string(),
                        input_quantity: row[10].as_f64().unwrap_or(0.0),
                        input_unit: row[11].to_string(),
                        computed_quantity: row[12].as_f64().unwrap_or(0.0),
                        computed_unit: row[13].to_string(),
                        reference_service_life: row[14].as_f64().unwrap_or(0.0) as u16,
                        delayed_start: row[15].as_f64().unwrap_or(0.0) as u16,
                        replacements: row[16].as_f64().unwrap_or(0.0) as u16,
                        weight: row[17].as_f64().unwrap_or(0.0),
                        weight_unit: row[18].to_string(),
                        environmental_data: BREnvironmentalData {
                            link: row[22].to_string(),
                            epd_number: row[23].to_string(),
                            expiration_data: row[24].to_string(),
                            standard: row[25].to_string(),
                        },
                        results: Impacts::from_br((
                            row,
                            28,
                            gross_floor_area,
                            reference_service_life,
                        )),
                    })
                }
            }
            Err(_) => continue,
        }
    }
    Ok(components)
}
