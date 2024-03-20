use std::io::{Cursor, Error, ErrorKind};
use image::imageops;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::PyOSError;
use image::io::Reader;
use image::error::ImageError;
use imageops::FilterType;


fn image_error_to_py_error(image_error: ImageError) -> PyErr {
    PyOSError::new_err(format!("{}", image_error))
}

#[pyfunction]
fn resize_image_buffer(py: Python, src_image_buffer: &[u8], width: u32, height: u32) -> PyResult<PyObject> {
    let mut reader = Reader::new(Cursor::new(src_image_buffer))
        .with_guessed_format()?;
    reader.no_limits();
    let format = reader.format().ok_or(Error::new(ErrorKind::Other, "Unknown image format"))?;

    let src_image = reader.decode().map_err(image_error_to_py_error)?;
    let dst_image = src_image.resize(width, height, FilterType::Lanczos3);

    let mut dst_image_buffer = Cursor::new(Vec::new());
    dst_image.write_to(&mut dst_image_buffer, format).map_err(image_error_to_py_error)?;
    Ok(PyBytes::new(py, &dst_image_buffer.get_ref()).into())
}

#[pyfunction]
fn guess_image_format(src_image_buffer: &[u8]) -> PyResult<&'static str> {
    let reader = Reader::new(Cursor::new(src_image_buffer))
        .with_guessed_format()?;
    let format = reader.format().ok_or(Error::new(ErrorKind::Other, "Unknown image format"))?;
    Ok(format.to_mime_type())
}

#[pymodule]
fn shrink_img(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resize_image_buffer, m)?)?;
    m.add_function(wrap_pyfunction!(guess_image_format, m)?)?;
    Ok(())
}
