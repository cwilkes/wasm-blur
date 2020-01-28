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
pub fn save_image(buffer: Box<[u8]>) -> i32 {
  log(&format!("Saving image len {}", buffer.len()));
  for (index, val) in buffer.iter().enumerate() {
    if index >= OUTPUT_BUFFER_SIZE {
      log(&format!("Warning: reached max size of {}", index));
      return index as i32 / 4;
    }
    unsafe {
      OUTPUT_BUFFER[index] = *val;
    }
  }
  return buffer.len() as i32 / 4;
}


#[wasm_bindgen]
pub fn blur_all(width: i32, height: i32) {
  log("Start blur");
  let mut data = make_rgb_vector(width, height);
  let real_height = data.len() as i32 / width;
  gaussian_blur(&mut data, width as usize, real_height as usize, 1.0);
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
fn make_rgb_vector(width: i32, height: i32) -> Vec<[u8;3]> {
  let mut data: Vec<[u8; 3]> = Vec::new();
  for row in 0..height {
    if 4 * row * width >= OUTPUT_BUFFER_SIZE as i32 {
      break;
    }
    for col in 0..width {
      let index = 4 * (row * width as i32 + col);
      if index + 2 >= OUTPUT_BUFFER_SIZE as i32 {
        log(&format!("Reached max row {} and col {}", row, col));
        break;
      }
      unsafe {
        data.push( [
          OUTPUT_BUFFER[(index+0) as usize],
          OUTPUT_BUFFER[(index+1) as usize],
          OUTPUT_BUFFER[(index+2) as usize] ]);
      }
    }
  }
  data
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
