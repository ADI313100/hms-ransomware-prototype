
use axum::{routing::get, Json, Router};
use serde_json::json;
#[tokio::main]
async fn main(){
 let app=Router::new()
   .route("/",get(||async{"HMS Backend API"}))
   .route("/health",get(||async{Json(json!({"status":"healthy"}))}));
 let listener=tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
 println!("Backend http://127.0.0.1:8080");
 axum::serve(listener,app).await.unwrap();
}
