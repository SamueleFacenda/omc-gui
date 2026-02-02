use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;
use common_game::components::resource::BasicResourceType::Carbon;
use common_game::components::resource::ComplexResourceType::Diamond;

use super::ecs::components::{ButtonActions, DropdownButton, DropdownItem, DropdownLabel, DropdownList, DropdownRoot,
                             Edge, ExplorerOnlyButton, LogText, PlanetOnlyButton, UiExplorerText, UiPlanetText};
use super::ecs::events::Scroll;
use super::ecs::resources::{EntityClickRes, ExplorerInfoRes, GameState, OrchestratorResource, PlanetInfoRes};
use crate::gui::types;
use crate::orchestrator::OrchestratorManualAction::{GenerateBasic, GenerateComplex, MoveExplorer, SendAsteroid,
                                                    SendSunray};

pub(crate) fn draw_game_options_menu(mut commands: Commands) {
    let root = Node {
        width: Val::Px(350.),
        height: Val::Percent(100.0),
        // Right aligned
        justify_content: JustifyContent::FlexEnd,
        margin: UiRect { left: Val::Auto, ..default() },
        ..default()
    };

    let side_menu_container =
        (BackgroundColor { 0: Color::Srgba(Srgba { red: 0.12, green: 0.18, blue: 0.24, alpha: 0.7 }) }, Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        });

    let button_row = Node { width: Val::Percent(100.0), flex_direction: FlexDirection::Row, ..default() };

    let log_square = (BackgroundColor(Color::Srgba(Srgba { red: 0., green: 0., blue: 0., alpha: 0.6 })), Node {
        flex_direction: FlexDirection::Column,
        align_self: AlignSelf::Stretch,
        height: Val::Percent(50.),
        overflow: Overflow::scroll_y(),
        ..default()
    });

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
                TextFont { font_size: 12., ..default() },
                TextLayout { justify: Justify::Center, ..default() },
                TextColor(Color::srgb(0.97, 0.98, 0.96))
            )]
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
                parent.spawn((button_factory(Text::new("Restart")), ButtonActions::StartGame));

                //4b. button 2
                parent.spawn((button_factory(Text::new("Blind")), ButtonActions::Blind));
            });

            parent.spawn(button_row.clone()).with_children(|parent| {
                //4a. button 1
                parent.spawn((button_factory(Text::new("Nuke")), ButtonActions::Nuke));

                //4b. button 2
                parent.spawn((button_factory(Text::new("Explorer Messages")), ButtonActions::StopGame));
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

    let side_menu_container =
        (BackgroundColor { 0: Color::Srgba(Srgba { red: 0.12, green: 0.18, blue: 0.18, alpha: 0.8 }) }, Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        });

    let button_row = Node {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    };

    let title_text = (Text::new("Selected Entity:"), TextFont { font_size: 32., ..default() });

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
                TextFont { font_size: 12., ..default() },
                TextLayout { justify: Justify::Center, ..default() },
                TextColor(Color::srgb(0.97, 0.98, 0.96))
            )]
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
                parent.spawn((Text::new(""), Visibility::Hidden, PlanetOnlyButton, UiPlanetText::Id));
                parent.spawn((Text::new(""), Visibility::Hidden, PlanetOnlyButton, UiPlanetText::Status));
                parent.spawn((Text::new(""), Visibility::Hidden, PlanetOnlyButton, UiPlanetText::Energy));
                parent.spawn((Text::new(""), Visibility::Hidden, PlanetOnlyButton, UiPlanetText::Rocket));
                parent.spawn((Text::new(""), Visibility::Hidden, ExplorerOnlyButton, UiExplorerText::Id));
                parent.spawn((Text::new(""), Visibility::Hidden, ExplorerOnlyButton, UiExplorerText::Status));
                parent.spawn((Text::new(""), Visibility::Hidden, ExplorerOnlyButton, UiExplorerText::Visiting));
                parent.spawn((Text::new(""), Visibility::Hidden, ExplorerOnlyButton, UiExplorerText::ResourceBag));
            });

            parent
                .spawn((
                    button_row.clone(),
                    Visibility::Hidden, //only in the beginning
                    PlanetOnlyButton
                ))
                .with_children(|parent| {
                    parent.spawn((button_factory(Text::new("Send asteroid")), ButtonActions::ManualAsteroid));
                    parent.spawn((button_factory(Text::new("Send sunray")), ButtonActions::ManualSunray));
                });

            parent.spawn(button_row.clone()).with_children(|parent| {
                parent.spawn((
                    button_factory(Text::new("Make basic resource")),
                    ButtonActions::CreateBasic,
                    Visibility::Hidden, //only in the beginning
                    ExplorerOnlyButton
                ));
                parent.spawn((
                    button_factory(Text::new("Make complex resource")),
                    ButtonActions::CreateComplex,
                    Visibility::Hidden, //only in the beginning
                    ExplorerOnlyButton
                ));
                parent
                    .spawn((
                        Node { width: Val::Px(220.0), flex_direction: FlexDirection::Column, ..default() },
                        Visibility::Hidden,
                        ExplorerOnlyButton,
                        DropdownRoot
                    ))
                    .with_children(|parent| {
                        // Button
                        parent
                            .spawn((
                                Button,
                                Node {
                                    height: Val::Px(32.0),
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::horizontal(Val::Px(8.0)),
                                    ..default()
                                },
                                DropdownButton
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("Select destination"),
                                    TextFont { font_size: 16.0, ..Default::default() },
                                    DropdownLabel
                                ));
                            });

                        parent.spawn((
                            Node { flex_direction: FlexDirection::Column, ..default() },
                            BackgroundColor(Color::Srgba(Srgba::new(0.15, 0.15, 0.15, 1.))),
                            DropdownList
                        ));
                    });
            });
        });
    });
}

