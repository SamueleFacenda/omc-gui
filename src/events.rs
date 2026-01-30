use bevy::{ecs::component::Component, prelude::Event};

#[derive(Event)]
pub(crate) struct PlanetDespawn {
    pub planet_id: u32,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum CelestialBody {
    Sunray,
    Asteroid,
}

#[derive(Event, Component)]
pub(crate) struct Celestial {
    pub kind: CelestialBody,
    pub planet_id: u32,
}
