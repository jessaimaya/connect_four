use crate::components::app::{Game, GameState};
use crate::components::button::Button;
use crate::components::modal::Modal;
use leptos::*;

#[component]
pub fn Start() -> impl IntoView {
    view! {
        <Modal
            header=view!{<img src="assets/logo.svg" />}
            bg_color="--purple-light".to_string()
            overlay=true
        >
                <Button
                    text="Play vs player".to_string()
                    to_state=GameState::Playing
                    with_icon=true
                    bg_color="--yellow".to_string()
                    color="--black".to_string()
                />
                <Button text="Game Rules".to_string() to_state=GameState::Rules />
        </Modal>
    }
}
