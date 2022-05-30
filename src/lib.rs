//! Expr.rs -  A safe and simple math evaluator for Rust and Python.

mod ast;
pub mod error;
mod parser;
mod formulas;

use chumsky::prelude::Parser;
use rust_decimal::prelude::ToPrimitive;

use parser::parser;

#[cfg(feature = "python")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "python")]
use pyo3::prelude::*;


/// Evaluate a string, returns the result as ``f64``
/// 
/// # Errors
/// 
/// This function will return an Error as ``String`` if the string is not a valid expression
/// or it overflowed f64 or i64.
/// 
/// # Examples
/// 
/// ``` rust
/// use expr_rs::eval;
/// 
/// println!("{}", eval("1+2").unwrap());
/// ```
pub fn eval(expr: &str) -> Result<f64, String> {
    let res = parser()
        .parse(expr)
        .map_err(|e| e.into_iter().map(|e| e.to_string()).collect::<String>())?
        .eval()
        .map_err(|e| format!("Evaluation error: {}", e))?
        .to_f64()
        .ok_or_else(|| "Cannot convert to f64".to_string())?;

    Ok(res)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(name = "eval")]
fn py_eval(expr: &str) -> PyResult<f64> {
    match eval(expr) {
        Ok(res) => Ok(res),
        Err(e) => Err(PyRuntimeError::new_err(e)),
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn expr_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_eval, m)?)?;

    Ok(())
}
