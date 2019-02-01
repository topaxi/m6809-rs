use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Registers {
    #[wasm_bindgen(readonly)]
    pub ix: u16,
    #[wasm_bindgen(readonly)]
    pub iy: u16,

    #[wasm_bindgen(readonly)]
    pub su: u16,
    #[wasm_bindgen(readonly)]
    pub ss: u16,

    #[wasm_bindgen(readonly)]
    pub pc: u16,

    #[wasm_bindgen(readonly)]
    pub aa: u8,
    #[wasm_bindgen(readonly)]
    pub ab: u8,

    #[wasm_bindgen(readonly)]
    pub dp: u8,
    #[wasm_bindgen(readonly)]
    pub cc: u8,
}
