use wasm_bindgen::prelude::*;
use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const IMAGE_SIZE_WIDTH: usize = 100;
const IMAGE_SIZE_HEIGHT: usize = 100;

// times 4 for the four colors (r,g,b,a)
const OUTPUT_BUFFER_SIZE: usize = IMAGE_SIZE_WIDTH * IMAGE_SIZE_HEIGHT * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

// TODO: calculate gaussian matrix    https://en.wikipedia.org/wiki/Gaussian_blur
const BLUR_SIZE: usize = 7;
const EDGE_DELTA: i32 = ((BLUR_SIZE - 1) / 2) as i32;

const BLUR: [[f32; BLUR_SIZE]; BLUR_SIZE] = [
    [0.00000067, 0.00002292, 0.00019117, 0.00038771, 0.00019117, 0.00002292, 0.00000067],
    [0.00002292, 0.00078633, 0.00655965, 0.01330373, 0.00655965, 0.00078633, 0.00002292],
    [0.00019117, 0.00655965, 0.05472157, 0.11098164, 0.05472157, 0.00655965, 0.00019117],
    [0.00038771, 0.01330373, 0.11098164, 0.22508352, 0.11098164, 0.01330373, 0.00038771],
    [0.00019117, 0.00655965, 0.05472157, 0.11098164, 0.05472157, 0.00655965, 0.00019117],
    [0.00002292, 0.00078633, 0.00655965, 0.01330373, 0.00655965, 0.00078633, 0.00002292],
    [0.00000067, 0.00002292, 0.00019117, 0.00038771, 0.00019117, 0.00002292, 0.00000067]
];

fn get_pixel(x: i32, y: i32, offset: i32) -> u8 {
    let pixel: u8;
    unsafe {
        let index = 4 * (y * IMAGE_SIZE_HEIGHT as i32 + x) + offset;
        pixel = OUTPUT_BUFFER[index as usize]
    }
    pixel
}

fn blur_pixel(x: i32, y: i32) -> Vec<u8> {
    let mut result2: Vec<u8> = Vec::new();
    for offset in 0..3 {
        let mut result = 0.0;
        for (col_index, col_delta) in (-EDGE_DELTA..EDGE_DELTA).enumerate() {
            for (row_index, row_delta) in (-EDGE_DELTA..EDGE_DELTA).enumerate() {
                result += BLUR[col_index][row_index] * get_pixel(x + col_delta, y + row_delta, offset) as f32;
            }
        }
        result2.push(result as u8);
    }
    result2.push(255);
    result2
}

#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }
    pointer
}

#[wasm_bindgen]
pub fn save_image(buffer: Box<[u8]>) {
    unsafe {
        for index in 0..100 * 100 * 4 {
            OUTPUT_BUFFER[index] = buffer[index];
        }
    }
}

#[wasm_bindgen]
pub fn blur_all() {
    blur(0, 0, IMAGE_SIZE_WIDTH as i32, IMAGE_SIZE_HEIGHT as i32)
}

fn top_bottom_fill(row_offset: i32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    for row in 0..EDGE_DELTA {
        for col in 0..IMAGE_SIZE_WIDTH {
            for offset in 0..3 {
                data.push(get_pixel(col as i32, row + row_offset, offset));
            }
            data.push(255);
        }
    }
    data
}

fn left_right_fill(row: i32, col_offset: i32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    for tmp_col in 0..EDGE_DELTA {
        for offset in 0..3 {
            data.push(get_pixel(tmp_col + col_offset, row, offset));
        }
        data.push(255);
    }
    data
}

pub fn blur(x: i32, y: i32, dx: i32, dy: i32) {
    let mut data: Vec<u8> = Vec::new();
    data.extend_from_slice(&top_bottom_fill(0));
    for row in y + EDGE_DELTA..y + dy - EDGE_DELTA {
        data.extend_from_slice(&left_right_fill(row, 0));
        for col in x + EDGE_DELTA..x + dx - EDGE_DELTA {
            data.extend_from_slice(&blur_pixel(col, row));
        }
        data.extend_from_slice(&left_right_fill(row, IMAGE_SIZE_WIDTH as i32 - EDGE_DELTA));
    }
    data.extend_from_slice(&top_bottom_fill(IMAGE_SIZE_HEIGHT as i32 - EDGE_DELTA));
    // put back into shared array
    for (index, value) in data.iter().enumerate() {
        unsafe {
            OUTPUT_BUFFER[index] = *value;
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();


    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