pub fn populate_dropdown(
    mut commands: Commands,
    edges: Query<&Edge>,
    list: Single<Entity, With<DropdownList>>,
    explorer_status: Res<ExplorerInfoRes>,
    target_entity: Res<EntityClickRes> // or however you store it
) {
    if target_entity.explorer.is_none() || !target_entity.is_changed() {
        return;
    }

    commands.entity(*list).despawn_children();
    let explorer_id = target_entity.explorer.unwrap();
    let planet_id = explorer_status.map.get_current_planet(&explorer_id);

    let mut neighbors = Vec::new();

    for edge in edges {
        if edge.connects.0 == planet_id {
            neighbors.push(edge.connects.1);
        } else if edge.connects.1 == planet_id {
            neighbors.push(edge.connects.0);
        }
    }

    neighbors.sort_unstable();
    neighbors.dedup();

    commands.entity(*list).with_children(|parent| {
        for planet_id in neighbors {
            parent
                .spawn((
                    Button,
                    Node {
                        height: Val::Px(28.0),
                        padding: UiRect::horizontal(Val::Px(8.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    DropdownItem { planet_id, explorer_id }
                ))
                .with_children(|item| {
                    item.spawn((Text::new(format!("Planet {}", planet_id)), TextFont {
                        font_size: 14.0,
                        ..Default::default()
                    }));
                });
        }
    });
}

pub(crate) fn button_hover(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
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
    mut state: ResMut<GameState>
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::StartGame =>
                    if state.set_if_neq(GameState::Playing) {
                        orchestrator.orchestrator.set_mode_auto();
                        info!("game started");
                    },
                ButtonActions::StopGame =>
                    if state.set_if_neq(GameState::Paused) {
                        println!("game should pause now...");
                    },
                ButtonActions::Blind => {
                    if state.set_if_neq(GameState::Override) {
                        info!("entering manual override mode");
                        orchestrator.orchestrator.set_mode_manual();
                    }

                    let targets = orchestrator.orchestrator.get_alive_planets();

                    println!("targets: {:?}", targets);

                    for planet_id in targets {
                        orchestrator.orchestrator.schedule_manual_action(SendSunray { planet_id });
                    }

                    println!("done sending sunrays");
                }
                ButtonActions::Nuke => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }

                    let targets = orchestrator.orchestrator.get_alive_planets();

                    for planet_id in targets {
                        orchestrator.orchestrator.schedule_manual_action(SendAsteroid { planet_id })
                    }
                }
                ButtonActions::CreateBasic => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }

                    // handled in manual explorer action
                }
                ButtonActions::CreateComplex => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }

                    // handled in manual explorer action
                }
                _ => {}
            }
        }
    }
}

