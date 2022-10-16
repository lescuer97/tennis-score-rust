use serde::{de::DeserializeOwned,  Serialize};
// cargo build --target wasm32-unknown-unknown && mkdir pkg/ -p &&  wasm-bindgen --out-dir pkg/  --target deno ./target/wasm32-unknown-unknown/debug/wasm_tennis.wasm
// mkdir pkg/
// wasm-bindgen --out-dir pkg/  --target deno --typescript ./target/wasm32-unknown-unknown/debug/wasm_tennis.wasm
use wasm_bindgen::prelude::*;
use tennis_lib::{FullGame, Player};


fn convert_to_rs_type<T: DeserializeOwned>(game: JsValue) -> Result<T, JsError> {

    let rust_type: T  = match serde_wasm_bindgen::from_value(game) {
        Ok(val) => val,
        Err( _err) => return Err(JsError::new("Error converting to Rust Type")),
    };
    Ok(rust_type)

}

fn convert_to_js_type<T: Serialize>(game: &T)-> Result<JsValue, serde_wasm_bindgen::Error> {

    let js_value: JsValue  = match serde_wasm_bindgen::to_value(game) {
        Ok(val) => val,
        Err( err) => return Err(err),
    };
    Ok(js_value)
}

#[wasm_bindgen]
pub fn create_game() -> Result<JsValue, JsError> {    
    let game: FullGame = FullGame::new();
    let js_game = convert_to_js_type(&game)?;
    Ok(js_game)

}
#[wasm_bindgen]
pub fn add_point(game: JsValue, who_scored: JsValue) -> Result<JsValue, JsError> {
    let full_game: FullGame=  convert_to_rs_type(game)?;
    let who_scored: Player  = convert_to_rs_type(who_scored)?;

    let game_results: FullGame = full_game.add_point(who_scored);
    let js_game = convert_to_js_type(&game_results)?;
    Ok(js_game) 
}


#[wasm_bindgen]
pub fn roll_back_to_last_point(game: JsValue) -> Result<JsValue, JsError> {

    let full_game: FullGame= convert_to_rs_type(game)?;

    let game_results: FullGame = full_game.roll_back_last_point();
    let js_game: JsValue = convert_to_js_type(&game_results)?;
    Ok(js_game) 
}


#[wasm_bindgen]
pub fn roll_back_to_specific_point(game: JsValue, index: usize) -> Result<JsValue, JsError> {
    let full_game: FullGame = convert_to_rs_type(game)?;
    let game_results: FullGame = full_game.roll_back_to_specific_point(index);
    let js_game: JsValue = convert_to_js_type(&game_results)?;
    Ok(js_game) 
}
