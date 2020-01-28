use wasm_bindgen::prelude::*;
use web_sys::console;
use std::format;
// use js_sys::buffer;
use fastblur::gaussian_blur;



#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const IMAGE_SIZE_WIDTH: usize = 640;
const IMAGE_SIZE_HEIGHT: usize = 426;

// times 4 for the four colors (r,g,b,a)
const OUTPUT_BUFFER_SIZE: usize = IMAGE_SIZE_WIDTH * IMAGE_SIZE_HEIGHT * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];



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
    let msg1 = format!("Saving image {}", buffer.len());
    console::log_1(&JsValue::from_str(&msg1));
    unsafe {
        // for index in 0..100 * 100 * 4 {
        for (index, val) in buffer.iter().enumerate() {
            if index >= OUTPUT_BUFFER_SIZE {
                break;
            }
            // let msg = format!("Saving image {} {}", index, val);
            // console::log_1(&JsValue::from_str(&msg));
            OUTPUT_BUFFER[index] = *val;
        }
    }
}


#[wasm_bindgen]
pub fn blur_all() {
  log("Start blur");
  let mut data = make_rgb_vector();
  gaussian_blur(&mut data, IMAGE_SIZE_WIDTH, IMAGE_SIZE_HEIGHT, 1.0);
  push_rgb_vector_into_buffer(data);
}

fn log(s: &str) {
    console::log_1(&JsValue::from_str(s));
}


fn push_rgb_vector_into_buffer(data : Vec<[u8;3]>) {
  for (index, value) in data.iter().enumerate() {
    set_tri_pixel(index as i32, *value);
  }
}


// uploaded images are in RGBa format, the 4th value always being 255
fn make_rgb_vector() -> Vec<[u8;3]> {
  let mut data: Vec<[u8; 3]> = Vec::new();
  for row in 0..IMAGE_SIZE_HEIGHT {
    for col in 0..IMAGE_SIZE_WIDTH {
      data.push(get_tri_pixel(col as i32, row as i32));
    }
  }
  data
}

fn get_tri_pixel(col: i32, row: i32) -> [u8;3]{
  let index = 4 * (row * IMAGE_SIZE_WIDTH as i32 + col);
  unsafe {
    [
      OUTPUT_BUFFER[(index + 0) as usize],
      OUTPUT_BUFFER[(index + 1) as usize],
      OUTPUT_BUFFER[(index + 2) as usize]
    ]
  }
}


fn set_tri_pixel(index: i32, value: [u8;3]) {
  unsafe {
        OUTPUT_BUFFER[(4 * index+0) as usize] = value[0];
        OUTPUT_BUFFER[(4 * index+1) as usize] = value[1];
        OUTPUT_BUFFER[(4 * index+2) as usize] = value[2];
  }
}




#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Hello world!"));
    Ok(())
}
