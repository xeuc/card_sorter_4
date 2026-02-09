// use bevy::ecs::observer::Trigger;
use bevy::prelude::*;


use crate::tier::Tier;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Startup, add_card.after(setup))
            .add_systems(Startup, add_card2.after(setup))
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
                Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(0),
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
                Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(1),
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
                Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(2),
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
                Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(3),
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
        Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(4),
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
                Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(4),
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
        Pickable { should_block_lower: true, ..Default::default() },
        GlobalZIndex(2),
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
                Pickable { should_block_lower: true, ..Default::default() },
        BackgroundColor(Color::linear_rgb(0.15, 0.15, 1.0)),
        GlobalZIndex(1),
    )
}

// UnrankedContainer is a marker component to easily query the unranked container and add cards to it when loading them from json for example
#[derive(Component)]
struct UnrankedContainer;


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
                Pickable { should_block_lower: true, ..Default::default() },
                GlobalZIndex(999),
            ))
            // .observe(update_tint_color_on::<Pointer<Over>>(Color::WHITE.into()))
            // .observe(update_tint_color_on::<Pointer<Out>>(Color::BLACK.into()))
            // .observe(update_tint_color_on::<Pointer<Press>>(Color::BLACK.into()))
            // .observe(update_tint_color_on::<Pointer<Release>>(Color::WHITE.into()))
            // .observe(
            //     |mut event: On<Pointer<Over>>,
            //         mut button_color: Single<&mut BackgroundColor>| {
            //         button_color.0 = Color::srgb(1.0, 0.5, 0.0);
            //         event.propagate(false);
            //     },
            // )
            .observe(|mut event: On<Pointer<Press>>, mut button_color: Query<&mut BackgroundColor>| {
                info!("!!!! IMAGE PRESSED");
                // button_color.0 = Color::srgb(1.0, 0.0, 0.0);
                // event.propagate(false);
                if let Ok(mut background_color) = button_color.get_mut(event.event_target()) {
                    *background_color = bevy::prelude::BackgroundColor(Color::srgb(1.0, 0.0, 0.0));
                    event.propagate(false);
                }
            },)
            ;
        }
    })
    // .observe(update_background_color_on::<Pointer<Over>>(Color::WHITE.into()))
    // .observe(update_background_color_on::<Pointer<Out>>(Color::BLACK.into()))
    // .observe(update_background_color_on::<Pointer<Press>>(Color::BLACK.into()))
    // .observe(update_background_color_on::<Pointer<Release>>(Color::WHITE.into()))
    .observe(|mut event: On<Pointer<Press>>, mut button_color: Query<&mut BackgroundColor>| {
        info!("hovered!!!!");
        // button_color.0 = Color::srgb(1.0, 0.0, 0.0);
        // event.propagate(false);
        if let Ok(mut background_color) = button_color.get_mut(event.event_target()) {
            *background_color = bevy::prelude::BackgroundColor(Color::srgb(1.0, 0.0, 0.0));
            event.propagate(false);
        }
    },)
    ;
}
/// An observer to rotate an entity when it is dragged
fn rotate_on_drag(mut event: On<Pointer<Click>>,) {
    event.propagate(false);
}

/// Returns an observer that updates the entity's color to the one specified.
fn update_background_color_on<E: EntityEvent>(
    new_color: BackgroundColor,
) -> impl Fn(On<E>, Query<&mut BackgroundColor>) {
    // An observer closure that captures `new_color`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // background_color. Instead, the event type is a generic, and the background_color is passed in.
    move |event, mut query| {
        if let Ok(mut background_color) = query.get_mut(event.event_target()) {
            *background_color = new_color;
            // event.event_target().propagate(false);
            // event.trigger().propagate(false);
            // event.entity.propagate(false);
            // event.propagate(false);
        }
    }
}

/// Returns an observer that updates the entity's color to the one specified.
fn update_tint_color_on<E: EntityEvent>(
    new_color: Color,
) -> impl Fn(On<E>, Query<&mut ImageNode>) {
    // An observer closure that captures `new_color`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // tint_color. Instead, the event type is a generic, and the tint_color is passed in.
    move |event, mut query| {
        if let Ok(mut tint_color) = query.get_mut(event.event_target()) {
            tint_color.color = new_color;
            // event.propagate(false);
        }
    }
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