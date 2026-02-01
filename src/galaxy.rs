use bevy::prelude::*;
use bevy_tweening::{CycleCompletedEvent, Tween, TweenAnim, lens::TransformPositionLens};
use omc_galaxy::Status;
use rand;
use std::{f32::consts::TAU, time::Duration};

use crate::{
    ecs::{
        components::{Edge, Explorer, Planet, UiExplorerText, UiPlanetText},
        events::{Celestial, CelestialBody, PlanetDespawn},
        resources::{EntityClickRes, ExplorerInfoRes, GalaxySnapshot, PlanetInfoRes},
    },
    utils::{
        assets::{CelestialAssets, ExplorerAssets, PlanetAssets},
        constants::{
            CELESTIAL_RAD, EXP_SPRITE_NUM, EXPLORER_SIZE, GALAXY_RADIUS, GAME_TICK, PLANET_RAD,
            PLANET_SPRITE_NUM,
        },
    },
};

pub fn setup(
    galaxy: Res<GalaxySnapshot>,
    planets: Res<PlanetInfoRes>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    planet_assets: Res<PlanetAssets>,
    explorer_assets: Res<ExplorerAssets>,
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

        let image_index = match planets.map.get_info(i).unwrap().name {
            omc_galaxy::utils::registry::PlanetType::BlackAdidasShoe => 0,
            omc_galaxy::utils::registry::PlanetType::Ciuc => 1,
            omc_galaxy::utils::registry::PlanetType::HoustonWeHaveABorrow => 2,
            omc_galaxy::utils::registry::PlanetType::ImmutableCosmicBorrow => 3,
            omc_galaxy::utils::registry::PlanetType::OneMillionCrabs => 4,
            omc_galaxy::utils::registry::PlanetType::Rustrelli => 5,
            omc_galaxy::utils::registry::PlanetType::RustyCrab => 6,
            omc_galaxy::utils::registry::PlanetType::TheCompilerStrikesBack => 7,
        };

        // Handle is based on Arc, so cloning is fine
        // the modulo with SPRITE_NUM is used to minimize runtime crashes
        // in case the index is out of bounds
        let planet_image_handle = planet_assets.handles[(image_index) % PLANET_SPRITE_NUM].clone();

        commands
            .spawn((
                Planet { id: i },
                Sprite {
                    image: planet_image_handle,
                    custom_size: Some(Vec2::splat(PLANET_RAD * 2.)),
                    ..Default::default()
                },
                Transform::from_xyz(x, y, 2.0),
                Pickable::default(),
            ))
            .observe(choose_on_click);

        if i == 0 {
            for j in 0..EXP_SPRITE_NUM {
                let explorer_image_handle = explorer_assets.handles[j].clone();
                let (rand_x, rand_y): (f32, f32) = rand::random();
                commands
                    .spawn((
                        Explorer {
                            id: j as u32,
                            current_planet: i,
                        },
                        Sprite {
                            image: explorer_image_handle,
                            custom_size: Some(Vec2::splat(EXPLORER_SIZE)),
                            ..Default::default()
                        },
                        Transform::from_xyz(
                            x + (rand_x * 100. % PLANET_RAD),
                            y - (rand_y * 100. % PLANET_RAD),
                            3.0,
                        ),
                        Pickable::default(),
                    ))
                    .observe(choose_on_click);
            }
        }
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
    explorer_query: Query<(&Explorer, Entity)>,
) {
    //despawn all its links
    for (e, s) in edge_query {
        if e.connects.0 == event.planet_id || e.connects.1 == event.planet_id {
            commands.entity(s).despawn();
        }
    }

    //if there is an explorer visiting, despawn
    for (exp, ent) in explorer_query {
        if exp.current_planet == event.planet_id {
            commands.entity(ent).despawn();
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
                Duration::from_secs_f32(GAME_TICK / 2.),
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
                    custom_size: Some(Vec2::splat(CELESTIAL_RAD * 2.)),
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
    mut params: ParamSet<(
        Query<(&mut Sprite, &Planet)>,
        Query<(&mut Sprite, &Explorer)>,
    )>,
    mut chosen_entity: ResMut<EntityClickRes>,
) {
    info!("Picking event was triggered");

    //reset all sprite dimensions to normal
    for (mut sprite, _) in &mut params.p0() {
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.));
    }

    for (mut sprite, _) in &mut params.p1() {
        sprite.custom_size = Some(Vec2::splat(EXPLORER_SIZE));
    }

    if let Ok((mut sprite, planet)) = params.p0().get_mut(click.entity) {
        info!("picked info for planet {}", planet.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(PLANET_RAD * 2.5));

        chosen_entity.planet = Some(planet.id);
        chosen_entity.explorer = None;
    }

    if let Ok((mut sprite, explorer)) = params.p1().get_mut(click.entity) {
        info!("picked info for explorer {}", explorer.id);
        // make sprite slightly bigger
        sprite.custom_size = Some(Vec2::splat(EXPLORER_SIZE * 1.5));

        chosen_entity.explorer = Some(explorer.id);
        chosen_entity.planet = None;
    }
}

