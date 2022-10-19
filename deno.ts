/// <deno-types path='./pkg/wasm_tennis.d.ts'/>
import {create_game, add_point} from './pkg/wasm_tennis.js';
enum Player {
    Home = 'Home',
    Oponent = 'Oponent',
}
const game: FullGame = create_game();
console.log(create_game());
console.log({Player});
console.log(add_point(game, Player.Home))
