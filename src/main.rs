use dioxus::prelude::*;


fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("./style.css")] }
        div { 
            Board {cols_count: 5}
         }
    ))
}

#[derive(PartialEq, Props)]
struct BoardProps {
    cols_count: u8,
}

#[allow(non_snake_case)]
fn Board(cx: Scope<BoardProps>) -> Element {
    let lights_lit_size = (cx.props.cols_count * cx.props.cols_count) as usize;
    let lights_lit = use_ref(&cx, || vec![false; lights_lit_size]);
    let size_px = (cx.props.cols_count as u16) * 100;

    cx.render(rsx!(
        div {
            class: "board",
            width: "{size_px}px",
            height: "{size_px}px",
            grid_template: format_args!("repeat({0}, 1fr) / repeat({0}, 1fr)", cx.props.cols_count),

            lights_lit.read().iter().map(|lit| cx.render(rsx!(
                div {
                    class: "cell",
                    Light{lit: *lit}
                }
            )))
        }
    ))
}

#[derive(PartialEq, Props)]
struct LightProps {
    lit: bool
}

#[allow(non_snake_case)]
fn Light(cx: Scope<LightProps>) -> Element {
    let image = if cx.props.lit
        {
            "src/assets/vectors/light_on.svg"
        }
        else {
            "src/assets/vectors/light_off.svg"
        };
    cx.render(rsx!(
        img {
            class: "light",
            src: format_args!("{}", image),
        }
    ))
}