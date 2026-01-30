use omc_galaxy::{Orchestrator, PlanetInfoMap };
// use omc_galaxy::utils::registry::PlanetType;
use bevy::prelude::*;

use crate::events::{Celestial, CelestialBody};

pub const GAME_TICK: f32 = 1.;

#[derive(Resource)]
pub struct OrchestratorResource {
    pub orchestrator: Orchestrator,
}

#[derive(Resource, PartialEq, Eq)]
pub enum GameState {
    WaitingStart,
    Playing,
    Paused,
}

#[derive(Resource, Clone)]
pub struct GalaxySnapshot {
    pub edges: Vec<(u32, u32)>,
    pub planet_num: usize,
}

#[derive(Resource)]
pub struct PlanetClickRes {
    pub planet: Option<u32>,
}

#[derive(Resource)]
pub struct PlanetInfoRes {
    pub map: PlanetInfoMap,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);

pub fn setup_orchestrator(mut commands: Commands) {
    dotenv::dotenv().ok();

    let mut orchestrator = Orchestrator::new().expect("Failed to create orchestrator");

    let file_path = std::env::var("INPUT_FILE").expect("Set INPUT_FILE in .env or env vars");

    orchestrator
        .initialize_galaxy_by_file(file_path.as_str().trim())
        .expect("Failed to initialize galaxy");

    let (topology, planet_num) = orchestrator.get_topology();

    match orchestrator.start_all() {
        Err(s) => {
            error!("orchestrator failed to start. details: {}", s);
        }
        _ => {}
    }

    let lookup = orchestrator.get_planets_info();

    commands.insert_resource(OrchestratorResource { orchestrator });

    commands.insert_resource(GalaxySnapshot {
        edges: topology,
        planet_num,
    });

    commands.insert_resource(PlanetInfoRes { map: lookup });

    commands.insert_resource(GameState::WaitingStart);

    commands.insert_resource(GameTimer(Timer::from_seconds(
        GAME_TICK,
        TimerMode::Repeating,
    )));

    commands.insert_resource(PlanetClickRes { planet: None });
}

pub fn game_loop(
    mut commands: Commands,
    mut orchestrator: ResMut<OrchestratorResource>,
    mut planets: ResMut<PlanetInfoRes>,
    mut timer: ResMut<GameTimer>,
    state: Res<GameState>,
    time: Res<Time>,
) {
    if state.into_inner() == &GameState::Playing {
        timer.tick(time.delta());

        if timer.is_finished() {
            println!("ENTERED TIMER");
            let events = std::mem::take(&mut orchestrator.orchestrator.gui_messages);

            print!("events this tick: [");

            for ev in events {
                match ev {
                    omc_galaxy::OrchestratorEvent::PlanetDestroyed { planet_id } => {
                        // handle the destruction of a planet
                        print!("planet {} has died, ", planet_id);
                    }
                    omc_galaxy::OrchestratorEvent::SunrayReceived { planet_id } => {
                        print!("planet {} got a sunray (UI update), ", planet_id);
                        commands.trigger(Celestial {
                            planet_id,
                            kind: CelestialBody::Sunray,
                        });
                    }
                    omc_galaxy::OrchestratorEvent::SunraySent { planet_id } => {
                        print!("planet {} should get a sunray, ", planet_id);
                        // TODO only log to screen, nothing changes in the GUI
                    }
                    omc_galaxy::OrchestratorEvent::AsteroidSent { planet_id } => {
                        print!("planet {} should get an asteroid, ", planet_id);
                        commands.trigger(Celestial {
                            planet_id,
                            kind: CelestialBody::Asteroid,
                        });
                    }
                    _ => {
                        // TODO add the rest of the matches
                    }
                }
            }

            print!("] ");

            // update the planet state map after the events occurred
            planets.as_mut().map = orchestrator.orchestrator.get_planets_info();
            // launch either an asteroid or a sunray with a random choice
            let _ = orchestrator.orchestrator.choose_random_action();
            // handle all of the previous events
            let _ = orchestrator.orchestrator.handle_game_messages();

            println!("EXITING TIMER");
            timer.reset();
        }
    }
}
