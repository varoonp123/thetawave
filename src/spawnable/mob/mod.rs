use bevy::prelude::*;
use bevy_rapier2d::geometry::Group;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::MobAssets,
    game::GameParametersResource,
    loot::ConsumableDropListType,
    misc::Health,
    spawnable::{InitialMotion, MobType, SpawnableBehavior, SpawnableComponent},
    states::{AppStateComponent, AppStates},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

mod behavior;
mod mob_segment;
pub use self::{behavior::*, mob_segment::*};

use super::behavior_sequence::MobBehaviorSequenceType;
use super::MobSegmentType;

/// Core component for mobs
#[derive(Component)]
pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
    /// Mob specific behaviors
    pub behaviors: Vec<behavior::MobBehavior>,
    /// behaviors that mob segments attached to the mob will perform, given the mobs current behavior
    pub mob_segment_behaviors:
        Option<HashMap<MobBehavior, HashMap<MobSegmentType, Vec<MobSegmentBehavior>>>>,
    pub behavior_sequence: Option<MobBehaviorSequenceType>,
    pub behavior_sequence_tracker: Option<BehaviorSequenceTracker>,
    /// Optional mob spawn timer
    pub mob_spawn_timer: Option<Timer>,
    /// Optional weapon timer
    pub weapon_timer: Option<Timer>,
    /// Damage dealt to other factions through attacks
    pub attack_damage: f32,
    /// Damage dealt to other factions on collision
    pub collision_damage: f32,
    /// Damage dealt to defense objective, after reaching bottom of arena
    pub defense_damage: f32,
    /// Health of the mob
    pub health: Health,
    /// List of consumable drops
    pub consumable_drops: ConsumableDropListType,
}

impl From<&MobData> for MobComponent {
    fn from(mob_data: &MobData) -> Self {
        MobComponent {
            mob_type: mob_data.mob_type.clone(),
            behaviors: mob_data.mob_behaviors.clone(),
            behavior_sequence: mob_data.behavior_sequence_type.clone(),
            mob_segment_behaviors: mob_data.mob_segment_behaviors.clone(),
            behavior_sequence_tracker: None,
            mob_spawn_timer: None,
            weapon_timer: None,
            attack_damage: mob_data.attack_damage,
            collision_damage: mob_data.collision_damage,
            defense_damage: mob_data.defense_damage,
            health: mob_data.health.clone(),
            consumable_drops: mob_data.consumable_drops.clone(),
        }
    }
}

pub struct BehaviorSequenceTracker {
    pub timer: Timer,
    pub index: usize,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct MobData {
    /// Type of mob
    pub mob_type: MobType,
    /// List of spawnable behaviors that are performed
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// Behavior sequence type
    pub behavior_sequence_type: Option<MobBehaviorSequenceType>,
    /// List of mob behaviors that are performed
    pub mob_behaviors: Vec<behavior::MobBehavior>,
    /// behaviors that mob segments attached to the mob will perform, given the mobs current behavior
    pub mob_segment_behaviors:
        Option<HashMap<MobBehavior, HashMap<MobSegmentType, Vec<MobSegmentBehavior>>>>,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// Maximum speed that can be accelerated to
    pub speed: Vec2,
    /// Angular acceleration stat
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    pub angular_deceleration: f32,
    /// Maximum angular speed that can be accelerated to
    pub angular_speed: f32,
    /// Motion that the mob initializes with
    pub initial_motion: InitialMotion,
    /// Dimensions of the mob's hitbox
    pub colliders: Vec<ColliderData>,
    /// Texture
    pub animation: AnimationData,
    /// Optional data describing the thruster
    pub thruster: Option<ThrusterData>,
    /// Damage dealt to other factions through attacks
    pub attack_damage: f32,
    /// Damage dealt to other factions on collision
    pub collision_damage: f32,
    /// Damage dealt to defense objective, after reaching bottom of arena
    pub defense_damage: f32,
    /// Health of the mob
    pub health: Health,
    /// List of consumable drops
    pub consumable_drops: ConsumableDropListType,
    /// Z level of the mobs transform
    pub z_level: f32,
    pub mob_segment_anchor_points: Option<Vec<MobSegmentAnchorPointData>>,
}

#[derive(Deserialize, Clone)]
pub struct ColliderData {
    pub dimensions: Vec2,
    pub rotation: f32,
    pub position: Vec2,
}

pub type CompoundColliderData = (Vec2, f32, Collider);

impl Into<CompoundColliderData> for ColliderData {
    fn into(self) -> CompoundColliderData {
        (
            self.position,
            self.rotation,
            Collider::cuboid(self.dimensions.x, self.dimensions.y),
        )
    }
}

#[derive(Deserialize, Clone)]
pub struct MobSegmentAnchorPointData {
    pub position: Vec2,
    pub mob_segment_type: MobSegmentType,
    pub joint: JointType,
    pub target_pos: f32,
    pub stiffness: f32,
    pub damping: f32,
}

#[derive(Deserialize, Clone)]
pub enum JointType {
    Revolute,
}

/// Event for spawning mobs
pub struct SpawnMobEvent {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// Position to spawn mob
    pub position: Vec2,

