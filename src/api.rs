use serde::Deserialize;
use serde_aux::prelude::*;

const data_source: &str =
    "https://s3-us-west-2.amazonaws.com/alertwildfire-data-public/all_cameras-v2.json";

#[derive(Debug, Deserialize)]
pub enum FeatureType {
    Feature,
}

#[derive(Debug, Deserialize)]
pub enum GeometryType {
    Point,
}
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    features: Vec<Feature>,
}
#[derive(Debug, Deserialize)]
pub struct Feature {
    r#type: FeatureType,
    properties: Properties,
    geometry: Option<Geometry>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Geometry {
    coordinates: (String, String, f32),
    r#type: GeometryType,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Properties {
    r#type: FeatureType,

    attribution: String,
    activated_at: Option<String>,

    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    tilt_current: Option<f32>,

    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    is_patrol_mode: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    is_currently_patrolling: bool,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    fov: f32,
    fov_rt: [Option<String>; 2],
    fov_lft: [Option<String>; 2],
    fov_center: [Option<String>; 2],

    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    zoom_current: Option<f32>,
    last_movement_at: Option<String>,

    ptz: u32,
    isp: String,
    county: String,
    sponsor: String,
    name: String,
    region: String,
    id: String,
    network: String,
    lastupdate: u32,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    az_current: Option<f32>,
    state: String,

    #[serde(rename = "ProdNbr")]
    prod_nbr: String,
}

#[cfg(test)]
mod test {
    use super::ApiResponse;
    use serde::Deserialize;

    #[test]
    fn parse_api_response() {
        let response = include_str!("../tests/all_cameras-v2.json");
        let jd = &mut serde_json::Deserializer::from_str(response);
        let result: Result<ApiResponse, _> = serde_path_to_error::deserialize(jd);
        // dbg!(&result);
        match result {
            Ok(_) => {}
            Err(e) => panic!("{:#?}", e),
        };
        // let response: ApiResponse = serde_json::from_str(response).unwrap();
        // panic!("{:#?}", response);
    }
}
