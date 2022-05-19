import * as pd from "./pkg/wasm_memtest.js"

self.window = self 

async function start() {
    await pd.default()
    await pd.initThreadPool(1);
    await pd.init_hooks()
}


onmessage = async function (e) {
    const res = await fetch(e.data)
    pd.start(res)
    this.postMessage("done")    
}

start()
console.log("started worker")