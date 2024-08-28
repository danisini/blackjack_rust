mod controller;
mod model;
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

    let routes = start;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}