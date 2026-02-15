// use bevy::ecs::observer::Trigger;
use bevy::prelude::*;


use crate::{card::{Card, CardStore, Dirty, SelectedCardUnused, UserSelection}, tier::{SelectedContainer, Tier}};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup.in_set(UILoadingEnded))
            .add_systems(Startup, add_unranked_container_bevavior.after(UILoadingEnded))
            .add_systems(Startup, add_ranked_container_bevavior.after(UILoadingEnded))
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
            height: Val::Percent(90.0),
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
            width: Val::Percent(10.0),
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
            width: Val::Percent(90.0),
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
            height: Val::Percent(10.0),
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
        BigCardFullShowArea,
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


fn add_unranked_container_bevavior(
    mut commands: Commands,
    unranked: Query<Entity, With<UnrankedContainer>>,
) {
    let unranked_entity = match unranked.single() {
        Ok(e) => e,
        Err(_) => return,
    };


    commands
        .entity(unranked_entity)
        .observe(|mut event: On<Pointer<Click>>, mut selected_card: ResMut<SelectedCardUnused>, mut dirty: ResMut<Dirty>, mut commands: Commands, mut user_selection: ResMut<UserSelection>,| {
            info!("Unranked Container Clicked");
            let tier_entity = event.entity;
            // Let the containe know it has been clicked
            // Making it Querry-able
            // commands.entity(tier_entity).insert(SelectedContainer);

            if user_selection.card.is_none() {
                info!("No card selected, click ignored");
                return;
            }

            user_selection.container = Some(tier_entity);
            event.propagate(false);

            // info!("Selected card: {:?}", selected_card.0);
            // let Some(selected_card_entity) = selected_card.0.clone() else { return; };
            // commands.entity(tier_entity).add_child(selected_card_entity);
            // event.propagate(false);
            // dirty.0 = true;
            // selected_card.0 = None;
        })
    ;
}

fn add_ranked_container_bevavior(
    mut commands: Commands,
    mut store: ResMut<CardStore>,
    tier_query: Query<(Entity, &Tier), With<Tier>>,
    card_query: Query<(Entity, &Card), With<Card>>,
) {
    for (tier_entity, tier) in &tier_query {

        commands
            .entity(tier_entity)
            .observe(|mut event: On<Pointer<Click>>, mut selected_card_res: ResMut<SelectedCardUnused>, mut dirty: ResMut<Dirty>, mut commands: Commands, mut store: ResMut<CardStore>,tier_query: Query<(Entity, &Tier), With<Tier>>, mut user_selection: ResMut<UserSelection>,| {
                info!("Ranked Container Clicked");
                let tier_entity = event.entity;
                // Let the containe know it has been clicked
                // Making it Querry-able
                // commands.entity(tier_entity).insert(SelectedContainer);

                if user_selection.card.is_none() {
                    info!("No card selected, click ignored");
                    return;
                }

                user_selection.container = Some(tier_entity);
                event.propagate(false);

                // info!("Selected card: {:?}", selected_card_res.0);
                // let Some(selected_card) = selected_card_res.0.clone() else { return; };
                // let tier_entity = event.entity;

                // let Some((card_entity, _)) = card_query
                //     .iter()
                //     .find(|(_, card)| card.id == selected_card.id)
                // else { return; };

                // commands.entity(tier_entity).add_child(card_entity);

                // event.propagate(false);

                // // I don't have card id I have card entity which doen't help to found the id to fill that tier
                // // I don't have the tier neither => Acually I manage to have
                // if let Some(card) = store.cards.iter_mut().find(|c| c.id == selected_card.id) {
                //     card.tier = Some(tier.clone());
                // }
                // dirty.0 = true;
                // selected_card_res.0 = None;
            })
        ;

    }


}


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UILoadingEnded;

#[derive(Component)]
pub struct BigCardFullShowArea;