pub(crate) fn update_selected_entity(
    selected_entity: Res<EntityClickRes>,
    planet_status: Res<PlanetInfoRes>,
    explorer_status: Res<ExplorerInfoRes>,
    mut params: ParamSet<(
        Query<(&mut Text, &UiPlanetText)>,
        Query<(&mut Text, &UiExplorerText)>,
    )>,
) {
    // exit early if the state is the same to avoid extra computation
    if !selected_entity.is_changed() && !planet_status.is_changed() {
        return;
    }

    info!("update_selected_entity: {:?}", selected_entity);

    if let Some(planet_id) = selected_entity.planet {
        info!("updating planet {}", planet_id);
        let map = &planet_status.map;

        if let Some(planet_info) = map.get_info(planet_id) {
            for (mut text, field_type) in &mut params.p0() {
                match field_type {
                    UiPlanetText::Name => {
                        **text = format!("Name: {:?}", planet_info.name);
                    }
                    UiPlanetText::Id => {
                        **text = format!("Planet ID: {:?}", planet_id);
                    }
                    UiPlanetText::Status => {
                        **text = format!("Status: {:?}", planet_info.status);
                    }
                    UiPlanetText::Rocket => {
                        if planet_info.rocket {
                            **text = "Rocket: AVAILABLE".to_string();
                        } else {
                            **text = "Rocket: NOT PRESENT".to_string();
                        }
                    }
                    UiPlanetText::Energy => {
                        let current_energy = planet_info.charged_cells_count;
                        let max_energy = planet_info.energy_cells.len();
                        **text = format!("Charged cells: {} out of {}", current_energy, max_energy);
                    }
                }
            }

            for (mut text, _) in &mut params.p1() {
                **text = "".to_string();
            }
        }
    }

    if let Some(explorer_id) = selected_entity.explorer {
        let map = &explorer_status.map;

        info!("updating explorer {}, map: {:?}", explorer_id, map.get(&explorer_id));

        if let Some(explorer_info) = map.get(&explorer_id) {
            for (mut text, field_type) in &mut params.p1() {
                match field_type {
                    UiExplorerText::Id => {
                        **text = format!("Explorer {:?}", explorer_id);
                    }
                    UiExplorerText::Status => {
                        let status = match explorer_info.status {
                            Status::Dead => "dead".to_string(),
                            Status::Paused => "paused".to_string(),
                            Status::Running => "running".to_string(),
                        };
                        **text = format!("Status: {}", status);
                    }
                    UiExplorerText::Visiting => {
                        **text = format!("Visiting planet {}", explorer_info.current_planet_id);
                    }
                    UiExplorerText::ResourceBag => {}
                }
            }

            for (mut text, _) in &mut params.p0() {
                **text = "".to_string();
            }
        }
    }
}
