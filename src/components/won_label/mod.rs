use dioxus::prelude::*;
use rust_i18n::t;

#[derive(PartialEq, Props)]
pub struct WonLabelProps {
    game_won: bool,
}

#[allow(non_snake_case)]
pub fn WonLabel(cx: Scope<WonLabelProps>) -> Element {
    if cx.props.game_won {
        let won_label = t!("game.won");
        cx.render(rsx!(p {
            class: "won_label",
            style { [include_str!("./style.css")] },
            [won_label]
        }))
    }
    else {
        cx.render(rsx!(div {

        }))
    }
}