use bevy::prelude::EventReader;
use bevy::prelude::*;
use thetawave_interface::{
    run::{CyclePhaseEvent, LevelPhaseType},
    states::GameCleanup,
};

use crate::run::CurrentRunProgressResource;

use super::game::{PhaseUiComponent, TutorialPhaseUI};

#[derive(Component)]
pub struct TopMiddleLeftUI;

#[derive(Component)]
pub struct TopMiddleRightUI;

//Phase UI
#[derive(Component)]
pub struct PhaseNameUI;

#[derive(Component)]
pub struct PhaseDataUI;

#[derive(Component)]
pub struct TextPhaseObjective;

#[derive(Component)]
pub struct BossHealthUI;

#[derive(Component)]
pub struct BossHealthValueUI;

pub fn build_phase_ui(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            //background_color: Color::GREEN.with_a(0.25).into(),
            ..default()
        })
        .insert(TopMiddleLeftUI)
        .with_children(|top_middle_left_ui| {
            top_middle_left_ui
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Tutorial: Movement",
                        TextStyle {
                            font,
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(PhaseNameUI);
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                padding: UiRect::new(Val::Vw(1.0), Val::Vw(1.0), Val::Vh(2.0), Val::Vh(2.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            //background_color: Color::YELLOW.with_a(0.1).into(),
            ..default()
        })
        .insert(TopMiddleRightUI)
        .with_children(|top_middle_right_ui| {
            // Uncomment for text phase objective

            top_middle_right_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(60.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: Color::RED.with_a(0.05).into(),
                    ..default()
                })
                .insert(BossHealthUI)
                .with_children(|boss_health_ui| {
                    boss_health_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::RED.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(BossHealthValueUI);
                });

            /*
            top_middle_right_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    //background_color: Color::BLUE.with_a(0.25).into(),
                    ..default()
                })
                .insert(PhaseDataUI)
                .with_children(|phase_data_ui| {
                    let text_sections = [
                        "Up",
                        "Down",
                        "Left",
                        "Right",
                        "Up+Left",
                        "Up+Right",
                        "Down+Left",
                        "Down+Right",
                    ];

                    for section in &text_sections {
                        phase_data_ui
                            .spawn(TextBundle {
                                style: Style {
                                    height: Val::Px(30.0), // Set a fixed height for each text section
                                    ..default()
                                },
                                text: Text::from_section(
                                    section.to_string(),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Left),
                                ..default()
                            })
                            .insert(TextPhaseObjective);
                    }
                });
                */
        });
}

pub fn update_phase_ui(
    mut tutorial_ui_query: Query<&mut Text, With<TutorialPhaseUI>>,
    run_resource: Res<CurrentRunProgressResource>,
) {
    if let Some(current_level) = &run_resource.current_level {
        if let Some(current_phase) = &current_level.current_phase {
            match &current_phase.phase_type {
                thetawave_interface::run::LevelPhaseType::FormationSpawn { .. } => {}
                thetawave_interface::run::LevelPhaseType::Break { .. } => {}
                thetawave_interface::run::LevelPhaseType::Boss { .. } => {}
                thetawave_interface::run::LevelPhaseType::Tutorial {
                    tutorial_lesson, ..
                } => {
                    if let Ok(mut tutorial_text) = tutorial_ui_query.get_single_mut() {
                        for (section, progress_str) in tutorial_text
                            .sections
                            .iter_mut()
                            .zip(tutorial_lesson.get_movement_timer_strs().iter())
                        {
                            section.value = format!("{}\n", progress_str.0);

                            section.style.color = if progress_str.1 {
                                Color::GREEN
                            } else {
                                Color::WHITE
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn setup_phase_ui(
    mut commands: Commands,
    mut cycle_phase_event_reader: EventReader<CyclePhaseEvent>,
    run_resource: Res<CurrentRunProgressResource>,
    asset_server: Res<AssetServer>,
    phase_ui_query: Query<Entity, With<PhaseUiComponent>>,
) {
    /*
    for _ in cycle_phase_event_reader.iter() {
        if let Some(current_level) = &run_resource.current_level {
            if let Some(current_phase) = &current_level.current_phase {
                // remove existing ui
                for entity in phase_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                let font = asset_server.load("fonts/wibletown-regular.otf");

                // spawn the name of the phase
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(15.0),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(PhaseUiComponent)
                    .insert(GameCleanup)
                    .with_children(|parent| match &current_phase.phase_type {
                        thetawave_interface::run::LevelPhaseType::FormationSpawn { .. } => {
                            info!("Entered formation spawn phase");
                            parent.spawn(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                text: Text::from_section(
                                    "Formation Invasion",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                ..default()
                            });
                        }
                        thetawave_interface::run::LevelPhaseType::Break { .. } => {
                            info!("Entered break phase");

                            parent.spawn(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                text: Text::from_section(
                                    "Break",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                ..default()
                            });
                        }
                        thetawave_interface::run::LevelPhaseType::Boss { .. } => {
                            info!("Entered boss phase");

                            parent.spawn(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                text: Text::from_section(
                                    "Boss",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                ..default()
                            });
                        }
                        thetawave_interface::run::LevelPhaseType::Tutorial {
                            tutorial_lesson,
                            ..
                        } => {
                            info!("Entered tutorial phase");

                            parent.spawn(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                text: Text::from_section(
                                    tutorial_lesson.get_name(),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                ..default()
                            });
                        }
                    });

                // spawn tutorial ui node if in tutorial phase
                if let LevelPhaseType::Tutorial { .. } = &current_phase.phase_type {
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(18.0),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PhaseUiComponent)
                        .insert(GameCleanup)
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle {
                                    style: Style {
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    text: Text::from_sections([
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        TextSection::new(
                                            format!(""),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                    ]),
                                    background_color: Color::BLACK.with_a(0.8).into(),
                                    ..default()
                                })
                                .insert(TutorialPhaseUI);
                        });
                }
            }
        }
    }
    */
}