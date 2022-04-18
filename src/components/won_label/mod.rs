use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct WonLabelProps {
    game_won: bool,
}

#[allow(non_snake_case)]
pub fn WonLabel(cx: Scope<WonLabelProps>) -> Element {
    if cx.props.game_won {
        cx.render(rsx!(p {
            class: "won_label",
            style { [include_str!("./style.css")] },
            "You've won !"
        }))
    }
    else {
        cx.render(rsx!(div {

        }))
    }
}