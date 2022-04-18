use super::light::Light;
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct BoardProps {
    cols_count: u8,
}

#[allow(non_snake_case)]
pub fn Board<'a>(cx: Scope<'a, BoardProps>) -> Element {
    let lights_lit_size = (cx.props.cols_count * cx.props.cols_count) as usize;
    let lights_lit = use_ref(&cx, || vec![false; lights_lit_size]);
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
                            onclick: move |_| toggle_light(&mut lights_lit.write(), index),
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
    if col < (side_size + 1) {
        lights_state[index + 1] = !lights_state[index + 1];
    }
}
