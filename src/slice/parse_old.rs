use crate::project::LCAxProject;
use crate::slice::model::SLiCEElement;
use parquet::basic::Type as PhysicalType;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use parquet::schema::types::Type;
use std::collections::HashSet;
use std::fs::File;
use std::sync::Arc;

/// Parse an SLiCE parquet file into LCAx.
///
/// # Arguments
///
/// * `project_data`: JSON formatted string containing the LCAbyg project data
/// * `result_data`: Optional JSON formatted string containing the result data from the LCAByg project
///
/// returns: LCAxProject
pub fn parse_slice(file: File) {
    //-> Result<LCAxProject, Error> {
    let reader = SerializedFileReader::new(file).unwrap();

    let fields = get_fields(&reader);
    // let archetypes = get_unique_values_from_column(&reader, "building_archetype_code");
    let archetypes= get_archetypes();
    let projects = get_projects_by_archetypes(&reader, &archetypes);
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
    reader: &SerializedFileReader<File>,
    archetypes: &HashSet<String>,
) -> Vec<LCAxProject> {
    let mut projects = Vec::new();
    for archetype in archetypes {
        let project_elements = get_elements_by_archetype(reader, archetype);
        let project = LCAxProject::from(&project_elements);
        projects.push(project);
    }
    projects
}

/// Get the elements by archetype from a parquet file
/// # Arguments
/// * `reader`: The parquet file reader
/// * `archetype`: The building_archetype_code to filter the elements by
/// returns: Vec<SLiCEElement>
fn get_elements_by_archetype(
    reader: &SerializedFileReader<File>,
    archetype: &String,
) -> Vec<SLiCEElement> {
    let fields = get_fields(reader);
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
        "material_category_Sturm_upd",
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

    let mut columns = Vec::new();
    for field in wanted_fields.iter() {
        let column = fields.iter().find(|&x| &x.name() == field).unwrap();
        columns.push(column.clone());
    }
    let schema_projection = Type::group_type_builder("schema")
        .with_fields(columns)
        .build()
        .unwrap();

    let mut iter = reader.get_row_iter(Some(schema_projection)).unwrap();
    let mut elements = Vec::new();
    while let Some(record) = iter.next() {
        let value = record.unwrap();
        let building_archetype = value.get_string(4).unwrap();
        if archetype == building_archetype {
            elements.push(SLiCEElement::from((&value, &wanted_fields.to_vec())));
        }
    }
    elements
}

/// Get the unique values from a column in a parquet file
/// # Arguments
///  * `reader`: The parquet file reader
///  * `column_name`: The name of the column to get the unique values from
fn get_unique_values_from_column(
    reader: &SerializedFileReader<File>,
    column_name: &str,
) -> HashSet<String> {
    let fields = get_fields(reader);
    let column = fields.iter().find(|&x| x.name() == column_name).unwrap();
    let columns = vec![column.clone()];
    let schema_projection = Type::group_type_builder("schema")
        .with_fields(columns)
        .build()
        .unwrap();

    let mut iter = reader.get_row_iter(Some(schema_projection)).unwrap();
    let mut unique_values = HashSet::new();
    while let Some(record) = iter.next() {
        let value = record.unwrap();
        unique_values.insert(value.get_string(0).unwrap().clone());
    }
    unique_values
}

fn get_fields(reader: &SerializedFileReader<File>) -> &[Arc<parquet::schema::types::Type>] {
    let parquet_metadata = reader.metadata();

    // Writing the type signature here, to be super
    // clear about the return type of get_fields()
    let fields: &[Arc<parquet::schema::types::Type>] =
        parquet_metadata.file_metadata().schema().get_fields();
    fields
}

fn print_fields(fields: &[Arc<parquet::schema::types::Type>]) {
    for (pos, column) in fields.iter().enumerate() {
        let name = column.name();
        let p_type = column.get_physical_type();
        let output_rust_type = match p_type {
            PhysicalType::FIXED_LEN_BYTE_ARRAY => "String",
            PhysicalType::BYTE_ARRAY => "String",
            PhysicalType::INT64 => "i64",
            PhysicalType::INT32 => "i32",
            PhysicalType::FLOAT => "f32",
            PhysicalType::DOUBLE => "f64",
            _ => panic!(
                "Cannot convert  this parquet file, unhandled data type for column {}",
                name
            ),
        };
        println!("{} {} {}", pos, name, output_rust_type);
    }
}
