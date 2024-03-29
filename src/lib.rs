mod fish_hash_context;

use pyo3::prelude::*;
use crate::fish_hash_context::FishHashContext;

#[pymodule]
fn pyironfishlib(_py: Python, m: &PyModule) -> PyResult<()> {
    let fish_hash_module = PyModule::new(_py, "fishhash")?;
    fish_hash_module.add_class::<FishHashContext>()?;

    m.add_submodule(fish_hash_module)?;
    Ok(())
}