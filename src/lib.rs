use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::panic::catch_unwind;
use std::panic;


#[no_mangle]
pub extern fn raw_oh_noes() {
    oh_noes();
}

#[pyfunction]
pub fn oh_noes() -> PyResult<String> {
    match catch_unwind(|| {
        foo()
    }) {
        Ok(e) => e,
        Err(e) => Ok(format!("Error: {:?}", e)),
    }
}

fn foo() -> PyResult<String> {
    panic!("panic");
}

#[pymodule]
fn pyo3_backtrace_repro(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(oh_noes))?;

    Ok(())
}
