use dioxus::prelude::*;


fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("./style.css")] }
        div { 
            Light{ lit : false}
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