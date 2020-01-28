let rustPromise = import("../pkg/index.js");
let wasmPromise = import ("../pkg/index_bg.wasm");


function handleFiles() {
    const file = this.files[0];
    const reader = new FileReader();
    reader.onload = function () {
        console.log("res", reader.result.length);
    }
    reader.readAsBinaryString(file);
}
document.getElementById("file_upload").addEventListener("change", handleFiles, false);

const doShade = async () => {

    let rust = await rustPromise;
    let wasm = await wasmPromise;

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

    var row, col;
    var index = 0;
    for (row = 0; row < height; row++) {
        for (col = 0; col < width; col++) {
            // imageDataArray[index++] = 256.0 * col / width;
            color = 256.0 * col / width;
            imageDataArray[4 * index + 0] = color;
            imageDataArray[4 * index + 1] = color;
            imageDataArray[4 * index + 2] = color;
            imageDataArray[4 * index + 3] = 255;
            index++;
        }
    }

    for (row = height / 4 ; row < 3*height/4; row++) {
        index = 4 * row * height + 4 * width / 4;
        imageDataArray[index + 0] = 0;
        imageDataArray[index + 1] = 0;
        imageDataArray[index + 2] = 0;
        imageDataArray[index + 4 * width/2 + 0] = 0;
        imageDataArray[index + 4 * width/2 + 1] = 0;
        imageDataArray[index + 4 * width/2 + 2] = 0;
    }
    for (col = width / 4; col < 3 * width / 4; col++) {
        row = height / 4;
        index = 4 * row * height + 4 * col;
        imageDataArray[index + 0] = 0;
        imageDataArray[index + 1] = 0;
        imageDataArray[index + 2] = 0;
        row = 3 * height / 4;
        index = 4 * row * height + 4 * col;
        imageDataArray[index + 0] = 0;
        imageDataArray[index + 1] = 0;
        imageDataArray[index + 2] = 0;


    }


    canvasImageData.data.set(imageDataArray);
    ctx.clearRect(0, 0, width, height);

    ctx.putImageData(canvasImageData, 0, 0);

    rust.save_image(imageDataArray);


    // make a box around canvas to more easily see it
    ctx.beginPath();
    ctx.lineWidth = "1";
    ctx.strokeStyle = "red";
    ctx.rect(0, 0, 100, 100);
    ctx.stroke();
};

const doBlur = async () => {

    let rust = await rustPromise;
    let wasm = await wasmPromise;

    rust.blur_all();

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

doShade().then(_ => {
    setInterval(doBlur,10000);
}
);
