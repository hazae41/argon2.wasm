use memory_wasm::Memory;
use wasm_bindgen::prelude::*;

use crate::rjse;

#[wasm_bindgen]
pub struct Argon2Deriver {
    pub(crate) inner: argon2::Argon2<'static>,
}

#[wasm_bindgen]
impl Argon2Deriver {
    #[wasm_bindgen(constructor)]
    pub fn new(algorithm: &str, version: u32, memory: u32, iterations: u32, parallelism: u32) -> Result<Argon2Deriver, JsError> {
        use std::str::FromStr;

        let algorithm2 = rjse!(argon2::Algorithm::from_str(algorithm))?;
        let version2 = rjse!(argon2::Version::try_from(version))?;

        let params = rjse!(argon2::Params::new(memory, iterations, parallelism, None))?;
        let inner = argon2::Argon2::new(algorithm2, version2, params);

        Ok(Self { inner })
    }

    #[wasm_bindgen]
    pub fn derive(&self, password: &Memory, salt: &Memory) -> Result<Memory, JsError> {
        let mut output = vec![0u8; 32];

        rjse!(self.inner.hash_password_into(&password.inner, &salt.inner, &mut output))?;

        Ok(Memory::new(output))
    }
}
