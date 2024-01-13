use crate::components::app::{Game, GameState};
use crate::components::button::MenuButton;
use leptos::*;
use std::option::Option;
use stylers::style;

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
    let styles = style! {"gameplay",
        main {
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: flex-start;
            flex-direction: column;
        }
        header {
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: space-between;
            max-width: 650px;
        }
        .board {
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: space-between;
            max-width: 1024px;
            margin-top: 50px;
            position: relative;
        }
        .grid {
            width: 100%;
            position: relative;
            max-width: 650px;
        }
        .board_black {
            top: 0;
            left: 0;
            z-index: 0;
        }
        .board_white {
            top: 0;
            left: 0;
            z-index: 1;
            position: absolute;
        }
        .player {
            display: flex;
            align-items: center;
            justify-content: center;
            flex-direction: column;
            width: 100%;
            max-width: 140px;
            position: relative;
            background-color: var(--white);
            border-radius: 20px;
            border: solid 3px var(--black);
            border-bottom: solid 10px var(--black);
        }
        .player img {
            top: 0;
            left: 50%;
            width: 100%;
            max-width: 50px;
            position: absolute;
            margin-left: -19%;
            margin-top: -25%;
        }
        .player-name {
            font-size: var(--text-small);
            font-weight: bold;
            text-transform: uppercase;
            color: var(--black);
            margin-top: 50px;
        }
        .player-score {
            font-size: var(--text-large);
            font-weight: bold;
            color: var(--black);
            margin-bottom: 10px;
        }
        footer {
            width: 100%;
            height: 200px;
            border-top-left-radius: 60px;
            border-top-right-radius: 60px;
            background-color: var(--purple);
            position: fixed;
            bottom: 0;
        }
        .timer {
            bottom: 0;
            left: 50%;
            margin-left: -100px;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-direction: column;
            width: 200px;
            z-index: 1;
            margin-bottom: -120px;
            position: absolute;
            background-color: var(--white);
            border-radius: 20px;
            border: solid 3px var(--black);
            border-bottom: solid 10px var(--black);
        }
        .timer::before {
            content: "";
            position: absolute;
            top: -10px;
            left: 50%;
            transform: translateX(-50%);
            border-left: 10px solid transparent;
            border-right: 10px solid transparent;
            border-bottom: 10px solid var(--white);
        }
        .timer-turn {
            font-size: var(--text-small);
            font-weight: bold;
            text-transform: uppercase;
            color: var(--black);
            margin-top: 50px;
        }
        .timer-value {
            font-size: var(--text-large);
            font-weight: bold;
            color: var(--black);
            margin-bottom: 10px;
        }
    };
    view! {
        class=styles,
        <main class="gameplay">
            <header>
                <MenuButton text="Menu".to_string() action=move || go_to_menu(game) />
                <img src="assets/logo.svg" alt="connect four"/>
                <MenuButton text="Restart".to_string() action= move || print_something() />
            </header>
            <section class="board">
                <div class="player">
                    <img src="assets/player-one.svg" alt="player1"/>
                    <span class="player-name">Player 1</span>
                    <span class="player-score">{0}</span>
                </div>
                <div class="grid">
                    <img src="assets/black_board.svg" alt="dashboard" class="board_black"/>
                    <img src="assets/white_board.svg" alt="dashboard" class="board_white"/>
                </div>
                <div class="player">
                    <img src="assets/player-two.svg" alt="player2"/>
                    <span class="player-name">Player 2</span>
                    <span class="player-score">{0}</span>
                </div>
                <div class="timer">
                    <span class="timer-turn">"Player 2's turn"</span>
                    <span class="timer-value">"14s"</span>
                </div>
            </section>
            <footer></footer>
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
