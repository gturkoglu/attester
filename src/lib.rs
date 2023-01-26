use nsm_io::{Request, Response};
use pyo3::{exceptions, prelude::*, types::PyBytes};
use serde_bytes::ByteBuf;

/// Attester is a Python module that provides a Python API for the NSM
/// service.
#[pyfunction]
fn get_attestation_doc(py: Python, user_data: String, public_key: String) -> PyResult<&PyBytes> {
    let nsm_fd = nsm_driver::nsm_init();

    let request = Request::Attestation {
        user_data: Some(ByteBuf::from(user_data)),
        public_key: Some(ByteBuf::from(public_key)),
        nonce: None,
    };

    match nsm_driver::nsm_process_request(nsm_fd, request) {
        Response::Attestation {
            document: attestation_doc,
        } => {
            let py_attestation_doc = PyBytes::new(py, attestation_doc.as_slice());
            // Return PyResult
            Ok(py_attestation_doc)
        }
        _ => Err(PyErr::new::<exceptions::PyException, _>(
            "Error during processing request",
        )),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn attester(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_attestation_doc, m)?)?;
    Ok(())
}
