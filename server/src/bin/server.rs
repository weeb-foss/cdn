use actix_web::web::get;
use actix_web::{App, HttpResponse, HttpServer};
use flexi_logger::Logger;
use logger::format_log;
use server::{AppError, routes};

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::try_with_str("info")? //
        .format(format_log)
        .log_to_stdout()
        .start()?;

    HttpServer::new(|| {
        App::new() //
            .route("/", get().to(HttpResponse::Ok))
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
