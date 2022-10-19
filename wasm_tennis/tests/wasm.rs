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
//     const full_game: JsValue = match create_game() {
//     Ok(game) => game,
//     _ => println!("problem converting"),
//     };
//     assert_eq!(1, 2);
// }
