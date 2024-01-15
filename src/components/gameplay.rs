use crate::components::app::{Game, GameState};
use crate::components::button::MenuButton;
use leptos::wasm_bindgen::prelude::wasm_bindgen;
use leptos::wasm_bindgen::{JsCast, JsValue};
use leptos::*;
use std::option::Option;
use stylers::{style, style_str};

#[derive(Debug, Clone, PartialEq)]
pub struct GamePlay {
    player1_score: u32,
    player2_score: u32,
    current_player: Player,
    time_remaining: u32,
    winner: Option<Player>,
}

impl GamePlay {
    fn new() -> Self {
        Self {
            player1_score: 0,
            player2_score: 0,
            current_player: Player::Player1,
            time_remaining: 14,
            winner: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[wasm_bindgen]
pub fn get_grid_width(id: &str) -> Result<u32, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let grid = document.get_element_by_id(id).unwrap();
    let grid_width = grid.dyn_into::<web_sys::HtmlElement>()?.offset_width();
    Ok(grid_width as u32)
}

#[component]
pub fn GamePlay() -> impl IntoView {
    let game = use_context::<WriteSignal<Game>>();
    let game_play = GamePlay::new();
    let current_player_marker = match game_play.current_player.clone() {
        Player::Player1 => "url('assets/turn_pink.svg')",
        Player::Player2 => "url('assets/turn_yellow.svg')",
    };
    let (mouse_x, set_mouse_x) = create_signal(0);
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
            position: relative;
            max-width: 650px;
            cursor: pointer;
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
        .marker {
            width: 33px;
            height: 36px;
            position: absolute;
            top: -15px;
            background-image: url("assets/mark.svg");
            background-repeat: no-repeat;
            background-position: center;
            background-size: 100%;
            z-index: 2;
            transform:translate(-50%,-50%);
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
                <div class="grid"
                    id="grid"
                    on:mousemove={move |event| {
                        let grid_width = event.target().unwrap().dyn_into::<web_sys::HtmlElement>().unwrap().offset_width();
                        let step = grid_width / 7;
                        let new_x = ((event.offset_x() / step) * step);
                        set_mouse_x(new_x + (step / 2));
                    }}
                >
                    <div class="marker" style:left=move || format!("{:?}px", mouse_x())></div>
                    <img src="assets/black_board.svg" alt="dashboard" class="board_black"/>
                    <img src="assets/white_board.svg" alt="dashboard" class="board_white"/>
                </div>
                <div class="player">
                    <img src="assets/player-two.svg" alt="player2"/>
                    <span class="player-name">Player 2</span>
                    <span class="player-score">{0}</span>
                </div>
                <div class="timer" style={format!("background-image: {};", current_player_marker)}>
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
