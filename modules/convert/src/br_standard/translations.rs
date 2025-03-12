use phf::{phf_map};

pub static GENERAL_INFORMATION_TRANSLATION: phf::Map<&'static str, &'static str> = phf_map! {
    "Bygningstypologi (BBR kode)" => "typology",
    "Projekttype" => "building_type",
    "Antal etager over terræn" => "floors_above_ground" ,
    "Antal etager under terræn" => "floors_below_ground" ,
    "Bygningens adresse" => "address" ,
    "Dato for ansøgning om byggetilladelse" => "building_permission_date" ,
    "Dato for ansøgning om ibrugtagningstilladelse" => "building_operation_date" ,
    "Gældende bygningsreglement" => "building_regulation" ,
    "Gældende grænseværdi for nybyggeri over 1.000 m2" => "threshold_new_constructions" ,
    "Gældende grænseværdi i lavemissionsklassen" => "threshold_low_emission" ,
    "Samlet klimapåvirkning ekskl. evt. tillæg (A1-A3, B4, B6, C3-C4)" => "total_emissions" ,
    "Tillæg for særlige forhold jf. §298, stk 3-4" => "emission_modifiers" ,
    "Samlet klimapåvirkning for alle moduler ekskl. D (A1-3, A4, A5, B1, B2, B3, B4, B5, B6, C1, C2, C3, C4)" => "emission_all_modules" ,
    "Klimapåvirkning for modul D" => "emission_module_d" ,
    "Etageareal jf. §455" => "gross_floor_area" ,
    "Kælderareal, affaldsrum i terræn og sikringsrum (integrerede) (100% medregnes)" => "basement_area" ,
    "Udvendige ramper, trapper, brandtrapper, altaner, altangange og lign (25% medregnes)" => "staircase_area",
    "Integrerede garager til enfamiliehuse, rækkehuse og lign. (50% medregnes)" => "garage_area",
    "Integrerede carporte, udhuse, overdækninger, skure og lign. (25% medregnes)" => "carport_area",
    "Walk-on-ceilings og lign. (25% medregnes)" => "walk_on_ceilings",
    "Samlet areal (A1-A3, B4, C3-C4)" => "total_area",
    "Udeareal" => "exterior_area",
    "Opvarmet etageareal jf. §256, nr. 3" => "heated_area",
    "Areal af integrerede solceller" => "pv_area",
    "Den projekterede energiklasse" => "energy_class",
    "Driftforbrug, varme (uden tillæg)" => "heating_no_modifiers",
    "Driftforbrug, el (uden tillæg)" => "electricity_no_modifiers",
    "Driftforbrug, varme (med tillæg)" => "heating_with_modifiers",
    "Driftforbrug, el (med tillæg)" => "electricity_with_modifiers",
    "Egenproduktion, el" => "electricity_production",
    "BBR kode for ejerforhold" => "bbr_code",
    "LCA software" => "lca_software",
    "LCA version" => "lca_version",
};





