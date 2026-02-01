use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::ecs::components::{ButtonActions, LogText, PlanetOnlyButton, UiExplorerText, UiPlanetText};
use crate::ecs::events::Scroll;
use crate::ecs::resources::{EntityClickRes, GameState, OrchestratorResource};

pub(crate) fn draw_game_options_menu(mut commands: Commands) {
    let root = Node {
        width: Val::Px(350.),
        height: Val::Percent(100.0),
        // Right aligned
        justify_content: JustifyContent::FlexEnd,
        margin: UiRect {
            left: Val::Auto,
            ..default()
        },
        ..default()
    };

    let side_menu_container = (
        BackgroundColor {
            0: Color::Srgba(Srgba {
                red: 0.12,
                green: 0.18,
                blue: 0.24,
                alpha: 0.7,
            }),
        },
        Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
    );

    let button_row = Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        ..default()
    };

    let log_square = (
        BackgroundColor(Color::Srgba(Srgba {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.6,
        })),
        Node {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            height: Val::Percent(50.),
            overflow: Overflow::scroll_y(),
            ..default()
        },
    );

    let title_text = Text::new("Galaxy Menu");

    let button_factory = |text: Text| {
        (
            Button,
            BackgroundColor(Color::srgb(0.67, 0.30, 0.53)),
            Node {
                width: Val::Percent(50.),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(15.)),
            children![(
                text,
                TextFont {
                    font_size: 12.,
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                TextColor(Color::srgb(0.97, 0.98, 0.96))
            )],
        )
    };

    // 1. Root node
    commands.spawn(root).with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container).with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);

            // 3b. Button Row
            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((button_factory(Text::new("Start")), ButtonActions::StartGame));

                //4b. button 2
                parent.spawn((button_factory(Text::new("Pause")), ButtonActions::StopGame));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((
                    button_factory(Text::new("Restart")),
                    ButtonActions::StartGame,
                ));

                //4b. button 2
                parent.spawn((button_factory(Text::new("Blind")), ButtonActions::Blind));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((button_factory(Text::new("Nuke")), ButtonActions::Nuke));

                //4b. button 2
                parent.spawn((
                    button_factory(Text::new("Explorer Messages")),
                    ButtonActions::StopGame,
                ));
            });
            parent.spawn(log_square).with_children(|parent| {
                parent.spawn((Text::new(""), LogText));
            });
        });
    });
}

///Draws the menu that holds the list of all explorers and planets
pub(crate) fn draw_entity_info_menu(mut commands: Commands) {
    let root = Node {
        width: Val::Px(350.0),
        height: Val::Percent(100.0),
        // Left aligned
        justify_content: JustifyContent::FlexStart,
        ..default()
    };

    let side_menu_container = (
        BackgroundColor {
            0: Color::Srgba(Srgba {
                red: 0.12,
                green: 0.18,
                blue: 0.18,
                alpha: 0.8,
            }),
        },
        Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
    );

    let button_row = Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    let title_text = (
        Text::new("Selected Entity:"),
        TextFont {
            font_size: 32.,
            ..default()
        },
    );

    let button_factory = |text: Text| {
        (
            Button,
            BackgroundColor(Color::srgb(0.67, 0.30, 0.53)),
            Node {
                width: Val::Percent(50.),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(15.)),
            children![(
                text,
                TextFont {
                    font_size: 12.,
                    ..default()
                },
                TextLayout {
                    justify: Justify::Center,
                    ..default()
                },
                TextColor(Color::srgb(0.97, 0.98, 0.96))
            )],
        )
    };

    // 1. Root node
    commands.spawn(root).with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container).with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);

            // 3b. Button Row
            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((Text::new("choose a planet!"), UiPlanetText::Name));
                parent.spawn((Text::new(""), UiPlanetText::Id));
                parent.spawn((Text::new(""), UiPlanetText::Status));
                parent.spawn((Text::new(""), UiPlanetText::Energy));
                parent.spawn((Text::new(""), UiPlanetText::Rocket));
                parent.spawn((Text::new(""), UiExplorerText::Id));
                parent.spawn((Text::new(""), UiExplorerText::Status));
                parent.spawn((Text::new(""), UiExplorerText::Visiting));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((
                    button_factory(Text::new("Send asteroid")),
                    ButtonActions::ManualAsteroid,
                    Visibility::Hidden, //only in the beginning 
                    PlanetOnlyButton
                ));
                parent.spawn((
                    button_factory(Text::new("Send sunray")),
                    ButtonActions::ManualSunray,
                    Visibility::Hidden, //only in the beginning
                    PlanetOnlyButton
                ));
            });
        });
    });
}

pub(crate) fn button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (&interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                println!("Button Pressed!");
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.07, 0.30, 0.53).into();
            }
        }
    }
}

pub(crate) fn game_menu_action(
    mut action_query: Query<(&Interaction, &ButtonActions), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut state: ResMut<GameState>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::StartGame => {
                    if state.set_if_neq(GameState::Playing) {
                        info!("game started");
                    }
                }
                ButtonActions::StopGame => {
                    if state.set_if_neq(GameState::Paused) {
                        println!("game should pause now...");
                    }
                }
                ButtonActions::Blind => {
                    state.set_if_neq(GameState::Override);
                    info!("entering manual override mode");

                    let mut targets = Vec::new();
                    for id in 0..orchestrator.orchestrator.planets_info.len() {
                        if !orchestrator.orchestrator.planets_info.is_dead(&(id as u32)) {
                            targets.push(id as u32);
                        }
                    }

                    println!("targets: {:?}", targets);

                    if let Err(s) = orchestrator.orchestrator.send_sunray_from_gui(targets) {
                        error!("{}", s);
                    }

                    println!("done sending sunrays");
                }
                ButtonActions::Nuke => {
                    state.set_if_neq(GameState::Override);

                    let mut targets = Vec::new();
                    for id in 0..orchestrator.orchestrator.planets_info.len() {
                        if !orchestrator.orchestrator.planets_info.is_dead(&(id as u32)) {
                            targets.push(id as u32);
                        }
                    }

                    if let Err(s) = orchestrator.orchestrator.send_asteroid_from_gui(targets) {
                        error!("{}", s);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn update_planet_buttons_visibility(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<PlanetOnlyButton>>,
) {
    if !selected.is_changed() {
        return;
    }

    for mut visibility in &mut query {
        if selected.planet.is_some() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub(crate) fn manual_planet_action(
    mut action_query: Query<(&Interaction, &ButtonActions), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    selected_planet: Res<EntityClickRes>,
    mut state: ResMut<GameState>,
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::ManualAsteroid => {
                    state.set_if_neq(GameState::Override);
                    if let Some(id) = selected_planet.planet {
                        if let Err(e) = orchestrator.orchestrator.send_asteroid_from_gui(vec![id]) {
                            error!(e)
                        }
                    }
                }
                ButtonActions::ManualSunray => {
                    state.set_if_neq(GameState::Override);
                    if let Some(id) = selected_planet.planet {
                        if let Err(e) = orchestrator.orchestrator.send_sunray_from_gui(vec![id]) {
                            error!(e)
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

/// Injects scroll events into the UI hierarchy.
pub(crate) fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= 21.;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

pub(crate) fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            // Consume the Y portion of the scroll delta.
            delta.y = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}
