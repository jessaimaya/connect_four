use crate::components::app::{Game, GameState};
use leptos::*;
use std::option::Option;
use stylers::{style, style_str};

#[component]
pub fn Button(
    text: String,
    to_state: GameState,
    #[prop(default = String::from("--white"))] bg_color: String,
    #[prop(default = String::from("--black"))] color: String,
    #[prop(default = false)] with_icon: bool,
) -> impl IntoView {
    // having bg_color as --purple I want to pass it like var(--purple) to the style! macro
    let styler_class = style! {"button",
        button {
            width: 100%;
            margin: 10px;
            max-height: 86px;
            border-radius: 20px;
            text-align: left;
            padding: 20px 30px;
            color: var(--black);
            border: 3px solid var(--black);
            border-bottom: 10px solid var(--black);
            font-size: var(--text-medium);
            font-weight: bold;
            text-transform: uppercase;
        }
        button::first-child {
            margin-top: 0;
        }
        button:hover {
            cursor: pointer;
            border-color: var(--purple-light);
        }
        .with-icon {
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
    };
    let setter = use_context::<WriteSignal<Game>>();
    match setter {
        Some(setter) => {
            if with_icon {
                view! {
                    class= styler_class,
                    <button
                        class="with-icon"
                        style={format!("background-color: var({}); color: var({})", bg_color, color)}
                        on:click={move |_| {
                          setter.update(|value| value.switch_state(to_state.clone()));
                      }}>
                        <span>{text.clone()}</span>
                        <span>{view!{<img src="assets/player-vs-player.svg" />}}</span>
                    </button>
                }
            } else {
                view! {
                    class= styler_class,
                    <button
                        style={format!("background-color: var({}); color: var({})", bg_color, color)}
                        on:click={move |_| {
                          setter.update(|value| value.switch_state(to_state.clone()));
                        }}
                    >
                        {text.clone()}
                    </button>
                }
            }
        }
        None => {
            view! {<button>{text}</button>}
        }
    }
}

#[component]
pub fn MenuButton<F>(text: String, mut action: F) -> impl IntoView
where
    F: FnMut() + 'static,
{
    let style = style! {"MenuButton",
        button {
            background-color: var(--purple);
            color: var(--white);
            border-radius: 50px;
            padding: 10px;
            text-transform: uppercase;
            text-align: center;
            border: none;
            cursor: pointer;
        }

        button:hover {
            background-color: var(--pink);
        }
    };
    view! {
        class=style,
        <button on:click=move |_| action() >
            {text}
        </button>
    }
}
