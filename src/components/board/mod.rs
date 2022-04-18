use super::light::Light;
use dioxus::prelude::*;
use rand::Rng;

#[derive(Props)]
pub struct BoardProps<'a> {
    cols_count: u8,
    onwon: EventHandler<'a>
}

#[allow(non_snake_case)]
pub fn Board<'a>(cx: Scope<'a, BoardProps<'a>>) -> Element {
    let lights_lit_size = (cx.props.cols_count * cx.props.cols_count) as usize;
    let game_finished = use_state(&cx, || false);
    let lights_lit = use_ref(&cx, || scramble(vec![false; lights_lit_size], 10));
    let size_px = (cx.props.cols_count as u16) * 100;

    cx.render(rsx!(
        div {
            class: "board",
            width: "{size_px}px",
            height: "{size_px}px",
            grid_template: format_args!("repeat({0}, 1fr) / repeat({0}, 1fr)", cx.props.cols_count),
            style { [include_str!("./style.css")] },

            lights_lit.read().iter().enumerate().map(|(index, lit)| {
                cx.render(rsx!(
                    div {
                        class: "cell",
                        Light{
                            lit: *lit,
                            onclick: move |_| {
                                if *game_finished.current() { return; }
                                toggle_light(&mut lights_lit.write(), index);
                                if is_game_won(&lights_lit.read()) {
                                    game_finished.modify(|_| true);
                                    cx.props.onwon.call(());
                                }
                            },
                        }
                    }
                ))
            })
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

fn scramble(mut lights_state: Vec<bool>, iterations: u8) -> Vec<bool> {
    let lights_count = lights_state.len();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        toggle_light(&mut lights_state, rng.gen_range(0..lights_count));
    }
    lights_state
}

fn is_game_won(lights_state: &Vec<bool>) -> bool {
    lights_state.iter().all(|&elt| elt == true)
}