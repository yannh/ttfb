/*
MIT License

Copyright (c) 2021 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use serde::{Deserialize, Serialize};
use crate::error::TtfbError;
use crate::outcome::TtfbOutcome;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
pub async fn ttfb_js(input: String, allow_insecure_certificates: bool) -> Result<JsValue, JsValue> {
    match ttfb(input, allow_insecure_certificates).await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(e) => return Err(JsValue::from(&*format!("error retrieving"))),
    };
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    pub name: String,
    pub domainLookupStart: u64,
    pub domainLookupEnd: u64,
}

pub fn foo () {
    log("BAAAAAHHHH\n")
}


#[wasm_bindgen]
pub fn sleep(ms: i32) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}
pub async fn ttfb(input: String, _allow_insecure_certificates: bool) -> Result<TtfbOutcome, TtfbError> {
    let window = web_sys::window().expect("should have a window in this context");
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    log("foo");
    let url = format!("https://yann.mandragor.org/");

    let request:Request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let window = web_sys::window().unwrap();
    let performance = window
        .performance()
        .expect("performance should be available");

    JsFuture::from(sleep(1500)).await;

    let resources = performance.get_entries_by_type("resource");
    let mut a: String = "".to_string();
    let mut res: Resource = Resource{
        name: "".to_string(),
        domainLookupStart: 0,
        domainLookupEnd: 0,
    };
    for item in resources.iter() {
        let b: Resource = serde_wasm_bindgen::from_value(item).unwrap();
        log (b.name.as_str());
        if b.name == url {
            res = b.clone();
            break
        }
    }
    // Convert this other `Promise` into a rust `Future`.
    let c = serde_json::to_string(&res).unwrap();

    Ok(TtfbOutcome::new(
        "foo".to_string(),
        c,
        66,
        100,
        10,
        100,
        100,
        100,
        // http_content_download_duration,
    ))
}
