//! `thetawave` player module
use bevy::prelude::*;
use ron::de::from_bytes;

mod components;
mod resources;
mod spawn;
mod systems;

use crate::{states, GameEnterSet, GameUpdateSet};

pub use self::{
    components::PlayerComponent,
    resources::{Character, CharacterType, CharactersResource, PlayerInput, PlayersResource},
    spawn::spawn_players_system,
    systems::{
        player_ability_system, player_death_system, player_fire_weapon_system,
        player_health_system, player_movement_system, player_scale_fire_rate_system,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<CharactersResource>(include_bytes!("../../assets/data/characters.ron"))
                .unwrap(),
        );

        app.insert_resource(PlayersResource::default());

        app.add_systems(
            OnEnter(states::AppStates::Game),
            spawn_players_system.in_set(GameEnterSet::SpawnPlayer),
        );

        app.add_systems(
            Update,
            (
                player_fire_weapon_system,
                player_death_system,
                player_health_system,
                player_scale_fire_rate_system,
                player_movement_system.in_set(GameUpdateSet::Movement),
                player_ability_system.in_set(GameUpdateSet::Abilities),
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}
