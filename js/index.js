
const runWasm = async() => {
    let rust = await import("../pkg/index.js");
    let wasm = await import ("../pkg/index_bg.wasm");

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

    const canvasImageData = ctx.createImageData( width, height );

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


    canvasImageData.data.set(imageDataArray);
    ctx.clearRect(0, 0, width, height);

    ctx.putImageData(canvasImageData, 0, 0);

    // make a box around canvas to more easily see it
    ctx.beginPath();
    ctx.lineWidth = "1";
    ctx.strokeStyle = "red";
    ctx.rect(0, 0, 100, 100);
    ctx.stroke();
};

runWasm();
