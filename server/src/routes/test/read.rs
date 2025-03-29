use actix_web::{HttpResponse, Responder, get};

macros_utils::routes! {
    route route_hello,
}

#[get("/hello")]
pub async fn route_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}