    pub rotation: Quat,
}

/// Spawns mobs from events
pub fn spawn_mob_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnMobEvent>,
    mob_resource: Res<MobsResource>,
    mob_segments_resource: Res<MobSegmentsResource>,
    mob_assets: Res<MobAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        spawn_mob(
            &event.mob_type,
            &mob_resource,
            &mob_segments_resource,
            &mob_assets,
            event.position,
            event.rotation,
            &mut commands,
            &game_parameters,
        );
    }
}

/// Data describing thrusters
#[derive(Deserialize)]
pub struct ThrusterData {
    /// Y offset from center of entity
    pub y_offset: f32,
    /// Texture
    pub animation: AnimationData,
}

/// Stores data about mob entities
#[derive(Resource)]
pub struct MobsResource {
    /// Mob types mapped to mob data
    pub mobs: HashMap<MobType, MobData>,
    /// Mob types mapped to their texture and optional thruster texture
    pub texture_atlas_handle:
        HashMap<MobType, (Handle<TextureAtlas>, Option<Handle<TextureAtlas>>)>,
}

/// Spawn a mob entity
pub fn spawn_mob(
    mob_type: &MobType,
    mob_resource: &MobsResource,
    mob_segments_resource: &MobSegmentsResource,
    mob_assets: &MobAssets,
    position: Vec2,
    rotation: Quat,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    // Get data from mob resource
    let mob_data = &mob_resource.mobs[mob_type];

    // create mob entity
    let mut mob = commands.spawn_empty();

    mob.insert(SpriteSheetBundle {
        texture_atlas: mob_assets.get_mob_asset(mob_type),
        transform: Transform {
            translation: position.extend(mob_data.z_level),
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            rotation,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(AnimationComponent {
        timer: Timer::from_seconds(mob_data.animation.frame_duration, TimerMode::Repeating),
        direction: mob_data.animation.direction.clone(),
    })
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Velocity {
        angvel: if let Some(random_angvel) = mob_data.initial_motion.random_angvel {
            thread_rng().gen_range(random_angvel.0..=random_angvel.1)
        } else {
            0.0
        },
        ..Default::default()
    })
    .insert(Collider::compound(
        mob_data
            .colliders
            .iter()
            .map(|collider_data| collider_data.clone().into())
            .collect::<Vec<CompoundColliderData>>(),
    ))
    .insert(Friction::new(1.0))
    .insert(Restitution {
        coefficient: 1.0,
        combine_rule: CoefficientCombineRule::Max,
    })
    .insert(CollisionGroups {
        memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
        filters: Group::ALL ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
    })
    .insert(MobComponent::from(mob_data))
    .insert(SpawnableComponent::from(mob_data))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(AppStateComponent(AppStates::Game))
    .insert(Name::new(mob_data.mob_type.to_string()));

    // spawn thruster as child if mob has thruster
    if let Some(thruster) = &mob_data.thruster {
        mob.with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture_atlas: mob_assets.get_thruster_asset(mob_type).unwrap(),
                    transform: Transform::from_xyz(0.0, thruster.y_offset, -1.0),
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(
                        thruster.animation.frame_duration,
                        TimerMode::Repeating,
                    ),
                    direction: thruster.animation.direction.clone(),
                })
                .insert(Name::new("Thruster"));
        });
    }

    let mob_entity = mob.id().clone();
    // add mob segment if anchor point
    if let Some(anchor_points) = mob_data.mob_segment_anchor_points.clone() {
        for anchor_point in anchor_points.iter() {
            // spawn mob_segment

            let mob_segment_data =
                &mob_segments_resource.mob_segments[&anchor_point.mob_segment_type];

            // create joint
            let joint = match &anchor_point.joint {
                JointType::Revolute => RevoluteJointBuilder::new()
                    .local_anchor1(anchor_point.position)
                    .local_anchor2(mob_segment_data.anchor_point)
                    .motor_position(
                        anchor_point.target_pos,
                        anchor_point.stiffness,
                        anchor_point.damping,
                    ),
            };

            spawn_mob_segment(
                &mob_segment_data.mob_segment_type,
                mob_entity,
                &joint,
                mob_segments_resource,
                mob_assets,
                position,
                anchor_point.position,
                commands,
                game_parameters,
            )
        }
    }
}
