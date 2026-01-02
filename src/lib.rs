mod factors;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Factor {
    prime: u32,
    deg: u32,
}

#[wasm_bindgen]
impl Factor {
    #[wasm_bindgen(constructor)]
    pub fn new(prime: u32, deg: u32) -> Self {
        Self {
            prime: prime,
            deg: deg,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn prime(&self) -> u32 {
        self.prime
    }

    #[wasm_bindgen(getter)]
    pub fn deg(&self) -> u32 {
        self.deg
    }

    pub fn to_string(&self) -> String {
        if self.deg == 1 {
            format!("{}", self.prime)
        } else {
            format!("{}^{}", self.prime, self.deg)
        }
    }
}

#[wasm_bindgen]
pub fn factors(n: u32) -> Vec<Factor> {
    factors::factors(n)
        .into_iter()
        .map(|(k, v)| Factor::new(k, v))
        .collect()
}
