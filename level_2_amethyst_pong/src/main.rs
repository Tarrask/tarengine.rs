mod pong;
mod systems;

use crate::pong::Pong;

#[allow(unused)]
use log::{debug, info, warn, error};

use amethyst::{
    core::TransformBundle, 
    input::{InputBundle, StringBindings}, 
    prelude::*, 
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    ui::{RenderUi, UiBundle}, 
    utils::application_root_dir};

fn main() -> amethyst::Result<()> {
    // Engine setup
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let display_config_path = config_path.join("display.ron");
    let binding_config_path = config_path.join("bindings.ron");
    let assets_path = app_root.join("assets");
    info!("Game starting..."); 
    info!("  - App root:       {:?}", app_root);
    info!("  - Config path:    {:?}", config_path);
    info!("  - Display config: {:?}", display_config_path);
    info!("  - Binding config: {:?}", binding_config_path);
    info!("  - Asset path:     {:?}", assets_path);

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.00196, 0.23726, 0.21765, 1.0])
        )
        // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());

    let transform_bundle = TransformBundle::new();
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config_path)?;

    let ui_bundle = UiBundle::<StringBindings>::new();

    // Initializing data
    let game_data = GameDataBuilder::default()
        .with_bundle(render_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(input_bundle)?
        .with_bundle(ui_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "bounce_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let mut game = Application::new(assets_path, Pong::default(), game_data)?;

    // Running the game loop
    game.run();

    Ok(())
}
