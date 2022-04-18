use dioxus::events::MouseEvent;
use dioxus::prelude::*;

#[derive(Props)]
pub struct LightProps<'a> {
    lit: bool,
    onclick: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn Light<'a>(cx: Scope<'a, LightProps>) -> Element<'a> {
    let image = if cx.props.lit {
        "src/assets/vectors/light_on.svg"
    } else {
        "src/assets/vectors/light_off.svg"
    };
    cx.render(rsx!(img {
        class: "light",
        src: format_args!("{}", image),
        onclick: |evt| cx.props.onclick.call(evt),
        style { [include_str!("./light.css")] },
    }))
}
