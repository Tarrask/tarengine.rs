mod game;
mod systems;
mod config;

use amethyst::utils::fps_counter::FpsCounterBundle;
#[allow(unused)]
use log::{debug, info, warn, error};

use amethyst::core::TransformBundle;
use amethyst::{Application, GameDataBuilder};
use amethyst::input::{InputBundle, StringBindings}; 
use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle, types::DefaultBackend};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::application_root_dir;
use amethyst::prelude::Config;

use crate::game::{BoidGameDataBuilder, BoidsState};
use crate::config::BoidConfig;

fn main() -> amethyst::Result<()> {
    // Engine setup
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let boid_config_path = config_path.join("boid.ron");
    let display_config_path = config_path.join("display.ron");
    let binding_config_path = config_path.join("bindings.ron");
    let assets_path = app_root.join("assets");
    info!("Game starting..."); 
    info!("  - App root:       {:?}", app_root);
    info!("  - Config path:    {:?}", config_path);
    info!("  - Display config: {:?}", display_config_path);
    info!("  - Binding config: {:?}", binding_config_path);
    info!("  - Asset path:     {:?}", assets_path);

    let boid_config = BoidConfig::load(&boid_config_path)?;
    println!("boid config: {:?}", boid_config);

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.592156, 0.796078, 0.941176, 1.0]) 
        )
        // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());

    let transform_bundle = TransformBundle::new();
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config_path)?;

    let ui_bundle = UiBundle::<StringBindings>::new();

    let fps_bundle = FpsCounterBundle {};

    // Initializing data
    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(ui_bundle)?
        .with_bundle(fps_bundle)?
        .with(systems::BoidSystem, "boid_system", &[])
        .with(systems::MoveSystem, "move_system", &["boid_system"])
        // .with(CameraOrthoSystem, "camera_system", &[]);
        .with(systems::CameraSystem, "camera_system", &[])
        .with(systems::FpsSystem, "fps_system", &[]);

    let mut app_builder = Application::build(assets_path, BoidsState::default())?;
    let game_data = BoidGameDataBuilder::default()
        .with_base_bundle(&mut app_builder.world, render_bundle)?
        .with_base_bundle(&mut app_builder.world, transform_bundle)?
        .with_base_bundle(&mut app_builder.world, input_bundle)?
        .with_base_bundle(&mut app_builder.world, ui_bundle)?
        .with_base_bundle(&mut app_builder.world, fps_bundle)?
        .with_running(systems::BoidSystem, "boid_system", &[])
        .with_running(systems::MoveSystem, "move_system", &["boid_system"])
        // .with(CameraOrthoSystem, "camera_system", &[]);
        .with_running(systems::CameraSystem, "camera_system", &[])
        .with_running(systems::FpsSystem, "fps_system", &[]);

    // let mut game = app_builder
    //     .with_resource(boid_config)
    //     .build(game_data)?;

    // let mut game = Application::new(assets_path, BoidsState::default(), game_data)?
    //     .with_resource(boid_config)?;

    // let mut game = Application::build(assets_path, BoidsState::default())?
    //     .with_resource(boid_config)
    //     .build(game_data)?;
    
        let mut game = app_builder.build(game_data)?;

    // Running the game loop
    game.run();
    
    Ok(())
}
