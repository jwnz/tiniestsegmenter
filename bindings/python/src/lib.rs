use pyo3::prelude::*;
use tiniestsegmenter as ts;

#[pyfunction]
fn tokenize<'a>(py: Python, s: &'a str) -> Vec<&'a str> {
    // I don't the best way to handle the unlikely event a ts::TokenizeError pops up,
    // so for the time being, we can just unwrap and hope for the best.
    py.allow_threads(|| ts::tokenize(s).unwrap())
}

#[pymodule]
#[pyo3(name = "tiniestsegmenter")]
fn _tiniestsegmenter(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;

    Ok(())
}
