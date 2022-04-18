mod light;
use dioxus::prelude::*;


#[derive(Debug, Copy, Clone)]
pub struct Toggle(pub usize);

#[derive(Props)]
pub struct BoardProps<'a> {
    cols_count: u16,
    lights_lit_state: Vec<bool>,
    ontoggle: EventHandler<'a, Toggle>
}

#[allow(non_snake_case)]
pub fn Board<'a>(cx: Scope<'a, BoardProps<'a>>) -> Element {
    let count_x = cx.props.cols_count;
    let count_y = (cx.props.lights_lit_state.len() / cx.props.cols_count as usize) as u16;
    let width = count_x * 100;
    let height = count_y * 100;

    cx.render(rsx!(
        div {
            class: "board",
            width: "{width}px",
            height: "{height}px",
            grid_template: format_args!("repeat({}, 1fr) / repeat({}, 1fr)", count_x, count_y),
            style { [include_str!("./style.css")] },

            cx.props.lights_lit_state.iter().enumerate().map(|(index, lit)| {
                cx.render(rsx!(
                    div {
                        class: "cell",
                        light::Light{
                            lit: *lit,
                            onclick: move |_| {
                                cx.props.ontoggle.call(Toggle(index))
                            },
                        }
                    }
                ))
            })
        }
    ))
}