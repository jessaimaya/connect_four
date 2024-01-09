#![feature(fn_traits)]

mod components;
mod theme;

use leptos::*;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug);
    mount_to_body(|| components::app::App);
}
