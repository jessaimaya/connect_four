use crate::components::gameplay::GamePlay;
use crate::components::rules::Rules;
use crate::components::start::Start;
use crate::theme::theme::APP_CLASS;
use leptos::*;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Game {
    state: GameState,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Menu,
        }
    }

    pub fn switch_state(&mut self, state: GameState) {
        self.state = state;
    }

    fn current_state(&self) -> GameState {
        self.state.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
    Rules,
}

#[component]
pub fn App() -> impl IntoView {
    // Create a Signal that holds your Game struct
    let (game, set_game) = create_signal(Game::new());

    provide_context(game);
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

    view! { class = APP_CLASS,
        <main class="app" >
            <Show
                when= move || {current_state() == GameState::Menu}
                fallback= || {view!{}}
            >
                <Start />
            </Show>
            <Show
                when= move || {current_state() == GameState::Rules}
                fallback= || {view!{}}
            >
                <Rules />
            </Show>
            <Show
                when= move || {current_state() == GameState::Playing}
                fallback= || {view!{}}
            >
                <GamePlay />
            </Show>
        </main>
    }
}
