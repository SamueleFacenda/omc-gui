use bevy::prelude::*;

pub const SPRITE_NUM: usize = 7;

#[derive(Resource)]
pub(crate) struct PlanetAssets {
    pub handles: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub(crate) struct CelestialAssets {
    pub handles: (Handle<Image>, Handle<Image>),
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut handles = Vec::new();
    for i in 0..SPRITE_NUM {
        let path = format!("planet{}.png", i);
        handles.push(asset_server.load(path));
    }
    commands.insert_resource(PlanetAssets { handles });

    let asteroid_handle = asset_server.load("asteroid.png");
    let sunray_handle = asset_server.load("sunray.png");

    commands.insert_resource(CelestialAssets {
        handles: (sunray_handle, asteroid_handle),
    });
}
