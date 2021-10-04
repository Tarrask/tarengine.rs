use bevy::{prelude::*, tasks::ComputeTaskPool};
use bevy_prototype_lyon::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{Inspectable, InspectableRegistry, InspectorPlugin, WorldInspectorPlugin, plugin::InspectorWindows};
use rand::{thread_rng, Rng};

mod fps_plugin;

const SELECT_DISTANCE: f32 = 50.0 * 50.0;
const BOIDS_COUNT: usize = 2000;
const BOID_SIZE: f32 = 0.1;
const BOID_BASE_VELOCITY: f32 = 100.0;

#[derive(Inspectable)]
struct BoidsParams {
    cohesion_radius: f32,
    cohesion_factor: f32,
    alignment_radius: f32,
    alignment_factor: f32,
    repulsion_radius: f32,
    repulsion_factor: f32,
    boid_min_velocity: f32,
    boid_max_velocity: f32,
    boid_max_acceleration: f32,
    boid_dead_angle: f32
}

// mostly working
// cohesion_radius: 200.0,
// cohesion_factor: 500.0,
// alignment_radius: 100.0,
// alignment_factor: 200.0,
// repulsion_radius: 20.0,
// repulsion_factor: 1000.0,
// boid_min_velocity: 100.0,
// boid_max_velocity: 150.0,
// boid_max_acceleration: 500000.0

impl Default for BoidsParams {
    fn default() -> Self {
        BoidsParams {
            cohesion_radius: 200.0,
            cohesion_factor: 500.0,
            alignment_radius: 100.0,
            alignment_factor: 200.0,
            repulsion_radius: 20.0,
            repulsion_factor: 1000.0,
            boid_min_velocity: 100.0,
            boid_max_velocity: 150.0,
            boid_max_acceleration: 500000.0,
            boid_dead_angle: 0.4
        }
    }
}

#[derive(Inspectable, Default)]
struct Boid {
    pub alignment: Vec3,
    pub cohesion: Vec3,
    pub repulsion: Vec3
}

#[derive(Inspectable, Default)]
struct Physics {
    pub velocity: Vec3,
    pub acceleration: Vec3
}

#[derive(Inspectable, Default)]
struct SelectedBoid {
    boid: Option<Entity>,
    alignment: f32,
    cohesion: f32,
    repulsion: f32
}

struct SelectShape;

struct MainCamera;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Paused,
    Running
}

struct GameArea {
    width: f32,
    height: f32,
    demi_width: f32,
    demi_height: f32
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(fps_plugin::FpsPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<SelectedBoid>::new())
        .add_plugin(InspectorPlugin::<BoidsParams>::new())
        .insert_resource(ClearColor(Color::rgb(0.592156, 0.796078, 0.941176)))
        .insert_resource(SelectedBoid { boid: None, ..Default::default() })
        .insert_resource(BoidsParams::default())
        .insert_resource(GameArea { width: 800.0, height: 600.0, demi_width: 400.0, demi_height: 300.0 })
        .add_state(GameState::Running)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_boids.system())
        .add_startup_system(spawn_selected_shape.system())
        .add_startup_system(setup_inspector.system())
        .add_system(boids_system.system())
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(move_system.system())
        )
        .add_system(wrap_system.system())
        .add_system(select_system.system())
        .add_system(display_selected_system.system())
        .add_system(toggle_inspector.system())
        .add_system(restart_system.system())
        .add_system(inspector_system.system())
        .add_system(pause_system.system())
        .add_system(zoom_system.system())
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn spawn_boids(
    mut commands: Commands, 
    game_area: Res<GameArea>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut rng = thread_rng();
    let material = materials.add(asset_server.load("textures/boid_spritesheet.png").into());

    for _ in 0..BOIDS_COUNT {
        println!("add boid");
        let position = Vec3::new(
            rng.gen_range(-game_area.demi_width..game_area.demi_width),
            rng.gen_range(-game_area.demi_height..game_area.demi_height),
            0.0
        );
        let rotation = rng.gen_range(0.0..std::f32::consts::TAU);

        commands
            .spawn_bundle(SpriteBundle {
                material: material.clone(),
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(BOID_SIZE),
                    rotation: Quat::from_rotation_z(rotation)
                },
                ..Default::default()
            })
            .insert(Boid {..Default::default()})
            .insert(Physics {
                velocity: Vec3::new(rotation.cos() * BOID_BASE_VELOCITY, rotation.sin() * BOID_BASE_VELOCITY, 0.0),
                acceleration: Vec3::new(0.0, 0.0, 0.0)
            });
        
    }
}

