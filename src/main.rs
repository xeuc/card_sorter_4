use bevy::prelude::*;

mod app;
mod ui;
mod tier;
mod scroll;
mod card;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(app::TierListAppPlugin)
        .run()
        ;
}