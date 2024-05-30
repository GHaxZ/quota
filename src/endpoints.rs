use std::io;
use rocket::{get, post, State};
use rocket::form::validate::len;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use crate::quote_store::{Quote, QuoteStore};
use rocket::tokio::sync::RwLock;
use crate::config::Config;

/*
TODO: GET parameters such as specific quotee, amount, all, etc. 
 */

#[get("/get")]
pub async fn get_quote(store: &State<RwLock<QuoteStore>>) -> Result<String, io::Error> {
    let store = store.read().await;
    match store.get_quote() {
        None => Err(io::Error::new(io::ErrorKind::InvalidData, "The API failed getting a quote from its storage")),
        Some(q) => to_json_string(&q),
    }
}

#[get("/get/all")]
pub async fn get_all_quotes(store: &State<RwLock<QuoteStore>>) -> Result<String, io::Error> {
    let store = store.read().await;
    let quotes = store.get_all_quotes();

    Ok(to_json_string(&quotes)?)
}

#[get("/get/<amount>")]
pub async fn get_quotes_amount(store: &State<RwLock<QuoteStore>>, amount: usize) -> Result<String, io::Error> {
    let store = store.read().await;
    let quotes = store.get_quotes_amount(amount);

    Ok(to_json_string(&quotes)?)
}

#[post("/add", data = "<new_quote>")]
pub async fn add_quote(store: &State<RwLock<QuoteStore>>, config: &State<Config>, new_quote: Json<Quote>) -> Status {
    if config.allow_post == false {
        return Status::Forbidden;
    }

    let mut store = store.write().await;

    let quote = Quote {
        quote: new_quote.quote.clone(),
        quotee: new_quote.quotee.clone(),
    };

    if quote.quote.chars().count() > config.allowed_quote_length ||
        quote.quotee.chars().count() > config.allowed_quotee_length {
        return Status::PayloadTooLarge
    }

    match store.add_quote(quote) {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

fn to_json_string<T: Serialize>(element: &T) -> Result<String, io::Error> {
    match serde_json::to_string(element).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)) {
        Ok(o) => { Ok(o) }
        Err(_) => { Err(io::Error::new(io::ErrorKind::InvalidData, "The API failed serializing the response to json")) }
    }
}
