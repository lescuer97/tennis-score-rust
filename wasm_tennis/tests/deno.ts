/// <deno-types path='../../pkg/wasm_tennis.d.ts'/>
/// <deno-types types='../../pkg/wasm_tennis.d.ts'/>
// @deno-types='../../pkg/wasm_tennis.d.ts'/>
import {create_game, add_point,roll_back_to_last_point, Stage} from '../../pkg/wasm_tennis.js';

import {assertEquals} from "https://deno.land/std@0.160.0/testing/asserts.ts";

    enum Player {
    Home = 'Home',
        Oponent = 'Oponent',
}
    //     enum Stage {
    //         Normal = 'Normal',
    //             Deuce = 'Deuce',
    //             TieBreak = 'TieBreak',
    //    }
    Deno.test('create a new game', () => {
        const game = create_game();

        assertEquals(game, {
            game_score: {
                score: {
                    home: 0,
                    oponent: 0,
                },
                sets: [
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                ],
                // this is the "normal Stage"
                stage: Stage["0"],
            },
            list_history: [],
        })
    })
    Deno.test('add point to player Home', () => {
        let game = create_game();

        game = add_point(game,Player.Home); 

        assertEquals(game, {
            game_score: {
                score: {
                    home: 15,
                    oponent: 0,
                },
                sets: [
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                ],
                // this is the "normal Stage"
                stage: Stage["0"],
            },
            list_history: [{
                score: {
                    home: 0,
                    oponent: 0,
                },
                sets: [
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                ],
                // this is the "normal Stage"
                stage: Stage["0"],
            }],
        })
    })

    Deno.test('pop score from history', () => {
        let game = {
            game_score: {
                score: {
                    home: 15,
                    oponent: 0,
                },
                sets: [
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                ],
                // this is the "normal Stage"
                stage: Stage["0"],
            },
            list_history: [{
                score: {
                    home: 0,
                    oponent: 0,
                },
                sets: [
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                ],
                // this is the "normal Stage"
                stage: Stage["0"],
            }],
        };

        game = roll_back_to_last_point(game);


        assertEquals(game, {
            game_score: {
                score: {
                    home: 0,
                    oponent: 0,
                },
                sets: [
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                    {
                        finished: false,
                        home: 0,
                        oponent: 0,
                    },
                ],
                // this is the "normal Stage"
                stage: Stage["0"],
            },
            list_history: [],
        });
    });
