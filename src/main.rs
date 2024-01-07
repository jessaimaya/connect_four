mod components;

use leptos::*;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug);
    mount_to_body(||{ components::app::App });
}
