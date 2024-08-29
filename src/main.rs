mod controller;
mod model;
mod game_service;
mod response_builder;
mod utils;
use warp::Filter;
use crate::controller::GameController;
use crate::model::GameRequest;


#[tokio::main]

async fn main() {
    let start = warp::path("start")
    .and(warp::post())
    .and(warp::body::json())
    .map(|request: GameRequest| {
        let response = GameController::start(request);
        warp::reply::json(&response)
    });

    let hit = warp::path("hit")
    .and(warp::post())
    .and(warp::body::json())
    .map(|request: GameRequest| {
        let response = GameController::hit(request);
        warp::reply::json(&response)
    });

    let split = warp::path("split")
    .and(warp::post())
    .and(warp::body::json())
    .map(|request: GameRequest| {
        let response = GameController::split(request);
        warp::reply::json(&response)
    });

    let double_stake = warp::path("double")
    .and(warp::post())
    .and(warp::body::json())
    .map(|request: GameRequest| {
        let response = GameController::double_stake(request);
        warp::reply::json(&response)
    });

    let stand = warp::path("stand")
    .and(warp::post())
    .and(warp::body::json())
    .map(|request: GameRequest| {
        let response = GameController::stand(request);
        warp::reply::json(&response)
    });

    let routes = start.or(hit).or(double_stake).or(split).or(stand);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}