use bevy::prelude::*;

use crate::game::{GameState, PlanetClickRes};

#[derive(Component)]
pub enum ButtonActions {
    StartGame,
    StopGame
}

pub(crate) fn draw_game_options_menu(
    mut commands: Commands,
) {
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
        BackgroundColor{
            0: Color::Srgba(Srgba { red: 0.12, green: 0.18, blue: 0.18, alpha: 0.8 })
        },
        Node {
            width: Val::Px(350.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
    });

    let button_row = Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        };

    let title_text = Text::new("Galaxy Menu");

    let button = (Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)));
    
    // 1. Root node
    commands.spawn(root)
        .with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container)
        .with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);
            
            // 3b. Button Row 
            parent.spawn(button_row)
            .with_children(|parent| {
                
                //4a. button 1
                parent.spawn((button.clone(), ButtonActions::StartGame))
                .with_children(|parent| {
                    // 5. Button text
                    parent.spawn(Text::new("Start Game"));
                });

                //4a. button 2
                parent.spawn((button.clone(), ButtonActions::StopGame))
                .with_children(|parent| {
                    // 5. Button text
                    parent.spawn(Text::new("Stop Game"));
                });
            });
        });
    });
}

///Draws the menu that holds the list of all explorers and planets
pub(crate) fn draw_selection_menu(
    mut commands: Commands,
    selected_planet: Res<PlanetClickRes>
) {

    let root = Node {
        width: Val::Px(350.0),
        height: Val::Percent(100.0),
        // Left aligned
        justify_content: JustifyContent::FlexStart, 
        ..default()
    };

    let side_menu_container = (
        BackgroundColor{
            0: Color::Srgba(Srgba { red: 0.12, green: 0.18, blue: 0.18, alpha: 0.8 })
        },
        Node {
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

    let title_text = (
        Text::new("Select Entity"),
        TextFont{
            font_size: 32.,
            ..default()
        }
    );
    
    // 1. Root node
    commands.spawn(root)
        .with_children(|parent| {
        // 2. Side menu panel
        parent.spawn(side_menu_container)
        .with_children(|parent| {
            // 3a. Menu title
            parent.spawn(title_text);
            
            // 3b. Button Row 
            parent.spawn(button_row)
            .with_children(|parent| {
                    match &selected_planet.planet {
                        Some(planet) => {
                            parent.spawn(Text::new(format!("planet with id {}", planet.id)));
                        },
                        None => {
                            parent.spawn(Text::new("choose a planet!"));
                        },
                    }
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
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub(crate) fn menu_action(
    mut action_query: Query<
        (&Interaction, &ButtonActions),
        (Changed<Interaction>, With<Button>)
    >,
    mut state: ResMut<GameState>
) {
    for(&interaction, action) in &mut action_query {
        if interaction == Interaction::Pressed {
            match action {
                ButtonActions::StartGame => {
                    state.set_if_neq(GameState::Playing);
                    println!("game should start now...");
                },
                ButtonActions::StopGame => {
                    state.set_if_neq(GameState::Paused);
                    println!("game should pause now...");
                }
            }
        }
    }
}