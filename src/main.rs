use dioxus::prelude::*;
mod components;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let game_won = use_state(&cx, || false);
    cx.render(rsx! (
        div { 
            components::WonLabel {game_won: *game_won.current()},
            components::Board {
                cols_count: 3,
                onwon: |_| game_won.modify(|_| true),
            }
         }
    ))
}