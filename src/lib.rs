mod schema;

// pub use schema::placeholder;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

// TODO TEST
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
///
/// Run: maturin develop --features=pybindings
#[pymodule]
#[cfg(feature = "pybindings")]
fn vioux(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use pybindings::*;

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}

#[cfg(feature = "pybindings")]
mod pybindings {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
