use wasm_bindgen_test::*;
// use wasm_bindgen::JsValue;
// use wasm_tennis::create_game;
// use tennis_lib::FullGame;


#[wasm_bindgen_test]
fn pass() {
    let num: u8 = 0 + 1;
    assert_eq!(1, num);
}

// #[wasm_bindgen_test]
// fn create_new_game() {
//       if let Ok(full_game) = create_game() {
//
//     println!("full_game: {:?}", full_game);
//     assert_eq!(1, 1+1);
//       }
// }
