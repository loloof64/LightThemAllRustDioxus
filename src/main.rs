use dioxus::prelude::*;
use std::cell::Cell;

mod components;
mod game_core;
use components::*;
use game_core::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let scramble_iterations = 10;
    // side_lights_count must not trigger interface update on change.
    let side_lights_count: &Cell<u16> = cx.use_hook(|_| Cell::new(3));
    let game_won = use_state(&cx, || false);
    let lights_lit_state = use_ref(&cx, || {
        let mut vector = vec![false; 9];
        scramble(&mut vector, 10);
        vector
    });

    let select_options = (2..=6).map(move |count| rsx!(
        option {
            value: format_args!("{}", count),
            selected: format_args!("{}", count as u16 == side_lights_count.get()),
            "{count}"
        }
    ));

    cx.render(rsx! (
        div {
            div {
                class: "level_line",
                "Lights per side",
                select {
                    onchange: |evt| {
                        if let Ok(new_side_lights_count) = &evt.value.parse::<u16>() {
                            side_lights_count.set(*new_side_lights_count);
                        }
                    },
                    select_options
                },
                button {
                    onclick: move |_| {
                        let new_size = side_lights_count.get() as usize;
                        let new_size = new_size * new_size;
                        lights_lit_state.write().resize(new_size, false);
                        lights_lit_state.write().fill(false);
                        scramble(&mut lights_lit_state.write(), scramble_iterations);
                        game_won.modify(|_| false);
                    },
                    "New game"
                }
            }
            Board {
                cols_count: side_lights_count.get() as u16,
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
