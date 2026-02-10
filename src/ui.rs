// use bevy::ecs::observer::Trigger;
use bevy::prelude::*;


use crate::tier::Tier;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup.in_set(UILoadingEnded))
            // .add_systems(Startup, add_card.after(setup))
            // .add_systems(Startup, add_card2.after(setup))
            ;
    }
}


/*
-------------------------------
|LABEL|X_CONTAINER |          |
|LABEL|X_CONTAINER |BIG_PRIVEW|
|UNRANKED_CONTAINER|          |
-------------------------------
*/
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera));

    // Root node, whole window
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![
            spawn_tier_list(),
            spawn_big_card_preview(),
        ],
    ))
    ;
}

// Tier list is itself splitted into 2
// - Ranked   containers (top half)    => Where all labels are, S A B.. With their respective cards
// - Unranked container  (bottom half) => Where all cards waiting to be classed, no labels, just a big list of cards
fn spawn_tier_list() -> impl Bundle {
    (
        Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            spawn_ranked_tier_list_table(),
            spawn_unranked_container(),
        ],
    )
}

// ✅⬜
// ⬜⬜
fn spawn_ranked_tier_list_table() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            ..default()
        },
        Children::spawn(SpawnIter(
            Tier::ALL.iter().map(|tier| spawn_tier_list_line(*tier))
        )),
    )
}

// LABEL | RANCKED CONTAINER
// +--------------------+
// | S | CARD CARD CARD |
// +--------------------+
fn spawn_tier_list_line(tier: Tier) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            min_height: Val::Percent(1.0/Tier::ALL.len() as f32 * 100.0),
            height: Val::Percent(1.0/Tier::ALL.len() as f32 * 100.0),
            max_height: Val::Percent(1.0/Tier::ALL.len() as f32 * 100.0),
            ..default()
        },
        // observe(| _: On<Pointer<Click>>| {
        //     info!("Clicked on tier");
        // }),
        children![
            spawn_label_container(tier),
            spawn_ranked_container(tier),

        ],
    )
}

// +-----------------+
// | ✅ | ⬜ ⬜ ⬜ |
// +-----------------+
fn spawn_label_container(tier: Tier) -> impl Bundle {
    (
        Node {
            width: Val::Percent(15.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(tier.color()),
        Text::new(tier.label()),
        // No Children, just the label
    )
}

// +-----------------+
// | ⬜ | ✅ ✅ ✅ |
// +-----------------+
fn spawn_ranked_container(tier: Tier) -> impl Bundle {
    (
        tier,
        Node {
            width: Val::Percent(85.0),
            height: Val::Percent(100.0),
            
            flex_wrap: FlexWrap::Wrap,
            padding: UiRect::all(Val::Px(8.0)),
            overflow: Overflow::scroll_y(), // n.b.
            align_content: AlignContent::FlexStart,
            ..default()
        },
        BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
        // Children (cards) will be set at runtime (when loading them from json for example)
    )
}



// ⬜⬜
// ✅⬜
fn spawn_unranked_container() -> impl Bundle {
    (
        UnrankedContainer,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_wrap: FlexWrap::Wrap,
            padding: UiRect::all(Val::Px(8.0)),
            align_content: AlignContent::FlexStart,
            overflow: Overflow::scroll_y(), // n.b.
            ..default()
        },
        BackgroundColor(Color::linear_rgb(0.05, 0.05, 0.05)),
        // Children (cards) will be set at runtime (when loading them from json for example)
    )
}


// ⬜✅
// ⬜✅
fn spawn_big_card_preview() -> impl Bundle {
    (
        // BigCardFullShowArea,
        Node {
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            overflow: Overflow::hidden(),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::linear_rgb(0.15, 0.15, 1.0)),
    )
}

// UnrankedContainer is a marker component to easily query the unranked container and add cards to it when loading them from json for example
#[derive(Component)]
pub struct UnrankedContainer;


fn add_card(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    unranked_container: Query<Entity, With<UnrankedContainer>>,
) {
    let Ok(unranked_container) = unranked_container.single() else {return};
    let image_handle = asset_server.load("AIDigitalMediaAgency_cosmic_pastels_aurora_borealis_double_ex_e0040751-cd27-4a71-8862-ebda4a1282c8_0.png");
    
    // Spawn new image
    commands.entity(unranked_container)
    .with_children(|parent| {
        for _i in 0..100 {
            parent.spawn((
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    margin: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                ImageNode {
                    image: image_handle.clone(),
                    color: Color::BLACK.into(),
                    ..default()
                },
            ))
            .observe(|mut event: On<Pointer<Click>>, mut button_color: Query<&mut BackgroundColor>| {
                info!("Image Clicked");
                if let Ok(mut background_color) = button_color.get_mut(event.event_target()) {
                    *background_color = bevy::prelude::BackgroundColor(Color::srgb(1.0, 0.0, 0.0));
                    event.propagate(false);
                }
            },)
            ;
        }
    })
    .observe(|mut event: On<Pointer<Click>>, mut button_color: Query<&mut BackgroundColor>| { // add selectedCard + replace Query by Single
        // selectedCard = None => Event click should NOT propagate to container ❌
        // From card spawn: propagate: false ❌
        // Because user would like to reach out a card, note a container,
        // There is no purpose to click a container without having a cart selected
        // Worst: It will triger the card + the container, so put the card in the
        // container it is in, then deselect the card... Basically doing nothing
        
        // selectedCard = Some => Event click should propagate to container ✅
        // From card spawn: propagate: true ✅
        // Because user would like to reach out a container, he already selected a card
        // If container full of cards, I want the user to easily reach the container anyway
        // meaning: by clicking on a card
        // BUT DO NOT RESELECT THE CARD THEN!
        // Skip the assign card to selectedCard if you click on card while already
        // having a card selected

        info!("Container Clicked");
        if let Ok(mut background_color) = button_color.get_mut(event.event_target()) {
            *background_color = bevy::prelude::BackgroundColor(Color::srgb(1.0, 0.0, 0.0));
            event.propagate(false);
        }
    },)
    ;
}

fn add_card2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Tier)>,
) {
    let Some((container, _)) = query
        .iter()
        .find(|(_, tier)| tier.index() == Tier::RARE.index()) else { return };

    let image_handle = asset_server.load("AIDigitalMediaAgency_cosmic_pastels_aurora_borealis_double_ex_e0040751-cd27-4a71-8862-ebda4a1282c8_0.png");
    
    
    // Spawn new image
    commands.entity(container)
    .with_children(|parent| {
        for _i in 0..100 {
            parent.spawn((
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
            ));
        }
    });
}



#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UILoadingEnded;