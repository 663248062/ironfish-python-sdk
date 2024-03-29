use fish_hash::Context;
use pyo3::prelude::*;
use bytes::Bytes;

#[pyclass]
pub struct FishHashContext {
    inner: Context,
}

#[pymethods]
impl FishHashContext {
    #[new]
    pub fn new(full: bool) -> Self {
        Self {
            inner: Context::new(full, None),
        }
    }

    pub fn hash(&mut self, header: Vec<u8>) -> Vec<u8> {
        let bytes = Bytes::from(header);

        let mut output = [0u8; 32];
        fish_hash::hash(&mut output, &mut self.inner, bytes.as_ref());

        output.to_vec()
    }
}