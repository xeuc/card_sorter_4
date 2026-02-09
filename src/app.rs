use bevy::prelude::*;

// use crate::card::CardPlugin;
use crate::ui::UiPlugin;
use crate::scroll::ScrollPlugin;
// use crate::ui::card_view::CardViewPlugin;

pub struct TierListAppPlugin;

impl Plugin for TierListAppPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<CardStore>()
            // .init_resource::<Dirty>()

            // === Load Cards from cards.json ===
            // .add_systems(Startup, load_cards)

            // === UI ===
            .add_plugins(UiPlugin)
            .add_plugins(ScrollPlugin)

            // .add_plugins(InteractionPlugin)

            // === Cards ? ===
            // .add_plugins(CardPlugin)
            // .add_plugins(CardViewPlugin)

            // === Auto Save ===
            // .add_systems(Update, auto_save_system)

            ;

    }
}

// TODO to move
// fn load_cards(mut store: ResMut<CardStore>) {
//     store.load_from_json("assets/cards.json");
// }


// use std::fs::File;
// use std::io::BufWriter;

// TODO to move
// fn auto_save_system(
//     mut dirty: ResMut<Dirty>,
//     store: Res<CardStore>,
// ) {
//     if !dirty.0 { return; }

//     let path = "assets/cards.json";
//     if let Ok(file) = File::create(path) {
//         let writer = BufWriter::new(file);
//         if let Err(e) = serde_json::to_writer_pretty(writer, &store.cards) {
//             error!("Auto-save failed: {}", e);
//         } else {
//             info!("Auto-save completed!");
//             dirty.0 = false; // reset dirty
//         }
//     } else {
//         error!("Cannot create cards.json for auto-save");
//     }
// }


