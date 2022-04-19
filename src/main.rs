use dioxus::prelude::*;

use rust_i18n;
rust_i18n::i18n!("locales");

use rust_i18n::t;

use std::cell::Cell;

mod components;
mod game_core;
use components::*;
use dioxus_desktop::tao::dpi::{
    LogicalSize,
    Size::{self, Logical},
};
use game_core::*;

fn main() {
    rust_i18n::set_locale("es");
    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|w| {
            w.with_title(t!("app.title"))
                .with_resizable(false)
                .with_inner_size(Size::new(Logical(LogicalSize {
                    width: 600.0,
                    height: 700.0,
                })))
        })
    });
}

fn app(cx: Scope) -> Element {
    let scramble_iterations = 10;
    // side_lights_count must not trigger interface update on change.
    let side_lights_count: &Cell<u16> = cx.use_hook(|_| Cell::new(3));
    let game_won = use_state(&cx, || false);
    let lights_lit_state = use_ref(&cx, || {
        let mut vector = vec![true; 9];
        scramble(&mut vector, 10);
        vector
    });

    let select_options = (2..=6).map(move |count| {
        rsx!(
            option {
                value: format_args!("{}", count),
                selected: format_args!("{}", count as u16 == side_lights_count.get()),
                "{count}"
            }
        )
    });

    let config_label = t!("game.lights_per_side");
    let new_label = t!("game.new");

    cx.render(rsx! (
        div {
            div {
                class: "level_line",
                [config_label],
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
                        lights_lit_state.write().resize(new_size, true);
                        lights_lit_state.write().fill(true);
                        scramble(&mut lights_lit_state.write(), scramble_iterations);
                        game_won.modify(|_| false);
                    },
                    [new_label]
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
