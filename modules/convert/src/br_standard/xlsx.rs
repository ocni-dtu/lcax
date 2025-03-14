use crate::br_standard::models::{
    BRComponents, BREnvironmentalData, BROperation, BRProjectInfo, BRResults,
};
use crate::br_standard::translations::GENERAL_INFORMATION_TRANSLATION;
use calamine::{open_workbook, Data, DataType, Error, Reader, Xlsx};
use lcax_models::life_cycle_base::{ImpactCategoryKey, Impacts, LifeCycleStage};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
// pub fn br_standard_from_file(file_path: PathBuf) -> Result<Project, Error> {
//     let (project_info, components, operations) = read_br_standard_from_file(file_path)?;
//
//     Ok(parse_br_standard(&project_info, &components, &operations))
// }

pub fn read_br_standard_from_file(
    file_path: PathBuf,
) -> Result<(BRProjectInfo, Vec<BRComponents>, Vec<BROperation>), Error> {
    let mut workbook: Xlsx<_> = open_workbook(file_path)?;
    let project_info = read_project_info(&mut workbook)?;
    let components = read_components(&mut workbook)?;
    let operations = read_operations(&mut workbook)?;
    Ok((project_info, components, operations))
}

fn read_project_info(workbook: &mut Xlsx<BufReader<File>>) -> Result<BRProjectInfo, Error> {
    let mut project_info_map: HashMap<&str, Data> = HashMap::new();

    if let Ok(r) = workbook.worksheet_range("General information") {
        for row in r.rows() {
            match &row[0] {
                Data::String(name) => {
                    match GENERAL_INFORMATION_TRANSLATION.get(name) {
                        Some(key) => project_info_map.insert(*key, row[1].clone()),
                        None => continue,
                    };
                }
                _ => continue,
            }
        }
    }
    Ok(BRProjectInfo {
        typology: project_info_map.get("typology").unwrap().to_string(),
        building_type: project_info_map.get("building_type").unwrap().to_string(),
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
        address: project_info_map.get("address").unwrap().to_string(),
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
            .unwrap(),
        basement_area: project_info_map.get("basement_area").unwrap().as_f64(),
        staircase_area: project_info_map
            .get("staircase_area")
            .unwrap()
            .as_f64()
            .unwrap(),
        garage_area: project_info_map
            .get("garage_area")
            .unwrap()
            .as_f64()
            .unwrap(),
        carport_area: project_info_map.get("carport_area").unwrap().as_f64(),
        walk_on_ceilings: project_info_map.get("walk_on_ceilings").unwrap().as_f64(),
        total_area: project_info_map
            .get("total_area")
            .unwrap()
            .as_f64()
            .unwrap(),
        exterior_area: project_info_map.get("exterior_area").unwrap().as_f64(),
        heated_area: project_info_map
            .get("heated_area")
            .unwrap()
            .as_f64()
            .unwrap(),
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
            .unwrap(),
        electricity_with_modifiers: project_info_map
            .get("electricity_with_modifiers")
            .unwrap()
            .as_f64()
            .unwrap(),
        electricity_production: project_info_map
            .get("electricity_production")
            .unwrap()
            .as_f64()
            .unwrap(),
        bbr_code: Some(project_info_map.get("bbr_code").unwrap().to_string()),
        lca_software: project_info_map.get("lca_software").unwrap().to_string(),
        lca_version: project_info_map.get("lca_version").unwrap().to_string(),
    })
}

fn read_operations(workbook: &mut Xlsx<BufReader<File>>) -> Result<Vec<BROperation>, Error> {
    let mut operations = vec![];

    if let Ok(r) = workbook.worksheet_range("Drift") {
        for (index, row) in r.rows().enumerate() {
            if index < 2 {
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
                results: Impacts::from_br((row, 14)),
                // results: BRResults {
              //     gwp: row[13].as_f64().unwrap_or(0.0),
              //     gwp_fos: row[14].as_f64().unwrap_or(0.0),
              //     gwp_bio: row[15].as_f64().unwrap_or(0.0),
              //     gwp_lul: row[16].as_f64().unwrap_or(0.0),
              //     odp: row[17].as_f64().unwrap_or(0.0),
              //     ap: row[18].as_f64().unwrap_or(0.0),
              //     ep_fw: row[19].as_f64().unwrap_or(0.0),
              //     ep_mar: row[20].as_f64().unwrap_or(0.0),
              //     ep_ter: row[21].as_f64().unwrap_or(0.0),
              //     pocp: row[22].as_f64().unwrap_or(0.0),
              //     adpe: row[23].as_f64().unwrap_or(0.0),
              //     adpf: row[24].as_f64().unwrap_or(0.0),
              //     wdp: row[25].as_f64().unwrap_or(0.0),
              // },
            })
        }
    }
    Ok(operations)
}

trait FromBR<T> {
    fn from_br(_: T) -> Self;
}

impl FromBR<(&[Data], i32)> for Impacts {
    fn from_br((row, start_index): (&[Data], i32)) -> Self {
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
            LifeCycleStage::A1A3,
            LifeCycleStage::A4,
            LifeCycleStage::A5,
            LifeCycleStage::B1,
            LifeCycleStage::B2,
            LifeCycleStage::B3,
            LifeCycleStage::B4,
            LifeCycleStage::B5,
            LifeCycleStage::B6,
            LifeCycleStage::B7,
            LifeCycleStage::C1,
            LifeCycleStage::C2,
            LifeCycleStage::C3,
            LifeCycleStage::C4,
            LifeCycleStage::D,
        ];
        let mut results = HashMap::new();
        for (c_index, category) in categories.iter().enumerate() {
            let mut result_category = HashMap::new();
            for (s_index, stage) in stages.iter().enumerate() {
                let index = start_index + (c_index as i32 * 15) + s_index as i32;
                let data = &row[index as usize];
                result_category.insert(stage.clone(), data.as_f64());
            }
            results.insert(category.clone(), result_category);
        }
        results
    }
}

fn read_components(workbook: &mut Xlsx<BufReader<File>>) -> Result<Vec<BRComponents>, Error> {
    let mut components = vec![];

    if let Ok(r) = workbook.worksheet_range("Bygningsdele") {
        for (index, row) in r.rows().enumerate() {
            if index < 3 {
                continue;
            }

            components.push(BRComponents {
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
                results: BRResults {
                    gwp: row[27].as_f64().unwrap_or(0.0),
                    gwp_fos: row[28].as_f64().unwrap_or(0.0),
                    gwp_bio: row[29].as_f64().unwrap_or(0.0),
                    gwp_lul: row[30].as_f64().unwrap_or(0.0),
                    odp: row[31].as_f64().unwrap_or(0.0),
                    ap: row[32].as_f64().unwrap_or(0.0),
                    ep_fw: row[33].as_f64().unwrap_or(0.0),
                    ep_mar: row[34].as_f64().unwrap_or(0.0),
                    ep_ter: row[35].as_f64().unwrap_or(0.0),
                    pocp: row[36].as_f64().unwrap_or(0.0),
                    adpe: row[37].as_f64().unwrap_or(0.0),
                    adpf: row[38].as_f64().unwrap_or(0.0),
                    wdp: row[39].as_f64().unwrap_or(0.0),
                },
            })
        }
    }
    Ok(components)
}
