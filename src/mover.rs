use bevy::{prelude::*, state::commands};

use crate::{card::{Card, CardStore, Dirty, UserSelection}, tier::Tier};


pub struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_card_system)
            ;

    }
}



fn _move_card_system(
    mut commands: Commands,
    mut user_selection: ResMut<UserSelection>,
    mut dirty: ResMut<Dirty>,
    mut store: ResMut<CardStore>,


    // mut commands: Commands,
    // mut store: ResMut<CardStore>,
    // mut selected: ResMut<SelectedCard>,
    // mut dirty: ResMut<Dirty>,

    // tier_query: Query<(Entity, &Interaction, &TierContainer), Changed<Interaction>>,
    // card_query: Query<(Entity, &CardId), With<CardView>>,
) {
    let Some(container_entity) = user_selection.container else { return; };
    let Some(card_entity) = user_selection.card else { return; };

    info!("Moving card {:?} to container {:?}", card_entity, container_entity);

    // Move in hierarchy
    commands.entity(container_entity).add_child(card_entity);

    // Update card component
    // ????????????????

    dirty.0 = true;

    user_selection.card = None;
    user_selection.container = None;







    // let Some(selected_id) = selected.card_id.clone() else { return; };

    // for (tier_entity, interaction, tier_container) in &tier_query {

    //     let Some((card_entity, _)) = card_query
    //         .iter()
    //         .find(|(_, id)| id.0 == selected_id)
    //     else { return; };

    //     // Update card component
    //     if let Some(card) = store.cards.iter_mut().find(|c| c.id == selected_id) {
    //         card.tier = Some(tier_container.tier.clone());
    //     }
    // }
}


fn move_card_system(
    mut commands: Commands,
    mut user_selection: ResMut<UserSelection>,
    mut dirty: ResMut<Dirty>,
    mut card_query: Query<&mut Card>,
    mut cards: ResMut<CardStore>,
    container_query: Query<&Tier>,
) {
    let (Some(container_entity), Some(card_entity)) =
        (user_selection.container, user_selection.card)
    else {
        return;
    };

    info!(
        "Moving card {:?} to container {:?}",
        card_entity, container_entity
    );

    // 1️⃣ Move in UI hierarchy
    commands.entity(container_entity).add_child(card_entity);

    // 2️⃣ Read container tier
    let new_tier_container = match container_query.get(container_entity) {
    Ok(tier_container) => Some(tier_container.clone()),
    Err(_) => {
        info!("Container has no TierContainer => Unranked");
        None
    }
};

    // 3️⃣ Update card business data
    let Ok(mut card) = card_query.get_mut(card_entity) else {
        warn!("Card entity has no Card component");
        return;
    };

    cards
        .cards
        .iter_mut()
        .find(|c| c.id == card.id)
        .map(|c| c.tier = new_tier_container.clone());
    // card.tier = Some(tier_container.clone());

    // 4️⃣ Mark dirty for save system
    dirty.0 = true;

    // 5️⃣ Clear selection
    user_selection.card = None;
    user_selection.container = None;
}
