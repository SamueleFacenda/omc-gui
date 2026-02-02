use std::collections::VecDeque;

use bevy::prelude::*;

use super::super::types::{PlanetInfoMap, ExplorerInfoMap, Status};
use crate::Orchestrator;

#[derive(Resource)]
pub struct OrchestratorResource {
    pub orchestrator: Orchestrator,
}

#[derive(Resource, PartialEq, Eq)]
pub enum GameState {
    WaitingStart,
    Playing,
    Paused,
    Override,
}

#[derive(Resource, Clone)]
pub struct GalaxySnapshot {
    pub edges: Vec<(u32, u32)>,
    pub planet_num: usize,
}

#[derive(Resource, Debug)]
pub struct EntityClickRes {
    pub planet: Option<u32>,
    pub explorer: Option<u32>,
}

#[derive(Resource)]
pub struct PlanetInfoRes {
    pub map: PlanetInfoMap,
}

#[derive(Resource)]
pub struct ExplorerInfoRes {
    pub map: ExplorerInfoMap,
}

#[derive(Resource)]
pub struct LogTextRes {
    pub text: VecDeque<String>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);
