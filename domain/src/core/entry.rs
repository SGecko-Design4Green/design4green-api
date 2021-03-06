use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub global: Option<f64>,
    pub global_region: Option<f64>,
    pub global_dept: Option<f64>,
    pub global_national: Option<f64>,
    pub iris_code: Option<String>,
    pub iris_code_designation: Option<String>,
    pub information_access: Option<InformationAccess>,
    pub numeric_interfaces_access: Option<NumericInterfacesAccess>,
    pub administrative_competencies: Option<AdministrativeCompetencies>,
    pub numeric_competencies: Option<NumericCompetencies>,
}

impl Entry {
    pub fn new(
        global: Option<f64>,
        global_region: Option<f64>,
        global_dept: Option<f64>,
        global_national: Option<f64>,
        iris_code: Option<String>,
        iris_code_designation: Option<String>,
        information_access: Option<InformationAccess>,
        numeric_interfaces_access: Option<NumericInterfacesAccess>,
        administrative_competencies: Option<AdministrativeCompetencies>,
        numeric_competencies: Option<NumericCompetencies>,
    ) -> Self {
        Entry {
            global,
            global_region,
            global_dept,
            global_national,
            iris_code,
            iris_code_designation,
            information_access,
            numeric_interfaces_access,
            administrative_competencies,
            numeric_competencies,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InformationAccess {
    pub global: Option<f64>,
    pub global_region: Option<f64>,
    pub global_dept: Option<f64>,
    pub global_national: Option<f64>,
    pub monoparental_families_percent: Option<f64>,
    pub single_person_percent: Option<f64>,
    pub number_of_public_service_per_citizen: Option<f64>,
    pub number_of_public_services: Option<f64>,
}

impl InformationAccess {
    pub fn new(
        global: Option<f64>,
        global_region: Option<f64>,
        global_dept: Option<f64>,
        global_national: Option<f64>,
        monoparental_families_percent: Option<f64>,
        single_person_percent: Option<f64>,
        number_of_public_service_per_citizen: Option<f64>,
        number_of_public_services: Option<f64>,
    ) -> Self {
        InformationAccess {
            global,
            global_region,
            global_dept,
            global_national,
            monoparental_families_percent,
            single_person_percent,
            number_of_public_service_per_citizen,
            number_of_public_services,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NumericInterfacesAccess {
    pub global: Option<f64>,
    pub global_region: Option<f64>,
    pub global_dept: Option<f64>,
    pub global_national: Option<f64>,
    pub high_speed_internet_access_percent: Option<f64>,
    pub mobile_network_availability_percent: Option<f64>,
    pub percent_of_poor_people: Option<f64>,
    pub available_median_salary: Option<f64>,
}

impl NumericInterfacesAccess {
    pub fn new(
        global: Option<f64>,
        global_region: Option<f64>,
        global_dept: Option<f64>,
        global_national: Option<f64>,
        high_speed_internet_access_percent: Option<f64>,
        mobile_network_availability_percent: Option<f64>,
        percent_of_poor_people: Option<f64>,
        available_median_salary: Option<f64>,
    ) -> Self {
        NumericInterfacesAccess {
            global,
            global_region,
            global_dept,
            global_national,
            high_speed_internet_access_percent,
            mobile_network_availability_percent,
            percent_of_poor_people,
            available_median_salary,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdministrativeCompetencies {
    pub global: Option<f64>,
    pub global_region: Option<f64>,
    pub global_dept: Option<f64>,
    pub global_national: Option<f64>,
    pub unemployed_percent: Option<f32>,
    #[serde(alias = "15-29Percent")]
    pub _15_29_percent: Option<f32>,
}

impl AdministrativeCompetencies {
    pub fn new(
        global: Option<f64>,
        global_region: Option<f64>,
        global_dept: Option<f64>,
        global_national: Option<f64>,
        unemployed_percent: Option<f32>,
        _15_29_percent: Option<f32>,
    ) -> Self {
        AdministrativeCompetencies {
            global,
            global_region,
            global_dept,
            global_national,
            unemployed_percent,
            _15_29_percent,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NumericCompetencies {
    pub global: Option<f64>,
    pub global_region: Option<f64>,
    pub global_dept: Option<f64>,
    pub global_national: Option<f64>,
    #[serde(alias = "percentOf65+People")]
    pub percent_of_65_plus_people: Option<f32>,
    pub percent_of_people_without_grade: Option<f32>,
}

impl NumericCompetencies {
    pub fn new(
        global: Option<f64>,
        global_region: Option<f64>,
        global_dept: Option<f64>,
        global_national: Option<f64>,
        percent_of_65_plus_people: Option<f32>,
        percent_of_people_without_grade: Option<f32>,
    ) -> NumericCompetencies {
        NumericCompetencies {
            global,
            global_region,
            global_dept,
            global_national,
            percent_of_65_plus_people,
            percent_of_people_without_grade,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoLoc {
    pub lat: f64,
    pub long: f64,
}

impl GeoLoc {
    pub fn new(lat: f64, long: f64) -> GeoLoc {
        GeoLoc { lat, long }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Iris {
    pub code: Option<String>,
    pub geo_loc: Option<GeoLoc>,
}

impl Iris {
    pub fn new(code: Option<String>, geo_loc: Option<GeoLoc>) -> Iris {
        Iris { code, geo_loc }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub cities: BTreeMap<String, CityDetail>,
}

impl SearchResult {
    pub fn new() -> Self {
        SearchResult {
            cities: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityDetail {
    pub code_insee: Option<String>,
    pub districts: Option<Vec<District>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct District {
    pub code_iris: String,
    pub designation: String,
}

impl District {
    pub fn new(code_iris: String, designation: String) -> Self {
        District {
            code_iris,
            designation,
        }
    }
}
