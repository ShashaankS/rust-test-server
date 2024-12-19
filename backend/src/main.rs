use actix_web::{post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

async fn greet() -> HttpResponse {
    HttpResponse::Ok().body("Hello from Rust backend!")
}

#[post("/add")]
async fn add(req: Json<serde_json::Value>) -> impl Responder {
    let data = req.into_inner();
    let a = data["a"].as_f64().unwrap_or(0.0);
    let b = data["b"].as_f64().unwrap_or(0.0);
    let sum = a + b;
    let response = serde_json::json!({
        "message": "JSON received successfully",
        "data": sum
    });
    HttpResponse::Ok().json(response)
}

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(Cors::default()
        .allowed_origin("http://localhost:5173")  // Allow your frontend's origin
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])  // Allow specific HTTP methods
        .allowed_headers(vec!["Content-Type", "Authorization"])  // Allowed headers
        .max_age(3600),  // Cache preflight response for 1 hour
        )
        .route("/", web::get().to(greet))
        .route("/index", web::get().to(index))
        .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}