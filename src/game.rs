use std::collections::VecDeque;

use bevy::prelude::*;
use omc_galaxy::{Orchestrator, OrchestratorEvent};

use crate::{
    ecs::{
        components::LogText,
        events::{Celestial, CelestialBody},
        resources::{
            EntityClickRes, ExplorerInfoRes, GalaxySnapshot, GameState, GameTimer, LogTextRes, OrchestratorResource, PlanetInfoRes
        },
    },
    utils::constants::GAME_TICK,
};

pub fn setup_orchestrator(mut commands: Commands) {
    dotenv::dotenv().ok();

    let mut orchestrator = Orchestrator::new().expect("Failed to create orchestrator");

    let file_path = std::env::var("INPUT_FILE").expect("Set INPUT_FILE in .env or env vars");

    orchestrator
        .initialize_galaxy_by_file(file_path.as_str().trim())
        .expect("Failed to initialize galaxy");

    let (topology, planet_num) = orchestrator.get_topology();

    let first_string = String::from("Orchestrator has started.\nWelcome to the game!");

    let lookup = orchestrator.get_planets_info();

    let exp_info = orchestrator.get_explorer_states();

    if let Err(s) = orchestrator.start_all() {
        error!("{}", s);
    }

    commands.insert_resource(OrchestratorResource { orchestrator });

    commands.insert_resource(GalaxySnapshot {
        edges: topology,
        planet_num,
    });

    commands.insert_resource(PlanetInfoRes { map: lookup });

    commands.insert_resource(ExplorerInfoRes { map: exp_info} );

    commands.insert_resource(GameState::WaitingStart);

    commands.insert_resource(LogTextRes {
        text: VecDeque::from([first_string]),
    });

    commands.insert_resource(GameTimer(Timer::from_seconds(
        GAME_TICK,
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

                let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);

                handle_tick(&mut commands, events, log_text);

                // update the planet state map after the events occurrederr
                planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
                // yeah
                explorers.as_mut().map = orchestrator.orchestrator.get_explorer_states();
                // launch either an asteroid or a sunray with a random choice
                let _ = orchestrator.orchestrator.choose_random_action();
                // handle all of the previous events
                let _ = orchestrator.orchestrator.handle_game_messages();

                println!("EXITING TIMER");
                timer.reset();
            }
        }
        GameState::Override => {
            //if there are manually imputted events, run those immediately
            //else, keep going

            if orchestrator.orchestrator.gui_messages.len() > 0 {
                let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);
                handle_tick(&mut commands, events, log_text);

                info!(
                    "haiiiiii {:?} cell state",
                    orchestrator.orchestrator.planets_info.get_info(4)
                );

                // handle all of the previous events
                let _ = orchestrator.orchestrator.handle_game_messages();
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
            omc_galaxy::OrchestratorEvent::PlanetDestroyed { planet_id } => {
                // handle the destruction of a planet
                info!("game-loop: planet {} has died, ", planet_id);
                update_logs(&mut log_text, format!("planet {} died!\n", planet_id));
            }
            omc_galaxy::OrchestratorEvent::SunrayReceived { planet_id } => {
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
            omc_galaxy::OrchestratorEvent::SunraySent { planet_id } => {
                info!("game-loop: planet {} should get a sunray, ", planet_id);
                // TODO only log to screen, nothing changes in the GUI
            }
            omc_galaxy::OrchestratorEvent::AsteroidSent { planet_id } => {
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
            _ => {
                // TODO add the rest of the matches
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
