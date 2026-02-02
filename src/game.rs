use std::collections::VecDeque;

use bevy::prelude::*;
use crate::app::AppConfig;
use super::{
    ecs::{
        components::LogText,
        events::{BasicResEvent, Celestial, CelestialBody, ComplexResEvent, MoveExplorerEvent},
        resources::{
            EntityClickRes, ExplorerInfoRes, GalaxySnapshot, GameState, GameTimer, LogTextRes,
            OrchestratorResource, PlanetInfoRes,
        },
    }
};
use crate::explorers::ExplorerFactory;
use super::types::{OrchestratorEvent};
use crate::orchestrator::{Orchestrator, OrchestratorMode};

pub fn setup_orchestrator(mut commands: Commands) {
    let config = AppConfig::get();

    let explorers = config.explorers.iter().map(ExplorerFactory::make_from_name).collect();

    let mut orchestrator = Orchestrator::new(OrchestratorMode::Manual, config.number_of_planets, explorers)
        .unwrap_or_else(|e| {
            log::error!("Failed to create orchestrator: {e}");
            panic!("Failed to create orchestrator: {e}");
        });


    if let Err(e) = orchestrator.manual_init() {
        log::error!("Failed to initialize orchestrator: {e}");
        panic!("Failed to initialize orchestrator: {e}");
    }


    let topology = orchestrator.get_topology();

    let first_string = String::from("Orchestrator has started.\nWelcome to the game!");

    let lookup = orchestrator.get_planets_info();

    let exp_info = orchestrator.get_explorer_states();

    commands.insert_resource(OrchestratorResource { orchestrator });

    commands.insert_resource(GalaxySnapshot {
        edges: topology,
        planet_num: config.number_of_planets as usize,
    });

    commands.insert_resource(PlanetInfoRes { map: lookup });

    commands.insert_resource(ExplorerInfoRes { map: exp_info });

    commands.insert_resource(GameState::WaitingStart);

    commands.insert_resource(LogTextRes {
        text: VecDeque::from([first_string]),
    });

    commands.insert_resource(GameTimer(Timer::from_seconds(
        AppConfig::get().game_tick_seconds,
        TimerMode::Repeating,
    )));

    commands.insert_resource(EntityClickRes {
        planet: None,
        explorer: None,
    });
}

pub fn game_loop(
    mut commands: Commands,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut planets: ResMut<PlanetInfoRes>,
    mut explorers: ResMut<ExplorerInfoRes>,
    mut timer: ResMut<GameTimer>,
    log_text: ResMut<LogTextRes>,
    state: Res<GameState>,
    time: Res<Time>,
) {
    match *state {
        GameState::Playing => {
            timer.tick(time.delta());

            if timer.is_finished() {
                println!("ENTERED TIMER");

                let events = orchestrator.orchestrator.get_gui_events_buffer().drain_events();

                handle_tick(&mut commands, events, log_text);

                // update the planet state map after the events occurrederr
                planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
                // yeah
                explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();

                // launch either an asteroid or a sunray with a random choice
                if let Err(e) = orchestrator.orchestrator.manual_step() {
                    log::error!("Failed to advance orchestrator step: {e}");
                    commands.insert_resource(GameState::Paused);
                }

                // handle all of the previous events
                if let Err(e) = orchestrator.orchestrator.process_commands() {
                    log::error!("Failed to process orchestrator commands: {e}");
                    commands.insert_resource(GameState::Paused);
                }

                println!("EXITING TIMER");
                timer.reset();
            }
        }
        GameState::Override => {
            //if there are manually inputted events, run those immediately
            //else, keep going

            if orchestrator.orchestrator.get_gui_events_buffer().has_events() {
                let events = orchestrator.orchestrator.get_gui_events_buffer().drain_events();
                handle_tick(&mut commands, events, log_text);

                // handle all of the previous events
                if let Err(e) = orchestrator.orchestrator.process_commands() {
                    log::error!("Failed to process orchestrator commands: {e}");
                    commands.insert_resource(GameState::Paused);
                }

                // update the planet state map after the events occurred
                planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
                explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();
            }
        }
        _ => {}
    }
}

fn handle_tick(
    commands: &mut Commands,
    events: Vec<OrchestratorEvent>,
    mut log_text: ResMut<LogTextRes>,
) {
    for ev in events {
        match ev {
            OrchestratorEvent::PlanetDestroyed { planet_id } => {
                // handle the destruction of a planet
                info!("game-loop: planet {} has died, ", planet_id);
                update_logs(&mut log_text, format!("planet {} died!\n", planet_id));
            }
            OrchestratorEvent::SunrayReceived { planet_id } => {
                info!("game-loop: planet {} got a sunray (UI update), ", planet_id);
                commands.trigger(Celestial {
                    planet_id,
                    kind: CelestialBody::Sunray,
                });
                update_logs(
                    &mut log_text,
                    format!("planet {} received a sunray\n", planet_id),
                );
            }
            OrchestratorEvent::SunraySent { planet_id } => {
                info!("game-loop: planet {} should get a sunray, ", planet_id);
                // TODO only log to screen, nothing changes in the GUI
            }
            OrchestratorEvent::AsteroidSent { planet_id } => {
                info!("game-loop: planet {} should get an asteroid, ", planet_id);
                commands.trigger(Celestial {
                    planet_id,
                    kind: CelestialBody::Asteroid,
                });
                update_logs(
                    &mut log_text,
                    format!("planet {} received an asteroid\n", planet_id),
                );
            }
            OrchestratorEvent::ExplorerMoved {
                explorer_id,
                destination,
            } => {
                info!(
                    "game-loop: explorer {} has moved to planet {}",
                    explorer_id, destination
                );
                commands.trigger(MoveExplorerEvent {
                    id: explorer_id,
                    destination,
                });
            }
            OrchestratorEvent::BasicResourceGenerated {
                explorer_id,
                resource,
            } => {
                info!(
                    "game-loop: explorer {} has generated basic resource {:?}",
                    explorer_id, resource
                );
                commands.trigger(BasicResEvent {
                    id: explorer_id,
                    resource,
                });
            }
            OrchestratorEvent::ComplexResourceGenerated {
                explorer_id,
                resource,
            } => {
                info!(
                    "game-loop: explorer {} has generated complex resource {:?}",
                    explorer_id, resource
                );
                commands.trigger(ComplexResEvent {
                    id: explorer_id,
                    resource,
                });
            }
        }
    }
}

fn update_logs(log_text: &mut ResMut<LogTextRes>, event_to_push: String) {
    log_text.text.push_front(event_to_push);
}

// TODO find a more performant approach.
// Fine now because time is of the essence
// and the average game isn't that long, but this
// allocates a new string everytime a log event
// happens. That's a lot of memory gone for nothing!

// alternative approach: spawn the log text
// directly, using commands.spawn()
pub(crate) fn log_text(logs: ResMut<LogTextRes>, mut log_node: Single<&mut Text, With<LogText>>) {
    if !logs.is_changed() {
        return;
    };
    let mut bruh = String::new();

    for log_event in logs.text.iter() {
        bruh += log_event;
    }

    log_node.0 = bruh;
}
