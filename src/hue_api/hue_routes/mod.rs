use actix_web::{get, web, FromRequest};
use log::debug;

use self::v1::{
    capabilities::get_all_capabilities,
    configuration::*,
    lights::{get_all_lights, scan_for_new_lights},
    sensors::get_all_sensors,
};

mod v1;
mod v2;

pub type SharedState = web::Data<crate::HueAppState>;
#[derive(serde::Deserialize, Debug)]
pub struct V1ApiUserGuard {
    pub uid: String,
}

impl FromRequest for V1ApiUserGuard {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<V1ApiUserGuard, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let state = req.app_data::<SharedState>().unwrap();
        let uid = req.match_info().get("uid").unwrap();
        debug!(
            "APIUserGuard user_exists: {}",
            state.get_controller_read().user_exists(uid)
        );
        match state.get_controller_read().user_exists(uid) {
            true => futures::future::ok(V1ApiUserGuard {
                uid: uid.to_string(),
            }),
            false => futures::future::err(actix_web::error::ErrorUnauthorized("Invalid user")), /* TODO: Return proper NonAuth error */
        }
    }
}

#[derive(Debug)]
pub struct V2ApiUserGuard {
    pub key: String,
    pub name: String,
}

impl FromRequest for V2ApiUserGuard {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<V2ApiUserGuard, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let state = req.app_data::<SharedState>().unwrap();
        let key = req
            .headers()
            .get("hue-application-key")
            .unwrap()
            .to_str()
            .unwrap();
        debug!("HueApplicationKeyGuard key: {}", key);
        match state.get_controller_read().user_exists(key) {
            true => futures::future::ok(V2ApiUserGuard {
                key: key.to_string(),
                name: state
                    .get_controller_read()
                    .get_user_name(key)
                    .get_or_insert("Unknown".to_string())
                    .to_string(),
            }),
            false => {
                futures::future::err(actix_web::error::ErrorForbidden("Invalid application key"))
            }
        }
    }
}

pub fn hue_v1_routes() -> actix_web::Scope {
    // TODO: Refactor to use multiple scopes
    web::scope("/api")
        /* Debug Routes */
        .service(press_link_button)
        .service(is_link_button_pressed)
        .service(save_config)
        /* Configuration Routes */
        .service(get_config)
        .service(create_user)
        .service(get_configuration)
        .service(get_full_datastore)
        .service(modify_configuration)
        /* Light Routes */
        .service(scan_for_new_lights)
        .service(get_all_lights)
        /* Sensor Routes */
        .service(get_all_sensors)
        /* Capabilities */
        .service(get_all_capabilities)
}

pub fn hue_v2_routes() -> actix_web::Scope {
    /* clipstream: https://github.com/actix/actix-web/issues/1690 */
    /* commnad to listen: curl --request GET --url http://localhost/eventstream/clip/v2 --header 'hue-application-key: 74d307ca992111edb5b10242ac120002' -N */
    web::scope("/clip/v2/resource")
        .service(v2::bridge::get_bridge)
        .service(v2::bridge::get_all)
}

pub fn hue_v2_clipstream() -> actix_web::Scope {
    web::scope("/eventstream/clip/v2").service(test_clip)
}

#[get("")]
async fn test_clip() -> &'static str {
    "OK"
}

#[get("/save")]
async fn save_config(api_state: SharedState) -> &'static str {
    api_state.get_controller_read().save();
    "OK"
}

#[get("/slink")]
async fn press_link_button(api_state: SharedState) -> &'static str {
    api_state.get_controller_write().press_link_button();
    "Link button pressed"
}

#[get("/islink")]
async fn is_link_button_pressed(api_state: SharedState) -> String {
    println!(
        "is_link_button_pressed {} ",
        api_state.get_controller_write().is_link_button_pressed()
    );
    if api_state.get_controller_write().is_link_button_pressed() {
        "Link button pressed".to_string()
    } else {
        "Link button not pressed".to_string()
    }
}
