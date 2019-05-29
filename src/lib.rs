use js_sys::Error;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

#[wasm_bindgen]
pub struct Scene {
    ctx: WebGlRenderingContext,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen]
    pub fn draw(&self) -> Result<(), JsValue> {
        // The problem appears when there are `include_str!` macro and
        // `.ok_or_else...` part

        // commenting `frag_src` statement alone will return part of the error information
        let frag_src = include_str!("frag.glsl");

        // commenting `positions_buffer` statement alone will return all the required information
        let positions_buffer = self.ctx
            .create_buffer()
            .ok_or_else(|| Error::new("failed to create buffer"))?;

        let n = 1u32;
        test_types(n);

        Ok(())
    }
}

fn test_types(_a: i32) {}
