mod utils;

use std::io::{BufWriter, Cursor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut input_buf = Cursor::new(input);
    let mut output_buf_writer = BufWriter::new(Vec::new());
    lzma_rs_inner::lzma_compress(&mut input_buf, &mut output_buf_writer).unwrap();
    output_buf_writer.into_inner().unwrap()
}

#[wasm_bindgen]
pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut input_buf = Cursor::new(input);
    let mut output_buf_writer = BufWriter::new(Vec::new());
    lzma_rs_inner::lzma_decompress(&mut input_buf, &mut output_buf_writer).unwrap();
    output_buf_writer.into_inner().unwrap()
}