pub fn update_planet_buttons_visibility(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<PlanetOnlyButton>>
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

pub fn update_explorer_buttons_visibility(
    selected: Res<EntityClickRes>,
    mut query: Query<&mut Visibility, With<ExplorerOnlyButton>>
) {
    if !selected.is_changed() {
        return;
    }

    for mut visibility in &mut query {
        if selected.explorer.is_some() {
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
    mut state: ResMut<GameState>
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::ManualAsteroid => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }
                    if let Some(planet_id) = selected_planet.planet {
                        orchestrator.orchestrator.schedule_manual_action(SendAsteroid { planet_id });
                    }
                }
                ButtonActions::ManualSunray => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }
                    if let Some(planet_id) = selected_planet.planet {
                        orchestrator.orchestrator.schedule_manual_action(SendSunray { planet_id });
                    }
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn manual_explorer_action(
    mut action_query: Query<(&Interaction, &ButtonActions), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    selected_entity: Res<EntityClickRes>,
    explorer_status: Res<ExplorerInfoRes>,
    planet_status: Res<PlanetInfoRes>,
    mut state: ResMut<GameState>
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::CreateBasic => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }

                    log::info!("scheduling basic resource generation");

                    if let Some(explorer_id) = selected_entity.explorer {
                        let planet_id = explorer_status.map.get_current_planet(&explorer_id);
                        if let Some(planet) = planet_status.map.get_info(planet_id) {
                            let basic_resources = types::get_planet_basic_resources(&planet.name);
                            if !basic_resources.is_empty() {
                                let idx = rand::random::<i32>() as usize % basic_resources.len(); // get a random one

                                orchestrator.orchestrator.schedule_manual_action(GenerateBasic {
                                    explorer_id,
                                    resource: basic_resources[idx]
                                });
                            }
                        }
                    }
                }
                ButtonActions::CreateComplex => {
                    if state.set_if_neq(GameState::Override) {
                        orchestrator.orchestrator.set_mode_manual();
                    }

                    if let Some(explorer_id) = selected_entity.explorer {
                        let planet_id = explorer_status.map.get_current_planet(&explorer_id);
                        if let Some(planet) = planet_status.map.get_info(planet_id) {
                            let basic_resources = types::get_planet_complex_resources(&planet.name);
                            if !basic_resources.is_empty() {
                                let idx = rand::random::<i32>() as usize % basic_resources.len(); // get a random one

                                orchestrator.orchestrator.schedule_manual_action(GenerateComplex {
                                    explorer_id,
                                    resource: basic_resources[idx]
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn explorer_move_action(
    mut action_query: Query<(&Interaction, &DropdownItem), (Changed<Interaction>, With<Button>)>,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut state: ResMut<GameState>
) {
    for (&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            if state.set_if_neq(GameState::Override) {
                orchestrator.orchestrator.set_mode_manual();
            }
            orchestrator.orchestrator.schedule_manual_action(MoveExplorer {
                explorer_id: action.explorer_id,
                destination_planet_id: action.planet_id
            });
        }
    }
}

/// Injects scroll events into the UI hierarchy.
pub(crate) fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands
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

pub(crate) fn on_scroll_handler(mut scroll: On<Scroll>, mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. { scroll_position.x >= max_offset.x } else { scroll_position.x <= 0. };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. { scroll_position.y >= max_offset.y } else { scroll_position.y <= 0. };

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
