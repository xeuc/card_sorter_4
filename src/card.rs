use bevy::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

use crate::tier::Tier;
use crate::ui::UILoadingEnded;
use crate::ui::UnrankedContainer;


pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedCardUnused>()
            .init_resource::<CardStore>()
            .init_resource::<Dirty>()
            .init_resource::<UserSelection>()
            .add_systems(Startup, load_cards)
            .add_systems(Update, auto_save_system)
            .add_systems(Startup, spawn_cards.after(UILoadingEnded))
            ;

    }
}

fn load_cards(mut store: ResMut<CardStore>) {
    store.load_from_json("assets/cards.json");
}





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
        info!("store.cards: {:?}", store.cards);
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
                .find(|(_, tc)| tc.label() == tier.clone().label()) // TODO remove .label() while addint ParialEq to Tier
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
            // No need of card, only need id for move_card_system()
            card.clone(),
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
        )) // observe, please use the macro + replace Query by Single:
        .observe(|mut event: On<Pointer<Click>>, mut selected_card: ResMut<SelectedCardUnused>, card_query: Query<(Entity, &Card), With<Card>>, mut commands: Commands, mut user_selection: ResMut<UserSelection>| {
            // SelectedCardUnused = None => Event click should NOT propagate to container ❌
            // From card spawn: propagate: false ❌
            // Because user would like to reach out a card, note a container,
            // There is no purpose to click a container without having a cart selected
            // Worst: It will triger the card + the container, so put the card in the
            // container it is in, then deselect the card... Basically doing nothing
            
            // SelectedCardUnused = Some => Event click should propagate to container ✅
            // From card spawn: propagate: true ✅
            // Because user would like to reach out a container, he already selected a card
            // If container full of cards, I want the user to easily reach the container anyway
            // meaning: by clicking on a card
            // BUT DO NOT RESELECT THE CARD THEN!
            // Skip the assign card to SelectedCardUnused if you click on card while already
            // having a card selected
            info!("Image Clicked");
            let card_entity = event.entity;
            // commands.entity(card_entity).insert(SelectedCard);

            if user_selection.card.is_some() {
                info!("Card already selected, Assume user want to select the container");
                event.propagate(true);
                return;
            }

            user_selection.card = Some(card_entity);
            event.propagate(false);

            // for (card_entity, card) in &card_query {
            //     if card_entity == event.entity {
            //         info!("Clicked card: {:?}", card);
            //         if selected_card.0.is_none() {
            //             selected_card.0 = Some(card.clone());
            //             event.propagate(false);
            //         }
            //         break;
            //     }
            // }

        },)
        .id();

    commands.entity(parent).add_child(card_entity);
}

// SelectedCard makes it easy to pick Card user clicked on
#[derive(Component)]
pub struct SelectedCard;




// Card component, to be attached to card entities
#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub file_name: String,
    pub path: String,
    pub tier: Option<Tier>,
}


#[derive(Resource, Default)]
pub struct SelectedCardUnused(Option<Card>);

#[derive(Resource, Default)]
pub struct UserSelection {
    pub card: Option<Entity>,
    pub container: Option<Entity>,
}


// Hmm
#[derive(Resource, Default, Debug)]
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