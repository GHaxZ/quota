use rocket::{launch, routes};
use rocket::tokio::sync::RwLock;
use crate::endpoints::{add_quote, get_all_quotes, get_quote, get_quotes_amount};
use crate::quote_store::QuoteStore;
use crate::config::Config;

mod quote_store;
mod endpoints;
mod config;

#[launch]
fn rocket() -> _ {
    let store = QuoteStore::new("quotes.json").unwrap();

    // Define API configuration here
    let config = Config {
        allow_post: true
    };

    rocket::build()
        .manage(RwLock::new(store))
        .manage(config)
        .mount("/", routes![get_quote, get_all_quotes, get_quotes_amount, add_quote])
}