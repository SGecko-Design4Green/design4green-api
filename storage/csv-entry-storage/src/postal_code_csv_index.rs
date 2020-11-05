use domain::core::entry::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostalCodeIrisCodeCSV {
    #[serde(rename(deserialize = "INSEE_COM"))]
    pub iris_code: String,
    #[serde(rename(deserialize = "NOM_COM"))]
    pub nom_com: String,
    #[serde(rename(deserialize = "Code_postal"))]
    pub postal_code: String,
    #[serde(rename(deserialize = "Geo Point"))]
    pub geo_point: String,
}

impl PostalCodeIrisCodeCSV {
    pub fn to_postal_code(&self) -> Iris {
        let geo_parts: Vec<&str> = self.geo_point.split(",").collect();
        Iris::new(
            Some(self.iris_code.clone()),
            Some(GeoLoc::new(
                geo_parts[0].parse::<f64>().unwrap(),
                geo_parts[1].parse::<f64>().unwrap(),
            )),
        )
    }

    pub fn get_code(&self) -> String {
        match self.postal_code.trim().is_empty() {
            true => self.iris_code.chars().take(2).collect(),
            false => self.postal_code.clone()
        }
    }
}
