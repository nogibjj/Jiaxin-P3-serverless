use actix_web::{get, web::ServiceConfig, HttpResponse};
use shuttle_service::ShuttleActixWeb;

#[get("/play")]
async fn playgame() -> HttpResponse {
    let options = ["rock", "paper", "scissors"];
    let computer_choice = options[rand::random::<usize>() % options.len()];
    HttpResponse::Ok().body(format!(
        "The computer played: {}",
        computer_choice
    ))
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(playgame);
    })
}
