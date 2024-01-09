use crate::components::app::{Game, GameState};
use crate::components::button::MenuButton;
use leptos::*;
use std::option::Option;

#[derive(Debug, Clone, PartialEq)]
struct GamePlay {
    player1_score: u32,
    player2_score: u32,
    current_player: Player,
    time_remaining: u32,
    winner: Option<Player>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[component]
pub fn GamePlay() -> impl IntoView {
    let game = use_context::<WriteSignal<Game>>();
    view! {
        <main class="gameplay">
            <header>
                <MenuButton text="Menu".to_string() action=move || go_to_menu(game) />
                <MenuButton text="Restart".to_string() action= move || print_something() />
            </header>
        </main>
    }
}

fn go_to_menu(game: Option<WriteSignal<Game>>) {
    log::debug!("go_to_menu");
    game.expect("no writeSignal")
        .update(|value| value.switch_state(GameState::Menu));
}
fn print_something() {
    log::debug!("something");
}
