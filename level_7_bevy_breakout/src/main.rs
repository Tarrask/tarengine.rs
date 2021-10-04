use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

struct SplashImage;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
        .add_startup_system(setup.system())
        .add_startup_system(setup_ui.system())
        .add_system(transition_in.system())
        .run();
}

fn setup_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ImageBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            ..Default::default()
        },
        material: materials
            .add(asset_server.load("textures/bevy_logo_dark_big.png").into()),
        ..Default::default()
    });
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let bevy_logo = asset_server.load("textures/bevy_logo_dark_big.png");
    // let logo_material = materials.add(ColorMaterial {
    //     texture: Some(bevy_logo),
    //     color: Color::rgba(1.0, 1.0, 1.0, 0.0),
    //     ..Default::default()
    // });
    let logo_material = materials.add(bevy_logo.into());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: logo_material,
            visible: Visible {
                is_transparent: true,
                is_visible: true
            },
            draw: Draw {
render_commands
            },
            ..Default::default()
        })
        .insert(SplashImage);
}

fn transition_in(
    splash_images: Query<&Handle<ColorMaterial>, With<SplashImage>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for material_handle in splash_images.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            let current_a = material.color.a();
            material.color.set_a(current_a * 1.01 + 0.0001);
            println!("alpha is: {}", current_a);
        }
        // material.color.set_a(1.0);
    }
}