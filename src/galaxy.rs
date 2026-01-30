use bevy::prelude::*;
use bevy_tweening::{CycleCompletedEvent, Tween, TweenAnim, lens::TransformPositionLens};
use omc_galaxy::Status;
use std::{f32::consts::TAU, time::Duration};

use crate::{
    assets::{CelestialAssets, PlanetAssets, SPRITE_NUM},
    events::{Celestial, CelestialBody, PlanetDespawn},
    game::{
        self, GalaxySnapshot, PlanetClickRes, PlanetInfoRes,
    }, ui::UiText
};

#[derive(Component)]
pub(crate) struct Planet {
    id: u32,
}

#[derive(Component)]
pub(crate) struct Edge {
    connects: (u32, u32),
}

const PLANET_RAD: f32 = 50.;
const SUNRAY_RAD: f32 = PLANET_RAD / 2.;
const GALAXY_RADIUS: f32 = 250.;
//const MAX_PLANET_TYPES: usize = 7;

pub fn setup(
    galaxy: Res<GalaxySnapshot>,
    planets: Res<PlanetInfoRes>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    planet_assets: Res<PlanetAssets>,
) {
    commands.spawn(Camera2d);

    //create and load background image through sprites
    let background: Handle<Image> = asset_loader.load("sky.png");

    commands.spawn(Sprite {
        image: background,
        custom_size: Some(Vec2::new(1920., 1080.)), // default to FHD
        ..Default::default()
    });

    let planet_num = galaxy.planet_num;

    for (&i, _info) in planets.map.iter() {
        // spawn all the planets in a circle, with even spacing
        // Tau = 2 * pi, so all the planets go around the circle
        let angle = TAU * (i as f32) / (planet_num as f32);

        // extract x and y position via trigonometry
        let x = GALAXY_RADIUS * angle.cos();
        let y = GALAXY_RADIUS * angle.sin();

        //Handle is based on Arc, so cloning is fine
        let image_handle = planet_assets.handles[(i as usize) % SPRITE_NUM].clone();

        commands
            .spawn((
                Planet { id: i },
                Sprite {
                    image: image_handle,
                    custom_size: Some(Vec2::splat(PLANET_RAD * 2.)),
                    ..Default::default()
                },
                Transform::from_xyz(x, y, 2.0),
                Pickable::default(),
            ))
            .observe(choose_on_click);
    }
}

pub fn draw_topology(
    mut commands: Commands,
    snapshot: Res<GalaxySnapshot>,
    planets: Query<(&Planet, &Transform)>,
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
            let transform = Transform::from_xyz(midpoint.x, midpoint.y, 1.)
                .with_rotation(Quat::from_rotation_z(segment_rotation));

            commands.spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(length, 1.)),
                    ..default()
                },
                transform,
                Edge { connects: (*a, *b) },
            ));
        }
    }
}

pub fn destroy_link(
    event: On<PlanetDespawn>,
    mut commands: Commands,
    edge_query: Query<(&Edge, Entity)>,
    planet_query: Query<(&Planet, Entity)>,
) {
    //despawn all its links
    for (e, s) in edge_query {
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

pub fn move_celestial(
    event: On<Celestial>,
    mut commands: Commands,
    sprites: Res<CelestialAssets>,
    planet_query: Query<(&Planet, &Transform)>,
) {
    info!("MOVE_CELESTIAL: EVENT FROM ID {} ", event.planet_id);

    for (p, t) in planet_query {
        print!("{}, ", p.id);
        if p.id == event.planet_id {
            let sunray_sprite = match event.kind {
                CelestialBody::Sunray => {
                    info!("spawning sunray sprite");
                    sprites.handles.0.clone()
                }
                CelestialBody::Asteroid => {
                    info!("spawning asteroid sprite");
                    sprites.handles.1.clone()
                }
            };

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(game::GAME_TICK / 2.),
                TransformPositionLens {
                    start: Vec3::new(0., 0., 2.0),
                    end: Vec3::new(t.translation.x, t.translation.y, 2.0),
                },
            )
            .with_cycle_completed_event(true);

            commands.spawn((
                Celestial {
                    kind: event.kind,
                    planet_id: event.planet_id,
                },
                Sprite {
                    image: sunray_sprite,
                    custom_size: Some(Vec2::splat(SUNRAY_RAD * 2.)),
                    ..default()
                },
                Transform::from_xyz(0., 0., 2.0),
                TweenAnim::new(tween),
            ));
        }
    }
}

//TODO run this function at every tick, not every frame
pub(crate) fn despawn_celestial(
    mut commands: Commands,
    mut reader: MessageReader<CycleCompletedEvent>,
    status: Res<PlanetInfoRes>,
    celestial: Query<&Celestial>,
) {
    for event in reader.read() {
        info!("animation finished!");

        if let Ok(c) = celestial.get(event.anim_entity) {
            if c.kind == CelestialBody::Asteroid {
                if status.map.get_status(&c.planet_id) == Status::Dead {
                    info!("Triggerig PlanetDespawn for {}", c.planet_id);
                    commands.trigger(PlanetDespawn {
                        planet_id: c.planet_id,
                    });
                }
            }
        }

        commands.entity(event.anim_entity).despawn();
    }
}

pub(crate) fn choose_on_click(
    click: On<Pointer<Click>>,
    mut chosen_planet: ResMut<PlanetClickRes>,
    mut planet_query: Query<(&mut Sprite, &Planet)>,
) {
    info!("Picking event was triggered");

    //reset all sprite dimensions to normal
    for (mut sprite, _) in &mut planet_query {
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.));
    }

    if let Ok((mut sprite, planet)) = planet_query.get_mut(click.entity) {
        info!("picked info for planet {}", planet.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.5));

        chosen_planet.planet = Some(planet.id);
    }
}

pub(crate) fn update_selected_planet(
    selected_planet: Res<PlanetClickRes>,
    planet_status: Res<PlanetInfoRes>,
    mut text_to_set: Query<(&mut Text, &UiText)>,
) {
    // exit early if the state is the same to avoid extra computation 
    if !selected_planet.is_changed() && !planet_status.is_changed() {
        return;
    }


    if let Some(planet_id) = selected_planet.planet {

        let map = &planet_status.map;

        if let Some(planet_info) = map.get_info(planet_id) {
            for (mut text, field_type) in &mut text_to_set {
                match field_type {
                    UiText::Name => {
                        **text = format!("Name: {:?}", planet_info.name);
                    },
                    UiText::Id => {
                        **text = format!("Planet ID: {:?}", planet_id);
                    },
                    UiText::Rocket =>{
                        if planet_info.rocket {
                            **text = "Rocket: AVAILABLE".to_string();
                        } else {
                            **text = "Rocket: NOT PRESENT".to_string();
                        }
                    }
                    UiText::Energy =>{

                        let current_energy = planet_info.charged_cells_count;
                        let max_energy = planet_info.energy_cells.len();
                        **text = format!("Charged cells: {} out of {}", current_energy, max_energy);
                    }
                    _ => {}
                }
            }
        }

    }
}
