use actix_web::{error::ErrorBadRequest, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_persist::PersistInstance;
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
struct AppState {
    persist: PersistInstance,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Battery {
    id: u32,
    time: String,
    value: f32,
    name: String,
}

#[get("/batteries")]
async fn batteries(state: web::Data<AppState>) -> impl Responder {
    let files = fs::read_dir("./shuttle_persist/battery-monitor-api");
    let mut bats: Vec<Battery> = vec![];
    if let Ok(path) = files {
        path.for_each(|p| {
            let val = p.unwrap().path().display().to_string();
            let battery_number = val
                .rsplit_once("/")
                .unwrap()
                .1
                .rsplit_once(".")
                .unwrap()
                .0
                .to_string()
                .parse::<u32>()
                .unwrap();

            let data: Option<Battery> =
                match state.persist.load(format!("{}", battery_number).as_str()) {
                    Ok(i) => Some(i),
                    Err(_e) => None,
                };
            if data.is_some() {
                bats.insert(0, data.unwrap());
            }
        });
    };
    HttpResponse::Ok().json(bats)
}

#[post("/update_battery")]
async fn update_battery(data: web::Json<Battery>, state: web::Data<AppState>) -> impl Responder {
    let unwrapped_data = data.clone();

    match state
        .persist
        .save::<Battery>(
            format!("{}", &data.id.to_string().as_str()).as_str(),
            data.into_inner(),
        )
        .map_err(|e| ErrorBadRequest(e.to_string()))
    {
        Ok(val) => val,
        Err(err) => println!("{}", err),
    };

    HttpResponse::Ok().json(unwrapped_data)
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    std::env::set_var("RUST_LOG", "debug");
    let state = AppState { persist };
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(batteries)
            .service(update_battery)
            .app_data(web::Data::new(state.clone()));
    };

    Ok(config.into())
}
