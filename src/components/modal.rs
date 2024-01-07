use leptos::*;
use stylers::{ style_str};
use crate::components::app::{Game, GameState};

#[component]
pub fn Modal <T>(
    header: T,
    #[prop(default = String::from("--purple"))]
    bg_color: String,
    #[prop(default = false )]
    overlay: bool,
    #[prop(default = false )]
    check_button: bool,
    children: Children,
) -> impl IntoView
where T: IntoView {
    let (class_name, style_val) = style_str!("modal",
        .modal {
            width: 100%;
            height: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
            position: fixed;
        }
        .modal-content {
            width: 480px;
            min-height: 430px;
            background-color: bg_color;
            border-radius: 40px;
            box-shadow: 0 0 10px 0 --black;
            padding: 20px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-between;
            border: 3px solid var(--black);
            position: relative;
            border-bottom: 10px solid var(--black);
        }
        .modal-header {
            width: 100%;
            flex:1;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .modal-body {
            width: 100%;
            flex: 4;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-direction: column;
        }
        .modal-footer button {
            width: 64px;
            height: 64px;
            position: absolute;
            bottom: -32px;
            left: 50%;
            cursor:pointer;
            margin-left: -32px;
            text-align: center;
            background-color: var(--pink);
            border: 3px solid var(--black);
            border-radius: 50%;
            border-bottom: 6px solid var(--black);
        }
        .modal-footer button:hover{
           border-color: var(--purple-light);
        },
        bg_color
    );

    let setter = use_context::<WriteSignal<Game>>().expect("No context found");
    view! {class=class_name,
        <style>{style_val}</style>
        <div class="modal">
            <div class="modal-content" style={format!("background-color: var({})", bg_color)}>
                <div class="modal-header" style={format!("background-color: {}", bg_color)}>
                    {header}
                </div>
                <div class="modal-body">
                    {children()}
                </div>
                {if check_button {

                    view! {
                        class=class_name,
                        <div class="modal-footer">
                            <button
                                on:click={move |_| {setter.update(|value| value.switch_state(GameState::Menu));
                            }}>
                                <img src="assets/check.svg" />
                            </button>
                        </div>
                    }
                } else {
                    view! {<div></div>}
                }}
            </div>
        </div>
    }
}
