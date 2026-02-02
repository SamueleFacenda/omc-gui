use bevy::prelude::*;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};

#[derive(Event)]
pub(crate) struct PlanetDespawn {
    pub planet_id: u32
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum CelestialBody {
    Sunray,
    Asteroid
}

#[derive(Event, Component)]
pub(crate) struct Celestial {
    pub kind: CelestialBody,
    pub planet_id: u32
}

#[derive(Event)]
pub(crate) struct MoveExplorerEvent {
    pub id: u32,
    pub destination: u32
}

#[derive(Event)]
pub(crate) struct BasicResEvent {
    pub id: u32,
    pub resource: BasicResourceType
}

#[derive(Event)]
pub(crate) struct ComplexResEvent {
    pub id: u32,
    pub resource: ComplexResourceType
}
/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub(crate) struct Scroll {
    pub entity: Entity,
    /// Scroll delta in logical coordinates.
    pub delta: Vec2
}
