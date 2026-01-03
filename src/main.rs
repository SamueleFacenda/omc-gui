use bevy::prelude::*;
use std::{f32::consts::TAU, sync::{Arc, RwLock}};

use omc_galaxy::Orchestrator;
use std::env;

#[derive(Resource)]
struct GalaxyTopologyResource {
    topology: Arc<RwLock<Vec<Vec<bool>>>>
}

#[derive(Component)]
struct Planet{
    id: usize
}

pub fn main() -> Result<(), String>{

    // Load env
    dotenv::dotenv().ok();
    //Init and check orchestrator
    let mut orchestrator = Orchestrator::new()?;

    //Give the absolute path for the init file
    let file_path = env::var("INPUT_FILE")
        .expect("Imposta INPUT_FILE nel file .env o come variabile d'ambiente");

    orchestrator.initialize_galaxy_by_file(file_path.as_str().trim())?;

    let topology = orchestrator.get_topology();

    let mut app = App::new();
    app
    .insert_resource(GalaxyTopologyResource{topology})
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (setup, draw_topology.after(setup)));
    app.run();
    Ok(())
}

const PLANET_RAD: f32 = 25.;
const GALAXY_RADIUS: f32 = 175.;

fn setup(
    topology: Res<GalaxyTopologyResource>,
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn(Camera2d);

    let background: Handle<Image> = asset_loader.load("sky.png");

    commands.spawn(Sprite::from_image(background));

    let initial_gtop = topology
        .as_ref()
        .topology
        .try_read()
        .unwrap();

    let planet_num = initial_gtop.len();

    let shape = meshes.add(Circle::new(PLANET_RAD));

    for i in 0..planet_num {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / planet_num as f32, 0.95, 0.7);

        let angle = TAU * (i as f32) / (planet_num as f32);

        let x = GALAXY_RADIUS * angle.cos();
        let y = GALAXY_RADIUS * angle.sin();

        commands.spawn((
            Planet{id: i},
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                x,
                y,
                1.0,
            ),
        ));
    }

}

fn draw_topology(
    mut commands: Commands,
    topology: Res<GalaxyTopologyResource>,
    planets: Query<(&Planet, &Transform)>
) {
    let gtop = topology
        .as_ref()
        .topology
        .try_read()
        .unwrap();

    for (p1,p1t) in planets {
        for (p2, p2t) in planets{

            // using < avoids calculating "double edges" (not a bug but unneeded work)
            if p1.id < p2.id && gtop[p1.id][p2.id] == true {

                let start = &p1t.translation;
                let end = &p2t.translation;
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
                    0.
                ).with_rotation(Quat::from_rotation_z(segment_rotation));

                commands.spawn((Sprite{
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(length, 1.)),
                    ..default()
                }, transform));
            }
        }
    }
}