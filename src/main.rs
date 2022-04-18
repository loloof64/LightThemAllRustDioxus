use dioxus::prelude::*;

mod components;
mod game_core;
use components::*;
use game_core::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let scramble_iterations = 10;
    let game_won = use_state(&cx, || false);
    let lights_lit_state = use_ref(&cx, || {
        let mut vector = vec![false; 9];
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
            Board {
                cols_count: 3,
                lights_lit_state: (*lights_lit_state.read()).clone(),
                ontoggle: |evt: Toggle| {
                    if *game_won.current() { return; }
                    toggle_light(&mut lights_lit_state.write(), evt.0);
                    if is_game_won(&lights_lit_state.read()) {
                        game_won.modify(|_| true)
                    }
                },
            }
            WonLabel {game_won: *game_won.current()},
         }
    ))
}
