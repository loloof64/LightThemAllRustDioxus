use dioxus::prelude::*;
mod components;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { 
            components::Board {
                cols_count: 3,
                onwon: |_| println!("You won !"),
            }
         }
    ))
}