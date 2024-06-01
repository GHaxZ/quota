use rocket::{launch, routes};
use rocket::http::Method;
use rocket::tokio::sync::RwLock;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use crate::endpoints::{add_quote, get_all_quotes, get_quote, get_quotes_amount};
use crate::quote_store::QuoteStore;
use crate::config::Config;

mod quote_store;
mod endpoints;
mod config;

#[launch]
fn rocket() -> _ {
    let store = QuoteStore::new("quotes.json").unwrap();

    // Configure cors
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Options]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]))
        .allow_credentials(true)
        .to_cors().unwrap();

    // Define API configuration here
    let config = Config {
        allow_post: true,
        allowed_quote_length: 300,
        allowed_quotee_length: 50
    };

    rocket::build()
        .manage(RwLock::new(store))
        .manage(config)
        .mount("/", routes![get_quote, get_all_quotes, get_quotes_amount, add_quote])
        .attach(cors)
}