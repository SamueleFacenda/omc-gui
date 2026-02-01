use bevy::ecs::component::Component;

// Galaxy-centric components
#[derive(Component)]
pub(crate) struct Planet {
    pub id: u32,
}

#[derive(Component)]
pub(crate) struct Explorer {
    pub id: u32,
    pub current_planet: u32,
}

#[derive(Component)]
pub(crate) struct Edge {
    pub connects: (u32, u32),
}

/// Button associated actions
#[derive(Component)]
pub enum ButtonActions {
    StartGame,
    StopGame,
    ManualAsteroid,
    ManualSunray,
    Blind,
    Nuke,
}

/// Planet info marker component
#[derive(Component)]
pub enum UiPlanetText {
    Name,
    Id,
    Status,
    Energy,
    Rocket
}

/// Button visibility marker component;
/// makes it so that the buttons tagged 
/// with this component are rendered only
/// when a planet is selected.
#[derive(Component)]
pub struct PlanetOnlyButton;

/// Explorer info marker component
#[derive(Component)]
pub enum UiExplorerText {
    Id,
    Visiting,
    Status,
    ResourceBag,
}

/// Marker component for any loggable action
#[derive(Component)]
pub struct LogText;
