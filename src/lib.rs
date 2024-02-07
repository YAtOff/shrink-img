use std::io::Cursor;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use png::BitDepth;
use png::ColorType;
use resize::Pixel;
use resize::Type::Triangle;
use rgb::FromSlice;


fn shrink_size(src_width: usize, src_height: usize, max_width: usize, max_height: usize) -> (usize, usize) {
    let factor = f32::min(
        max_width as f32 / src_width as f32,
        max_height as f32 / src_height as f32
    );
    if factor < 1_f32 {
        (
            (src_width as f32 * factor).ceil() as usize,
            (src_height as f32 * factor).ceil() as usize
        )
    } else {
        (src_width, src_height)
    }
}

#[pyfunction]
fn shrink_png(py: Python, src_image: &[u8], max_width: usize, max_height: usize) -> PyResult<PyObject> {
    let decoder = png::Decoder::new(src_image);
    let mut reader = decoder.read_info().unwrap();
    let info = reader.info();
    let color_type = info.color_type;
    let bit_depth = info.bit_depth;
    let (src_width, src_height) = (info.width as usize, info.height as usize);
    let mut src_buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut src_buf).unwrap();

    let (dst_width, dst_height) = shrink_size(src_width, src_height, max_width, max_height);
    let mut dst_buf = vec![0u8; dst_width * dst_height * color_type.samples()];

    assert_eq!(BitDepth::Eight, bit_depth);
    match color_type {
        ColorType::Grayscale => resize::new(src_width, src_height, dst_width, dst_height, Pixel::Gray8, Triangle).unwrap().resize(src_buf.as_gray(), dst_buf.as_gray_mut()).unwrap(),
        ColorType::Rgb => resize::new(src_width, src_height, dst_width, dst_height, Pixel::RGB8, Triangle).unwrap().resize(src_buf.as_rgb(), dst_buf.as_rgb_mut()).unwrap(),
        ColorType::Indexed => unimplemented!(),
        ColorType::GrayscaleAlpha => unimplemented!(),
        ColorType::Rgba => resize::new(src_width, src_height, dst_width, dst_height, Pixel::RGBA8, Triangle).unwrap().resize(src_buf.as_rgba(), dst_buf.as_rgba_mut()).unwrap(),
    };

    let mut dst_image = Cursor::new(Vec::new());
    let mut encoder = png::Encoder::new(&mut dst_image, dst_width as u32, dst_height as u32);
    encoder.set_color(color_type);
    encoder.set_depth(bit_depth);
    encoder.write_header().unwrap().write_image_data(&dst_buf).unwrap();

    Ok( PyBytes::new(py, &dst_image.get_ref()).into())
}


#[pymodule]
fn shrink_img(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(shrink_png, m)?)?;
    Ok(())
}
