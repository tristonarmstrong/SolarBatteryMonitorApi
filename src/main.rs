use actix_web::{get, post, web::ServiceConfig, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;

#[derive(Debug, Serialize, Deserialize)]
struct Battery {
    id: u32,
    time: String,
    value: f32,
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/batteries")]
async fn batteries() -> impl Responder {
    let batteries = vec![
        Battery {
            id: 0,
            time: "Sat Jul 15 2023 07:29:40 GMT-0500 (Central Daylight Time)".to_string(),
            value: 12.34,
        },
        Battery {
            id: 1,
            time: "Sat Jul 15 2023 07:29:40 GMT-0500 (Central Daylight Time)".to_string(),
            value: 12.2,
        },
        Battery {
            id: 2,
            time: "Sat Jul 15 2023 07:29:40 GMT-0500 (Central Daylight Time)".to_string(),
            value: 12.14,
        },
    ];
    //grab batteries from api
    // return batteries
    HttpResponse::Ok().json(batteries)
}

#[post("/update_battery")]
async fn update_battery() -> impl Responder {
    // if battery doesnt exist, create one
    // update battery
    // return new battery details
    HttpResponse::Ok()
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(batteries)
            .service(update_battery);
    };

    Ok(config.into())
}
