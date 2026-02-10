use bevy::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

use crate::tier::Tier;
use crate::ui::UILoadingEnded;
use crate::ui::UnrankedContainer;


// Card component, to be attached to card entities
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub file_name: String,
    pub path: String,
    pub tier: Option<Tier>,
}


#[derive(Resource, Default)]
pub struct SelectedCard(pub bool);


pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<SelectedCard>()
            .init_resource::<CardStore>()
            .init_resource::<Dirty>()
            .add_systems(Startup, load_cards)
            .add_systems(Update, auto_save_system)
            .add_systems(Startup, spawn_cards.after(UILoadingEnded))
            ;

    }
}

fn load_cards(mut store: ResMut<CardStore>) {
    store.load_from_json("assets/cards.json");
}


#[derive(Resource, Default)]
pub struct CardStore {
    pub cards: Vec<Card>,
}

impl CardStore {
    pub fn load_from_json(&mut self, path: &str) {
        let file = File::open(path).expect("Failed to open cards.json");
        let reader = BufReader::new(file);
        self.cards = serde_json::from_reader(reader).expect("Invalid cards.json");
    }
}


// Dirty means the CardStore has unsaved changes
#[derive(Resource, Default)]
pub struct Dirty(pub bool);


use std::io::BufWriter;

// not fucntional code + dirty is diirty
fn auto_save_system(
    mut dirty: ResMut<Dirty>,
    store: Res<CardStore>,
) {
    if !dirty.0 { return; }

    let path = "assets/cards.json";
    if let Ok(file) = File::create(path) {
        let writer = BufWriter::new(file);
        if let Err(e) = serde_json::to_writer_pretty(writer, &store.cards) {
            error!("Auto-save failed: {}", e);
        } else {
            info!("Auto-save completed!");
            dirty.0 = false; // reset dirty
        }
    } else {
        error!("Cannot create cards.json for auto-save");
    }
}







fn spawn_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    store: Res<CardStore>,
    tier_containers: Query<(Entity, &Tier)>,
    unranked: Query<Entity, With<UnrankedContainer>>,
) {
    let unranked_entity = match unranked.single() {
        Ok(e) => e,
        Err(_) => commands.spawn(Node { ..default() }).id(),
    };

    for card in &store.cards {
        let parent = match &card.tier {
            Some(tier) => tier_containers
                .iter()
                .find(|(_, tc)| tc.clone().label() == tier.clone().label()) // TODO remove .label() while addint ParialEq to Tier
                .map(|(e, _)| e)
                .unwrap_or(unranked_entity),
            None => unranked_entity,
        };

        spawn_card_view(
            &mut commands,
            &asset_server,
            parent,
            card,
        );
    }
}


fn spawn_card_view(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent: Entity,
    card: &Card,
) {
    let image_handle = asset_server.load(format!(
        "thumbs/{}",
        card.path
    ));

    let card_entity = commands
        .spawn((
            // CardView,
            // CardId(card.id.clone()),
            Button,
            ImageNode {
                image: image_handle.clone(),
                ..default()
            },

            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect::all(Val::Px(4.0)),
                ..default()
            },
        ))
        // .observe(rotate_on_drag2)
        .id();

    commands.entity(parent).add_child(card_entity);
}
