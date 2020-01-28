let rustPromise = import("../pkg/index.js");
let wasmPromise = import ("../pkg/index_bg.wasm");


// handle upload and launch bluring process
function fileUpload() {
  const reader = new FileReader();
  reader.onload = function (e1) {
    var image = new Image();
    image.onload = function(e2) {
      setupBlurring(image);
    };
    image.src = reader.result;
  }
  reader.readAsDataURL(this.files[0]);
}
document.getElementById("file_upload").addEventListener("change", fileUpload, false);


// saves the image into rust's memory and launches
// a blurring task on a timer
async function setupBlurring(image) {
    const width = image.width;
    const height = image.height;

    let rust = await rustPromise;

    const canvasElement = document.querySelector("canvas");
    canvasElement.width = width;
    canvasElement.height = height;
    const ctx = canvasElement.getContext("2d");


    ctx.drawImage(image, 0, 0, width, height);

    const canvasImageData = ctx.getImageData(0, 0, width, height);
    console.log('Saving image', canvasImageData);
    rust.save_image(canvasImageData.data);
    console.log('Start blur');
    setInterval(doBlur,500);
}


// Runs the blurring task in rust
const doBlur = async () => {
    console.log('In doBlur');

    let rust = await rustPromise;
    let wasm = await wasmPromise;

    // in rust run the blurring code
    rust.blur_all();

    // get a handle on the canvas element
    const canvasElement = document.querySelector("canvas");
    const ctx = canvasElement.getContext("2d");
    const width = canvasElement.width;
    const height = canvasElement.height;

    // read in the rust memory of the manipulated image
    let rustMemory = new Uint8Array(wasm.memory.buffer);
    let bufferPointer = rust.get_output_buffer_pointer();
    const imageDataArray = rustMemory.slice(
        bufferPointer,
        bufferPointer + width * height * 4
    );

    // write that image data into the canvas element
    const canvasImageData = ctx.createImageData(width, height);
    canvasImageData.data.set(imageDataArray);
    ctx.clearRect(0, 0, width, height);
    ctx.putImageData(canvasImageData, 0, 0);

};
