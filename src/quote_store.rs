use std::fs::File;
use std::{io};
use std::io::{Read, Write};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::path::Path;
use rocket::form::validate::{len, range};

#[derive(Serialize, Deserialize, Clone)]
pub struct Quote {
    pub quote: String,
    pub quotee: String,
}

pub struct QuoteStore {
    file_path: String,
    quotes: Vec<Quote>,
}

impl QuoteStore {
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let mut store = Self {
            file_path: file_path.to_string(),
            quotes: Vec::new(),
        };

        if Path::new(file_path).exists() {
            store.load_quotes()?;
        } else {
            store.save_quotes()?; // Save an empty quotes list if the file does not exist
        }

        Ok(store)
    }

    pub fn add_quote(&mut self, quote: Quote) -> Result<(), io::Error> {
        self.quotes.push(quote);
        self.save_quotes()?;
        Ok(())
    }

    fn save_quotes(&mut self) -> Result<(), io::Error> {
        let json_str = serde_json::to_string(&self.quotes)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    fn load_quotes(&mut self) -> Result<(), io::Error> {
        let mut file = File::open(&self.file_path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        self.quotes = if file_content.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&file_content)?
        };

        Ok(())
    }

    pub fn get_quote(&self) -> Option<Quote> {
        let mut rng = rand::thread_rng();

        // Don't want to give references, so they can't modify the stored quotes, so we clone
        self.quotes.choose(&mut rng).cloned()
    }

    pub fn get_all_quotes(&self) -> Vec<Quote> {
        // Don't want to give references, so they can't modify the stored quotes, so we clone
        self.quotes.clone()
    }

    pub fn get_quotes_amount(&self, amount: usize) -> Vec<Quote> {
        let mut rng = rand::thread_rng();

        // Don't want to give references, so they can't modify the stored quotes, so we clone
        let mut quotes = self.quotes.clone();

        // Shuffle elements
        quotes.shuffle(&mut rng);

        // Return requested amount of quotes, or all quotes if more were requested than available
        quotes.into_iter().take(amount).collect()
    }
}
