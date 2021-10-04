use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, 
    prelude::*
};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_fps.system())
            .add_system(fps.system());
    }
}

struct FpsText;

fn setup_fps(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        text: Text::with_section(
            "00",
            TextStyle {
                font: asset_server.load("fonts/square.ttf"),
                font_size: 32.0,
                color: Color::GOLD,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            }
        ),
        ..Default::default()
    }).insert(FpsText);
}

fn fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("{:02.0}", average);
            }
        }
    }
}