fn spawn_selected_shape(
    mut commands: Commands
) {
    let shape = shapes::Circle {
        radius: 20.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape, 
            ShapeColors::outlined(Color::NONE, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(3.0),
            },
            Transform::default(),
        ))
        .insert(SelectShape);
}

fn setup_inspector(
    mut registry: ResMut<InspectableRegistry>
) {
    registry.register::<Boid>();
    registry.register::<Physics>();
    registry.register::<BoidsParams>();
    
}

fn toggle_inspector(
    selected_boid: Res<SelectedBoid>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<SelectedBoid>();
    if let Some(_) = selected_boid.boid {
        inspector_window_data.visible = true;
    }
    else {
        inspector_window_data.visible = false;
    }
}

fn inspector_system(
    mut selected_boid: ResMut<SelectedBoid>,
    boids: Query<(&Transform, &Boid, &Physics)>
) {
    if let Some(boid) = selected_boid.boid {
        if let Ok(things) = boids.get(boid) {
            let (_transform, boid, _physics) = things;
            selected_boid.alignment = boid.alignment.length();
            selected_boid.cohesion = boid.cohesion.length();
            selected_boid.repulsion = boid.repulsion.length();
        }

    }

}

fn move_system(
    time: Res<Time>,
    boids_params: Res<BoidsParams>,
    boids: Query<(&mut Transform, &mut Physics, &Boid), With<Boid>>
) {
    let dt = time.delta_seconds();
    let min_velocity_sq = boids_params.boid_min_velocity * boids_params.boid_min_velocity;
    let max_velocity_sq = boids_params.boid_max_velocity * boids_params.boid_max_velocity;

    boids.for_each_mut(|(mut transform, mut physics, boid)| {
        // acceleration
        physics.acceleration = boid.alignment + boid.cohesion + boid.repulsion;
        // avoid over acceleration
        let a = physics.acceleration.length_squared();
        if a > boids_params.boid_max_acceleration {
            physics.acceleration = physics.acceleration / a * boids_params.boid_max_acceleration;
        }

        // velocity
        physics.velocity = physics.velocity + physics.acceleration * dt;
        // avoid over speed and under speed
        let v = physics.velocity.length_squared();
        if v < min_velocity_sq {
            physics.velocity = physics.velocity / v * min_velocity_sq;
        }
        if v > max_velocity_sq {
            physics.velocity = physics.velocity / v * max_velocity_sq;
        }


        transform.translation = transform.translation + physics.velocity * dt;
        transform.rotation = Quat::from_rotation_z(f32::atan2(physics.velocity.y, physics.velocity.x));
    });
}

fn wrap_system(
    game_area: Res<GameArea>,
    boids: Query<&mut Transform, With<Boid>>
) {
    boids.for_each_mut(|mut transform| {
        // wrap boid around edge
        if transform.translation.x < -game_area.demi_width {
            transform.translation.x += game_area.width;
        }
        if transform.translation.x > game_area.demi_width {
            transform.translation.x -= game_area.width * 2.0;
        }
        if transform.translation.y < -game_area.demi_height {
            transform.translation.y += game_area.height * 2.0;
        }
        if transform.translation.y > game_area.demi_height {
            transform.translation.y -= game_area.height * 2.0;
        }
    });
}

fn boids_system(
    mut boids: Query<(Entity, &mut Boid, &Physics, &Transform)>,
    pool: Res<ComputeTaskPool>,
    boids_params: Res<BoidsParams>,
    others: Query<(Entity, &Physics, &Transform), With<Boid>>
) {
    boids.par_for_each_mut(&pool, 32, |(boid_entity, mut boid, physics, transform)| {
    // for (boid_entity, mut boid, physics, transform) in boids.iter_mut() {
        let mut cohesion_position: Vec3 = Vec3::ZERO;
        let mut cohesion_count: f32 = 0.0;
        let mut alignment_direction: Vec3 = Vec3::ZERO;
        let mut alignment_count: f32 = 0.0;
        let mut repulsion_force: Vec3 = Vec3::ZERO;

        let cohesion_radius_sq = boids_params.cohesion_radius * boids_params.cohesion_radius;
        let alignment_radius_sq = boids_params.alignment_radius * boids_params.alignment_radius;
        let repulstion_radius_sq = boids_params.repulsion_radius * boids_params.repulsion_radius;

        for (other_entity, other_physics, other_transform) in others.iter() {
            // avoid self
            if boid_entity == other_entity {
                continue;
            }

            let distance: Vec3 = transform.translation - other_transform.translation;
            let distance_sq = distance.length_squared();
            let angle = distance.angle_between(physics.velocity);

            if angle < boids_params.boid_dead_angle {
                continue;
            }
            
            // cohesion
            if distance_sq < cohesion_radius_sq {
                cohesion_position += other_transform.translation;
                cohesion_count += 1.0;
            }

            // alignment
            if distance_sq < alignment_radius_sq {
                alignment_direction += other_physics.velocity;
                alignment_count += 1.0; 
            }

            // repulsion
            if distance_sq < repulstion_radius_sq {
                repulsion_force += (distance / distance_sq.sqrt()) * boids_params.repulsion_factor;
            }
        }

        // cohesion
        if cohesion_count > 0.0 { 
            boid.cohesion = (cohesion_position / cohesion_count - transform.translation) / boids_params.cohesion_radius * boids_params.cohesion_factor;
        }
        else {
            boid.cohesion = Vec3::ZERO;
        }

        // alignment
        if alignment_count > 0.0 {
            boid.alignment = ( alignment_direction / alignment_count - physics.velocity) / (boids_params.boid_max_velocity * 2.0) * boids_params.alignment_factor;
        }
        else {
            boid.alignment = Vec3::ZERO;
        }

        // repulsion
        boid.repulsion = repulsion_force;
    // }
    })
}

