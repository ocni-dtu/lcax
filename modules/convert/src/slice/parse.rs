use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bytes::Bytes;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use parquet::schema::types::Type;

use lcax_models::project::Project as LCAxProject;

use crate::slice::model::{add_project_data, add_slice_element, SLiCEElement};

/// Parse an SLiCE parquet file into LCAx.
///
/// # Arguments
///
/// * `project_data`: JSON formatted string containing the LCAbyg project data
/// * `result_data`: Optional JSON formatted string containing the result data from the LCAByg project
///
/// returns: LCAxProject
pub fn parse_slice(file: Vec<u8>) -> Result<Vec<LCAxProject>, String> {
    let cursor: Bytes = file.into();
    let reader = match SerializedFileReader::new(cursor) {
        Ok(reader) => reader,
        Err(_) => return Err("Error reading parquet file".to_string()),
    };

    let archetypes = get_archetypes();
    let projects = get_projects_by_archetypes(&reader, &archetypes);
    Ok(projects)
}

fn get_archetypes() -> HashSet<&'static str> {
    HashSet::from([
        "CON_MFH_EXB_AVG",
        "CON_MFH_NEW_ADV",
        "CON_MFH_NEW_STD",
        "CON_MFH_REF_ADV",
        "CON_MFH_REF_STD",
        "CON_OFF_EXB_AVG",
        "CON_OFF_NEW_ADV",
        "CON_OFF_NEW_STD",
        "CON_OFF_REF_ADV",
        "CON_OFF_REF_STD",
        "CON_SFH_EXB_AVG",
        "CON_SFH_NEW_ADV",
        "CON_SFH_NEW_STD",
        "CON_SFH_REF_ADV",
        "CON_SFH_REF_STD",
        //---
        "MED_MFH_EXB_AVG",
        "MED_MFH_NEW_ADV",
        "MED_MFH_NEW_STD",
        "MED_MFH_REF_ADV",
        "MED_MFH_REF_STD",
        "MED_OFF_EXB_AVG",
        "MED_OFF_NEW_ADV",
        "MED_OFF_NEW_STD",
        "MED_OFF_REF_ADV",
        "MED_OFF_REF_STD",
        "MED_SFH_EXB_AVG",
        "MED_SFH_NEW_ADV",
        "MED_SFH_NEW_STD",
        "MED_SFH_REF_ADV",
        "MED_SFH_REF_STD",
        //---
        "NOR_MFH_EXB_AVG",
        "NOR_MFH_NEW_ADV",
        "NOR_MFH_NEW_STD",
        "NOR_MFH_REF_ADV",
        "NOR_MFH_REF_STD",
        "NOR_OFF_EXB_AVG",
        "NOR_OFF_NEW_ADV",
        "NOR_OFF_NEW_STD",
        "NOR_OFF_REF_ADV",
        "NOR_OFF_REF_STD",
        "NOR_SFH_EXB_AVG",
        "NOR_SFH_NEW_ADV",
        "NOR_SFH_NEW_STD",
        "NOR_SFH_REF_ADV",
        "NOR_SFH_REF_STD",
        //---
        "OCE_MFH_EXB_AVG",
        "OCE_MFH_NEW_ADV",
        "OCE_MFH_NEW_STD",
        "OCE_MFH_REF_ADV",
        "OCE_MFH_REF_STD",
        "OCE_OFF_EXB_AVG",
        "OCE_OFF_NEW_ADV",
        "OCE_OFF_NEW_STD",
        "OCE_OFF_REF_ADV",
        "OCE_OFF_REF_STD",
        "OCE_SFH_EXB_AVG",
        "OCE_SFH_NEW_ADV",
        "OCE_SFH_NEW_STD",
        "OCE_SFH_REF_ADV",
        "OCE_SFH_REF_STD",
    ])
}
/// Get the LCAx projects by archetypes from a parquet file
/// # Arguments
/// * `reader`: The parquet file reader
/// * `archetypes`: The unique values of the building_archetype_code column
/// returns: Vec<LCAxProject>
fn get_projects_by_archetypes(
    reader: &SerializedFileReader<Bytes>,
    archetypes: &HashSet<&str>,
) -> Vec<LCAxProject> {
    let mut projects: HashMap<String, LCAxProject> = archetypes
        .iter()
        .map(|a| (a.to_string(), LCAxProject::default()))
        .collect();

    let wanted_fields = [
        "stock_region_name",
        "building_use_subtype_name",
        "stock_activity_type_name",
        "building_energy_performance_name",
        "building_archetype_code",
        "element_class_generic_name",
        "element_class_sfb",
        "worksection_class_sfb",
        "techflow_name_mmg",
        "material_name_mmg",
        "material_name_JRC_CDW",
        "material_category_Sturm",
        //"material_category_Sturm_upd",
        "amount_material_kg_per_building",
        "activity_year",
        "LCS_EN15978",
        "ind_GWP_Tot",
        "ind_GWP_Fos",
        "ind_GWP_Bio",
        "ind_GWP_LuLuc",
        "ind_ODP",
        "ind_AP",
        "ind_EP_Fw",
        "ind_EP_Mar",
        "ind_EP_Ter",
        "ind_PCOP",
        "ind_ADP_MiMe",
        "ind_ADP_Fos",
        "ind_WDP",
        "ind_PM",
        "ind_IRP",
        "ind_ETP_Fw",
        "ind_HTP_c",
        "ind_HTP_nc",
        "ind_SQP",
    ];
    let schema_projection = get_schema_projection(wanted_fields.to_vec(), &reader);
    let mut iter = reader.get_row_iter(Some(schema_projection)).unwrap();
    while let Some(record) = iter.next() {
        let value = record.unwrap();
        match value.get_string(4) {
            Ok(archetype) => {
                let mut project: &mut LCAxProject = projects.get_mut(archetype).unwrap();
                let element = SLiCEElement::from((&value, &wanted_fields.to_vec()));
                if project.id.is_empty() {
                    add_project_data(&mut project, &element);
                }
                add_slice_element(&mut project, &element);
            }
            Err(_) => println!("Couldn't get archetype. Skipping row"),
        }
    }
    projects.values().cloned().collect()
}

fn get_schema_projection(wanted_fields: Vec<&str>, reader: &SerializedFileReader<Bytes>) -> Type {
    let fields = get_fields(reader);
    let mut columns = Vec::new();

    for field in wanted_fields.iter() {
        let column = fields.iter().find(|&x| &x.name() == field).unwrap();
        columns.push(column.clone());
    }
    Type::group_type_builder("schema")
        .with_fields(columns)
        .build()
        .unwrap()
}

fn get_fields(reader: &SerializedFileReader<Bytes>) -> &[Arc<parquet::schema::types::Type>] {
    let parquet_metadata = reader.metadata();

    // Writing the type signature here, to be super
    // clear about the return type of get_fields()
    let fields: &[Arc<parquet::schema::types::Type>] =
        parquet_metadata.file_metadata().schema().get_fields();
    fields
}
