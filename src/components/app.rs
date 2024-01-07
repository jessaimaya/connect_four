use leptos::*;
use rand::Rng;
use stylers::style;
use crate::components::button::Button;
use crate::components::modal::Modal;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
    player1_score: u32,
    player2_score: u32,
    current_player: Player,
    time_remaining: u32,
    winner: Option<Player>,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Menu,
            player1_score: 0,
            player2_score: 0,
            current_player: Player::Player1,
            time_remaining: 0,
            winner: None,
        }
    }

    pub fn switch_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
    }

    fn current_player(&self) -> Option<Player> {
        match self.clone().state {
            GameState::Playing(Some(player)) => Some(player),
            _ => None,
        }
    }

    fn current_state(&self) -> GameState {
        self.state.clone()
    }

    fn get_random_player(&self) -> Player {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..2);
        match random_number {
            0 => Player::Player1,
            1 => Player::Player2,
            _ => panic!("Random number generator failed"),
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing(Option<Player>),
    Paused,
    GameOver,
    Rules,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[component]
pub fn App() -> impl IntoView {
    let styler_class = style!{"App",
        :root {
            --black: #000;
            --purple: #5c2dd5;
            --purple-light: #7945ff;
            --pink: #fd6687;
            --yellow: #ffce67;
            --white: #fff;
            --text-large: 56px;
            --text-medium: 24px;
            --text-small: 20px;
            --text-extra-small: 16px;
            --font-family: "Space Grotesk", sans-serif;
            padding: 0;
            margin: 0;
        }
        * {
            font-family: var(--font-family);
        }
        .app {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            background-color: var(--purple);
            color: var(--white);
            padding: 0;
            margin: 0;
        }
        .rules-content {
            text-align: left;
            color: var(--black);
        }
        .rules-title {
            margin: 0;
            font-size: var(--text-large);
            text-transform: uppercase;
            font-weight: bold;
            color: var(--black);
        }
        .rules-content h2 {
                font-size: var(--text-medium);
                margin: 0;
                margin-bottom: 10px;
                font-weight: bold;
                color: var(--purple);
                text-transform: uppercase;
        }
        ol {
            list-style-type: "none";
            counter-reset: item;
            padding: 0;
        }
        ol li {
            padding-left: 20px;
        }
        ol li::before {
            content:counter(item);
            counter-increment: item;
            font-weight: bold;
            margin-right: 12px;
            margin-left: -20px;

        }
    };
        // Create a Signal that holds your Game struct
        let (game, set_game) = create_signal(Game::new());

        provide_context(set_game);

        // Use the `set` method to update the state of your game
        let switch_state = move |state: GameState| {
            set_game.update(|game| game.switch_state(state));
        };

        // Use the `get` method to access the current state of your game
        let current_state = move || game.get().current_state();

        create_effect(move |_| {
            log::debug!("Current state: {:?}", current_state());
        });

        view! { class = styler_class,
        <main class="app">
            <Show
                when= move || {current_state() == GameState::Menu}
                fallback= || {view!{

                }}
            >
                <Modal
                    header=view!{<img src="assets/logo.svg" />}
                    bg_color="--purple-light".to_string()
                    overlay=true
                >
                        <Button
                            text="Play vs player".to_string()
                            to_state=GameState::Playing(Some(game.get().get_random_player()))
                            with_icon=true
                            bg_color="--yellow".to_string()
                            color="--black".to_string()
                        />
                        <Button text="Game Rules".to_string() to_state=GameState::Rules />
                </Modal>
            </Show>
            <Show
                when= move || {current_state() == GameState::Rules}
                fallback= || {view!{
                }}
            >
            <Modal
                header=view!{class=styler_class, <h1 class="rules-title">Rules</h1>}
                bg_color="--white".to_string()
                overlay=true
                check_button=true
            >
                <div class="rules-content">
                    <h2>Objective</h2>
                    <p>Be the first player to connect 4 of the same colored discs in a row (either vertically, horizontally, or diagonally).</p>
                    <h2>How to play</h2>
                    <ol>
                        <li>Red goes first in the first game.</li>
                        <li>Players must alternate turns, and only one disc can be dropped in each turn.</li>
                        <li>The game ends when there is a 4-in-a-row or a stalemate.</li>
                        <li>The starter of the previous game goes second on the next game.</li>
                    </ol>
                </div>
            </Modal>
            </Show>
        </main>
    }


}
