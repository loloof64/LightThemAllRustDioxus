use dioxus::prelude::*;
use dioxus::events::MouseEvent;


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
fn Board<'a>(cx: Scope<'a, BoardProps>) -> Element {
    let lights_lit_size = (cx.props.cols_count * cx.props.cols_count) as usize;
    let lights_lit = use_ref(&cx, || vec![false; lights_lit_size]);
    let size_px = (cx.props.cols_count as u16) * 100;

    cx.render(rsx!(
        div {
            class: "board",
            width: "{size_px}px",
            height: "{size_px}px",
            grid_template: format_args!("repeat({0}, 1fr) / repeat({0}, 1fr)", cx.props.cols_count),

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

#[derive(Props)]
struct LightProps<'a> {
    lit: bool,
    onclick: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
fn Light<'a>(cx: Scope<'a, LightProps>) -> Element<'a> {
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
            onclick: |evt| cx.props.onclick.call(evt),
        }
    ))
}