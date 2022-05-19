use std::panic;

use futures::StreamExt;
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};
use wasm_streams::ReadableStream;
use web_sys::Response;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn init_hooks() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}


#[wasm_bindgen]
pub async fn start(r: JsValue) {
    // javascript response into websys response
    let resp: Response = r.dyn_into().unwrap();


    // from example here https://github.com/MattiasBuelens/wasm-streams/blob/master/examples/fetch_as_stream.rs
    let raw_body = resp.body().unwrap();
    let body = ReadableStream::from_raw(raw_body.dyn_into().unwrap());
    let mut stream = body.into_stream();


    let mut v = vec![];

    log("start streaming");

    while let Some(Ok(chunk)) = stream.next().await {
        let mut x = chunk.dyn_ref::<Uint8Array>().unwrap().to_vec();
        v.append(&mut x);
    }
    log(&format!("{}", v.len()));

    log("done streaming");
}