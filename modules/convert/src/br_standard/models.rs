use lcax_models::life_cycle_base::Impacts;

pub struct BRProjectInfo {
    // General Info
    pub typology: String,
    pub building_type: String,
    pub floors_above_ground: u16,
    pub floors_below_ground: u16,
    pub address: String,

    // Key Numbers
    pub building_permission_date: Option<String>,
    pub building_operation_date: Option<String>,
    pub building_regulation: String,
    pub threshold_new_constructions: f64,
    pub threshold_low_emission: f64,

    // Results
    pub total_emissions: f64,
    pub emission_modifiers: f64,
    pub emission_all_modules: f64,
    pub emission_module_d: f64,

    // Areas
    pub gross_floor_area: f64,
    pub basement_area: Option<f64>,
    pub staircase_area: f64,
    pub garage_area: f64,
    pub carport_area: Option<f64>,
    pub walk_on_ceilings: Option<f64>,
    pub total_area: f64,
    pub exterior_area: Option<f64>,
    pub heated_area: f64,
    pub pv_area: Option<f64>,

    // Operation
    pub energy_class: Option<String>,
    pub heating_no_modifiers: f64,
    pub electricity_no_modifiers: f64,
    pub heating_with_modifiers: f64,
    pub electricity_with_modifiers: f64,
    pub electricity_production: f64,

    // Ownership
    pub bbr_code: Option<String>,

    // Software
    pub lca_software: String,
    pub lca_version: String,
}

pub struct BRComponent {
    pub category: String,
    pub _type: String,
    pub building_part: String,
    pub building_part_main: String,
    pub building_part_sub: String,
    pub construction: String,
    pub product: String,
    pub included: bool,
    pub recycled: bool,
    pub structural: bool,
    pub input_quantity: f64,
    pub input_unit: String,
    pub computed_quantity: f64,
    pub computed_unit: String,
    pub reference_service_life: u16,
    pub delayed_start: u16,
    pub replacements: u16,
    pub weight: f64,
    pub weight_unit: String,
    pub environmental_data: BREnvironmentalData,
    pub results: Impacts,
}

pub struct BROperation {
    pub name: String,
    pub category: String,
    pub _type: String,
    pub product: String,
    pub included: bool,
    pub quantity: f64,
    pub unit: String,
    pub environmental_data: BREnvironmentalData,
    pub results: Impacts,
}

pub struct BREnvironmentalData {
    pub link: String,
    pub epd_number: String,
    pub expiration_data: String,
    pub standard: String,
}
