use actix_web::{get, web, Responder};

use crate::hue_api::hue_routes::{SharedState, V1ApiUserGuard};

#[get("/{uid}/sensors")]
pub async fn get_all_sensors(_uid: V1ApiUserGuard, _api_state: SharedState) -> impl Responder {
    // let controller = &api_state.get_controller_read();
    let resp = r#"{
        "1": {
          "manufacturername": "Signify Netherlands B.V.",
          "swversion": "1.0",
          "state": {
            "daylight": null,
            "lastupdated": "none"
          },
          "config": {
            "on": true,
            "configured": false,
            "sunriseoffset": 30,
            "sunsetoffset": -30,
            "reachable": true
          },
          "name": "Daylight",
          "type": "Daylight",
          "modelid": "PHDL00"
        }
      }"#;
    web::Json(serde_json::from_str::<serde_json::Value>(resp).unwrap())
}