fn select_system(
    mut selected_boid: ResMut<SelectedBoid>,
    boids: Query<(Entity, &Transform), With<Boid>>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    cameras: Query<&Transform, With<MainCamera>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            if let Some(position) = window.cursor_position() {
                if let Ok(cam) = cameras.single() {
                    let position = Vec3::new(
                        (position.x - window.width() * 0.5) * cam.scale.x,
                        (position.y - window.height() * 0.5) * cam.scale.y,
                        0.0
                    );

                    let mut closest_boid = None;
                    let mut closest_distance_sq = f32::INFINITY;

                    boids.for_each(|(entity, transform)| {
                        let distance_sq = position.distance_squared(transform.translation);
                        if distance_sq < closest_distance_sq && distance_sq < SELECT_DISTANCE {
                            closest_boid = Some(entity);
                            closest_distance_sq = distance_sq;
                        }
                    });
                    
                    if let Some(_) = closest_boid {
                        selected_boid.boid = closest_boid;
                        println!("Selected boid: {:?} at {}", closest_boid, closest_distance_sq.sqrt());
                    }
                }
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        selected_boid.boid = None;
    }
}

fn display_selected_system(
    selected_boid: Res<SelectedBoid>,
    mut set: QuerySet<(
        Query<&Transform, With<Boid>>,
        Query<&mut Transform, With<SelectShape>>
    )>
) {
    let boid_translation = match selected_boid.boid {
        Some(entity) => match set.q0().get(entity) {
            Ok(transform) => {
                transform.translation.clone()
            },
            Err(_) => {
                Vec3::new(100000.0, 100000.0, 0.0)
            }
        },
        None => Vec3::new(100000.0, 100000.0, 0.0)
    };

    set.q1_mut().for_each_mut(|mut shape_transform| {
        shape_transform.translation = boid_translation;
    })
}

fn restart_system(
    mut boids: Query<(&mut Physics, &mut Transform), With<Boid>>,
    keys: Res<Input<KeyCode>>,
    game_area: Res<GameArea>,
) {
    
    if keys.just_pressed(KeyCode::R) {
        let mut rng = thread_rng();
        for (mut physics, mut transform) in boids.iter_mut() {
            let position = Vec3::new(
                rng.gen_range(-game_area.demi_width..game_area.demi_width),
                rng.gen_range(-game_area.demi_height..game_area.demi_height),
                0.0
            );
            let rotation = rng.gen_range(0.0..std::f32::consts::TAU);

            transform.translation = position;
            transform.rotation = Quat::from_rotation_z(rotation);
            
            physics.velocity = Vec3::new(rotation.cos() * BOID_BASE_VELOCITY, rotation.sin() * BOID_BASE_VELOCITY, 0.0);
            physics.acceleration = Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

fn pause_system(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>
) {
    if keys.just_pressed(KeyCode::Space) {
        let new_state = match game_state.current() {
            GameState::Paused => GameState::Running,
            GameState::Running => GameState::Paused
        };
        game_state.set(new_state).unwrap();
    }
}

fn zoom_system(
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    windows: Res<Windows>,
    mut game_area: ResMut<GameArea>
) {
    if let Ok(mut cam) = cameras.single_mut() {
        if keys.just_pressed(KeyCode::NumpadAdd) {
            cam.scale *= 1.1;
        }
        if keys.just_pressed(KeyCode::NumpadSubtract) {
            cam.scale /= 1.1;
        }

        if let Some(window) = windows.get_primary() {
            game_area.width = window.width() * cam.scale.x;
            game_area.height = window.height() * cam.scale.y;
            game_area.demi_width = game_area.width * 0.5;
            game_area.demi_height = game_area.height * 0.5;
        }
    }
}