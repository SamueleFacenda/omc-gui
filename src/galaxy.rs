use bevy::{color::palettes::css::{GREEN, WHITE}, prelude::*};
use std::f32::consts::TAU;

use crate::{assets::{PlanetAssets, SPRITE_NUM}, events::PlanetDespawn, game::{GalaxySnapshot, OrchestratorResource, PlanetClickRes, SelectedPlanet}};

#[derive(Component)]
pub(crate) struct Planet{
    id: u32
}

#[derive(Component)]
pub(crate) struct Edge{
    connects: (u32,u32)
}

const PLANET_RAD: f32 = 50.;
const GALAXY_RADIUS: f32 = 250.;
//const MAX_PLANET_TYPES: usize = 7;

pub fn setup(
    galaxy: Res<GalaxySnapshot>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    planet_assets: Res<PlanetAssets>,
) {

    commands.spawn(Camera2d);

    //create and load background image through sprites
    let background: Handle<Image> = asset_loader.load("sky.png");

    commands.spawn(Sprite{
        image: background,
        custom_size: Some(Vec2::new(1920., 1080.)), // default to FHD
        ..Default::default()
    });

    let planet_num = galaxy.planet_num;

    for (&i, info) in galaxy.planet_states.iter() {

        // spawn all the planets in a circle, with even spacing
        // Tau = 2 * pi, so all the planets go around the circle
        let angle = TAU * (i as f32) / (planet_num as f32);

        // extract x and y position via trigonometry
        let x = GALAXY_RADIUS * angle.cos();
        let y = GALAXY_RADIUS * angle.sin();

        //Handle is based on Arc, so cloning is fine
        let image_handle = planet_assets.handles[(i as usize) % SPRITE_NUM].clone();

        commands.spawn((
            Planet{id: i},
            Sprite {
                image: image_handle,
                custom_size: Some(Vec2::splat(PLANET_RAD * 2.)),
                ..Default::default()
            },
            Transform::from_xyz(
                x,
                y,
                2.0,
            ),
            Pickable::default()
        ))
        .observe(choose_on_hover);
    }

}

pub fn draw_topology(
    mut commands: Commands,
    snapshot: Res<GalaxySnapshot>,
    planets: Query<(&Planet, &Transform)>
) {
    if snapshot.is_changed() {
        let gtop = &snapshot.edges; //TODO do something BETTER than this

        for (a, b) in gtop.iter() {
            let (_, t1) = planets.iter().find(|(p, _)| p.id as u32 == *a).unwrap();
            let (_, t2) = planets.iter().find(|(p, _)| p.id as u32 == *b).unwrap();

            let start = &t1.translation;
                    let end = &t2.translation;
                    let length = start.distance(*end);

                    // diff is the same segment as start and end,
                    // but transposed wrt the origin of the 
                    // coordinate system
                    let segment = start - end;
                    
                    // finds the rotation of the segment wrt the origin
                    // using the arctangent function
                    let segment_rotation = segment.y.atan2(segment.x);
                    let midpoint = (start + end) / 2.;

                    //creates the transform to manipulate the line position
                    let transform = Transform::from_xyz(
                        midpoint.x,
                        midpoint.y,
                        1.
                    ).with_rotation(Quat::from_rotation_z(segment_rotation));

                    commands.spawn((
                        Sprite{
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(length, 1.)),
                            ..default()
                        }, 
                        transform,
                        Edge{
                            connects: (*a,*b)
                        }
                        ));
        }
    }
}

pub fn destroy_link(
    event: On<PlanetDespawn>,
    mut commands: Commands,
    edge_query: Query<(&Edge, Entity)>,
    planet_query: Query<(&Planet, Entity)>
) {
    //despawn all its links
    for (e,s) in edge_query {
        if e.connects.0 == event.planet_id || e.connects.1 == event.planet_id {
            commands.entity(s).despawn();
        }
    }

    //despawn the planet itself
    for (p, e) in planet_query {
        if p.id == event.planet_id {
            commands.entity(e).despawn();
        }
    }
}

fn choose_on_hover(
    hover: On<Pointer<Click>>,
    mut planet_query: Query<(&mut Sprite, &Planet)>,
    clicked_planet: ResMut<PlanetClickRes>,
    orchestrator: Res<OrchestratorResource>,
) {
    info!("Picking event was triggered");

    //reset all sprite dimensions to normal
    for (mut sprite,_) in &mut planet_query {
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.));  
    }

    if let Ok((mut sprite,planet)) = planet_query.get_mut(hover.entity) {
        info!("picked info for planet {}", planet.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.5));
        update_selected_planet(clicked_planet, orchestrator, planet.id);
    }

    
}

fn update_selected_planet (
    clicked_planet: ResMut<PlanetClickRes>,
    orchestrator: Res<OrchestratorResource>,
    planet_id: u32
) {
    // TODO change this ASAP this is super wasteful.
    // there's no need to clone the map at every click
    // actually the alternative is once per tick so maybe that's not bad
    let map = orchestrator.orchestrator.get_planets_info();

    if let Some(planet_info) = map.get_info(planet_id) {
        clicked_planet.into_inner().planet = Some(SelectedPlanet{
            id: planet_id,
            name: planet_info.name
        })
    }
    
}