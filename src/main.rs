use dioxus::prelude::*;
mod components;

use rand::Rng;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let scramble_iterations = 10;
    let game_won = use_state(&cx, || false);
    let lights_lit_state = use_ref(&cx, || {
        let mut vector =  vec![false; 9];
        scramble(&mut vector, 10);
        vector
    });
    cx.render(rsx! (
        div { 
            button {
                onclick: move |_| {
                    game_won.modify(|_| false);
                    lights_lit_state.write().fill(false);
                    scramble(&mut lights_lit_state.write(), scramble_iterations);
                },
                "New game"
            }
            components::Board {
                cols_count: 3,
                lights_lit_state: (*lights_lit_state.read()).clone(),
                ontoggle: |evt: components::Toggle| {
                    if *game_won.current() { return; }
                    toggle_light(&mut lights_lit_state.write(), evt.0);
                    if is_game_won(&lights_lit_state.read()) {
                        game_won.modify(|_| true)
                    }
                },
            }
            components::WonLabel {game_won: *game_won.current()},
         }
    ))
}

fn toggle_light(lights_state: &mut Vec<bool>, index: usize) {
    let state_len = lights_state.len() as u16;
    let side_size = (state_len as f64).sqrt() as usize;
    assert_eq!(
        (side_size * side_size) as u16,
        state_len,
        "lights_state's length should be a square"
    );

    let col = index % side_size;
    let row = index / side_size;

    lights_state[index] = !lights_state[index];
    if row > 0 {
        lights_state[index - side_size] = !lights_state[index - side_size];
    }
    if row < (side_size - 1) {
        lights_state[index + side_size] = !lights_state[index + side_size];
    }
    if col > 0 {
        lights_state[index - 1] = !lights_state[index - 1];
    }
    if col < (side_size - 1) {
        lights_state[index + 1] = !lights_state[index + 1];
    }
}

fn scramble(mut lights_state: &mut Vec<bool>, iterations: u8) {
    let lights_count = lights_state.len();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        toggle_light(&mut lights_state, rng.gen_range(0..lights_count));
    }
}

fn is_game_won(lights_state: &Vec<bool>) -> bool {
    lights_state.iter().all(|&elt| elt == true)
}