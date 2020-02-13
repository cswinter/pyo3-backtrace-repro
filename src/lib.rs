use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn oh_noes() -> PyResult<String> {
    foo()
}

fn foo() -> PyResult<String> {
    panic!("panic");
}

#[pymodule]
fn pyo3_backtrace_repro(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(oh_noes))?;

    Ok(())
}
