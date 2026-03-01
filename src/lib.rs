mod utils;

use std::sync::{OnceLock, RwLock};
use wasm_bindgen::prelude::*;

static CACHE: OnceLock<RwLock<utils::Cache>> = OnceLock::new();

fn cache() -> &'static RwLock<utils::Cache> {
    CACHE.get_or_init(|| RwLock::new(utils::Cache::new()))
}

#[wasm_bindgen]
pub struct Factor {
    prime: u64,
    deg: usize,
}

#[wasm_bindgen]
impl Factor {
    #[wasm_bindgen(constructor)]
    pub fn new(prime: u64, deg: usize) -> Self {
        Self {
            prime: prime,
            deg: deg,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn prime(&self) -> u64 {
        self.prime
    }

    #[wasm_bindgen(getter)]
    pub fn deg(&self) -> usize {
        self.deg
    }
}

#[wasm_bindgen]
pub fn factors(n: u64) -> Vec<Factor> {
    let cache = cache().write().unwrap();
    utils::factors(n, cache.clone())
        .into_iter()
        .map(|(k, v)| Factor::new(k, v))
        .collect()
}
