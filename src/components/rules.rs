use crate::components::modal::Modal;
use leptos::*;
use stylers::style;

#[component]
pub fn Rules() -> impl IntoView {
    let styles = style! {"Rules",
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
    view! {class=styles,
        <Modal
             header=view!{class=styles, <h1 class="rules-title">Rules</h1>}
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
    }
}
