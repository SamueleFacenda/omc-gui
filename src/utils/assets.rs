use bevy::prelude::*;

use super::super::utils::constants::{EXP_SPRITE_NUM, PLANET_SPRITE_NUM};

#[derive(Resource)]
pub(crate) struct PlanetAssets {
    pub handles: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub(crate) struct ExplorerAssets {
    pub handles: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub(crate) struct CelestialAssets {
    pub handles: (Handle<Image>, Handle<Image>),
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut planet_handles = Vec::new();
    for i in 0..PLANET_SPRITE_NUM {
        let path = format!("planet{}.png", i);
        planet_handles.push(asset_server.load(path));
    }
    commands.insert_resource(PlanetAssets {
        handles: planet_handles,
    });

    let asteroid_handle = asset_server.load("asteroid.png");
    let sunray_handle = asset_server.load("sunray.png");

    commands.insert_resource(CelestialAssets {
        handles: (sunray_handle, asteroid_handle),
    });

    let mut exp_handles = Vec::new();
    for i in 0..EXP_SPRITE_NUM {
        let path = format!("explorer{}.png", i);
        exp_handles.push(asset_server.load(path));
    }
    commands.insert_resource(ExplorerAssets {
        handles: exp_handles,
    });
}
