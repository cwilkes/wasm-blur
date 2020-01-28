let rustPromise = import("../pkg/index.js");
let wasmPromise = import ("../pkg/index_bg.wasm");


function handleFiles() {
    const reader = new FileReader();
    reader.onload = function (e1) {
        var image = new Image();
        image.onload = function(e2) {
            doShade(image);
        };
        image.src = reader.result;
    }
    // reader.readAsBinaryString(this.files[0]);
    reader.readAsDataURL(this.files[0]);
}
document.getElementById("file_upload").addEventListener("change", handleFiles, false);

async function doShade(image) {
    console.log('In doShade');
    const width = image.width;
    const height = image.height;

    let rust = await rustPromise;
    let wasm = await wasmPromise;

    let rustMemory = new Uint8Array(wasm.memory.buffer);

    let bufferPointer = rust.get_output_buffer_pointer();

    const canvasElement = document.querySelector("canvas");
    const ctx = canvasElement.getContext("2d");

    canvasElement.width = width;
    canvasElement.height = height;

    ctx.drawImage(image, 0, 0, width, height);

    const imageDataArray = rustMemory.slice(
        bufferPointer,
        bufferPointer + width * height * 4
    );

    // const canvasImageData = ctx.createImageData(width, height);
    const canvasImageData = ctx.getImageData(0, 0, width, height);

    // canvasImageData.data.set(imageDataArray);
    // ctx.clearRect(0, 0, width, height);

    // ctx.putImageData(canvasImageData, 0, 0);

    console.log('Saving image', canvasImageData);
    //  Uint8ClampedArray(1090560)
    rust.save_image(canvasImageData.data);

    // make a box around canvas to more easily see it
    ctx.beginPath();
    ctx.lineWidth = "1";
    ctx.strokeStyle = "red";
    ctx.rect(0, 0, width, height);
    ctx.stroke();

    console.log('Start blur')
    // setInterval(doBlur,500);
    doBlur();
    console.log('end blur')

};

const doBlur = async () => {
    console.log('In doBlur');

    let rust = await rustPromise;
    let wasm = await wasmPromise;

    try {
        rust.blur_all();
    } catch (err) {
        console.log("Error in blur all ", err.message);
    }

    let rustMemory = new Uint8Array(wasm.memory.buffer);

    let bufferPointer = rust.get_output_buffer_pointer();

    const canvasElement = document.querySelector("canvas");
    const ctx = canvasElement.getContext("2d");
    const width = canvasElement.width;
    const height = canvasElement.height;
    const imageDataArray = rustMemory.slice(
        bufferPointer,
        bufferPointer + width * height * 4
    );
    const canvasImageData = ctx.createImageData(width, height);

    canvasImageData.data.set(imageDataArray);
    ctx.clearRect(0, 0, width, height);

    ctx.putImageData(canvasImageData, 0, 0);

};
//
// doShade().then(_ => {
//     setInterval(doBlur,10000);
// }
// );
