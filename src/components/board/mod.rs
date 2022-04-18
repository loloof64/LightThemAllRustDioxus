use dioxus::prelude::*;
use super::light::Light;

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
                let lights_lit_clone = lights_lit.read().clone();
                cx.render(rsx!(
                    div {
                        class: "cell",
                        Light{
                            lit: *lit,
                            onclick: move |_| lights_lit.write()[index] = !lights_lit_clone[index],
                        }
                    }
                ))
            })
        }
    ))
}
