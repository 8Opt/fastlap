use numpy::PyArray2;
use pyo3::prelude::*;

pub mod lap;
pub mod matrix;
pub mod utils;

use crate::lap::{auction, dantzig, hungarian, lapjv, lapmod, subgradient};
use crate::matrix::*;
use crate::utils::*;

#[pyfunction]
fn solve_lap<'py>(
    _py: Python<'py>,
    cost_matrix: &Bound<'py, PyArray2<f64>>,
    algorithm: &str,
) -> PyResult<(f64, Vec<usize>, Vec<usize>)> {
    let matrix = extract_matrix(cost_matrix)?;

    match algorithm {
        "lapjv" => Ok(lapjv::solve(matrix)),
        "hungarian" => Ok(hungarian::solve(matrix)),
        "lapmod" => Ok(lapmod::solve(matrix)),
        "subgradient" => Ok(subgradient::solve(matrix)),
        "auction" => Ok(auction::solve(matrix)),
        "dantzig" => Ok(dantzig::solve(matrix)),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Unknown algorithm. Supported algorithms: {}",
            supported_algorithms().join(", ")
        ))),
    }
}

#[pyfunction]
fn get_supported_algorithms<'py>(_py: Python<'py>) -> Vec<&'py str> {
    supported_algorithms()
}

#[pymodule]
fn fastlap(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_lap, m)?)?;
    m.add_function(wrap_pyfunction!(get_supported_algorithms, m)?)?;
    Ok(())
}
