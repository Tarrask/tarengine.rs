use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, 
    prelude::*
};

const BALL_RADIUS: f32 = 10.0;
const INITIAL_BALL_VELOCITY_X: f32 = 100.0;
const INITIAL_BALL_VELOCITY_Y: f32 = 150.0;
const INITIAL_PADDLE_SPEED: f32 = 150.0;

struct Ball {
    center: Vec2,
    velocity: Vec2
}

enum Side {
    LEFT,
    RIGHT
}

struct Paddle {
    center: Vec2,
    width: f32,
    height: f32,
    speed: f32
}

#[derive(Default)]
struct Score {
    player_1: i32,
    player_2: i32
}

struct FpsText;

struct ScoreText;

struct Atlases {
    pub main: Handle<TextureAtlas>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    Running,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(setup_world.system()))
        .add_system_set(SystemSet::on_enter(AppState::Running)
            .with_system(setup_ui.system())
            .with_system(setup_ball.system())
            .with_system(setup_paddles.system()))
        .init_resource::<Score>()
        .add_system(move_balls.system())
        .add_system(move_paddles.system())
        //
        .add_system(collision.system())
        //
        .add_system(apply_transform.system())
        .add_system(fps.system())
        .add_system(score_board.system())
        .run();
}

fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>
) {
    // load textures / sprites
    let texture_handle = asset_server.load("textures/pong_spritesheet.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(8.0, 16.0));
    texture_atlas.add_texture(bevy::sprite::Rect{ min: Vec2::new(0.0, 0.0), max: Vec2::new(4.0, 16.0) });
    texture_atlas.add_texture(bevy::sprite::Rect{ min: Vec2::new(4.0, 0.0), max: Vec2::new(8.0, 4.0) });
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(Atlases { main: texture_atlas_handle });

    // add a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    state.set(AppState::Running).unwrap();
}

fn setup_ui(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/square.ttf");

    // fps
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "FPS: ".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE
                    }
                },
                TextSection {
                    value: "99".to_string(),
                    style: TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::GOLD,
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(FpsText);

    // players score
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // left score board
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                position: Rect {
                                    top: Val::Px(50.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "00",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 75.0,
                                    color: Color::GOLD,
                                },
                                TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(ScoreText)
                        .insert(Side::LEFT);
                });

            // right score board
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::FlexEnd,
                                position: Rect {
                                    top: Val::Px(50.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "00",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 75.0,
                                    color: Color::GOLD,
                                },
                                TextAlignment {
                                    horizontal: HorizontalAlign::Center,
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(ScoreText)
                        .insert(Side::RIGHT);
                });
        });
}

fn fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn score_board(score: Res<Score>, mut query: Query<(&mut Text, &Side), With<ScoreText>>) {
    for (mut text, side) in query.iter_mut() {
        match side {
            Side::LEFT => {
                text.sections[0].value = format!("{:02}", score.player_1);
            }
            Side::RIGHT => {
                text.sections[0].value = format!("{:02}", score.player_2);
            }
        }
    }
}

fn setup_ball(mut commands: Commands, altlases_handle: Res<Atlases>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(1),
            texture_atlas: altlases_handle.main.clone(),
            transform: Transform::from_scale(Vec3::splat(5.0)),
            ..Default::default()
        })
        .insert(Ball {
            center: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(INITIAL_BALL_VELOCITY_X, INITIAL_BALL_VELOCITY_Y)
        });
}

fn setup_paddles(
    mut commands: Commands, 
    windows: Res<Windows>,
    altlases_handle: Res<Atlases>
) {
    let window_demi_width = match windows.get_primary() {
        Some(window) => window.width() / 2.0,
        None => 400.0
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: altlases_handle.main.clone(),
            transform: Transform {
                translation: Vec3::new(-window_demi_width, 0.0, 0.0),
                scale: Vec3::splat(5.0),
                rotation: Quat::IDENTITY
            },
            ..Default::default()
        })
        .insert(Paddle {
            center: Vec2::new(-window_demi_width, 0.0),
            width: 20.0,
            height: 80.0,
            speed: INITIAL_PADDLE_SPEED
        })
        .insert(Side::LEFT);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: altlases_handle.main.clone(),
            transform: Transform {
                translation: Vec3::new(-window_demi_width, 0.0, 0.0),
                scale: Vec3::splat(5.0),
                rotation: Quat::IDENTITY
            },
            ..Default::default()
        })
        .insert(Paddle {
            center: Vec2::new(window_demi_width, 0.0),
            width: 20.0,
            height: 80.0,
            speed: INITIAL_PADDLE_SPEED
        })
        .insert(Side::RIGHT);
}

fn move_balls(
    time: Res<Time>, 
    windows: Res<Windows>,
    mut score: ResMut<Score>,
    mut balls: Query<&mut Ball>
) {
    let window = windows.get_primary().unwrap();

    for mut ball in balls.iter_mut() {
        let v = ball.velocity;
        ball.center += v * time.delta_seconds();

        if ball.center.y > window.height() / 2.0 - BALL_RADIUS {
            ball.velocity.y = -ball.velocity.y;
            ball.center.y += window.height() / 2.0 - BALL_RADIUS - ball.center.y 
        }

        if ball.center.y < -window.height() / 2.0 + BALL_RADIUS {
            ball.velocity.y = -ball.velocity.y;
            ball.center.y += -window.height() / 2.0 + BALL_RADIUS - ball.center.y
        }

        // balle qui sort à droite ou à gauche
        if ball.center.x > window.width() / 2.0 + BALL_RADIUS {
            ball.velocity = Vec2::new(-INITIAL_BALL_VELOCITY_X, INITIAL_BALL_VELOCITY_Y);
            ball.center = Vec2::new(0.0, 0.0);
            score.player_1 += 1;
            println!("Score: {} - {}", score.player_1, score.player_2);
        } 
        if ball.center.x < -(window.width() / 2.0 + BALL_RADIUS) {
            ball.velocity = Vec2::new(INITIAL_BALL_VELOCITY_X, INITIAL_BALL_VELOCITY_Y);
            ball.center = Vec2::new(0.0, 0.0);
            score.player_2 += 1;
            println!("Score: {} - {}", score.player_1, score.player_2);
        }
    }
}

fn move_paddles(
    time: Res<Time>, 
    keys: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut paddles: Query<(&mut Paddle, &Side)>
) {
    let window = windows.get_primary().unwrap();
    let window_height = window.height();

    for (mut paddle, side) in paddles.iter_mut() {
        match side {
            Side::LEFT => {
                if keys.pressed(KeyCode::W) {
                    paddle.center.y += paddle.speed * time.delta_seconds();
                }
                if keys.pressed(KeyCode::S) {
                    paddle.center.y -= paddle.speed * time.delta_seconds();
                }

                paddle.center.x = (paddle.width - window.width()) / 2.0;
            }
            Side::RIGHT => {
                if keys.pressed(KeyCode::Up) {
                    paddle.center.y += paddle.speed * time.delta_seconds();
                }
                if keys.pressed(KeyCode::Down) {
                    paddle.center.y -= paddle.speed * time.delta_seconds();
                }

                paddle.center.x = (window.width() - paddle.width ) / 2.0;
            }
        }

        if paddle.center.y > (window_height - paddle.height) / 2.0 {
            paddle.center.y = (window_height - paddle.height) / 2.0;
        }
        if paddle.center.y < (paddle.height - window_height) / 2.0 {
            paddle.center.y = (paddle.height - window_height) / 2.0
        }
    }
}

fn collision(
    mut balls: Query<&mut Ball>,
    paddles: Query<&Paddle>
) {
    // rebond sur les palettes
    for mut ball in balls.iter_mut() {
        for paddle in paddles.iter() {
            if point_in_rect(
                ball.center.x, ball.center.y,
                paddle.center.x - paddle.width / 2.0 - BALL_RADIUS,
                paddle.center.y - paddle.height / 2.0 - BALL_RADIUS,
                paddle.center.x + paddle.width / 2.0 + BALL_RADIUS,
                paddle.center.y + paddle.height / 2.0 + BALL_RADIUS
            ) {
                if ball.velocity.x < 0.0 {
                    ball.center.x = ball.center.x + (paddle.center.x + paddle.width / 2.0 + BALL_RADIUS - ball.center.x) * 2.0;
                } 
                else {
                    ball.center.x = ball.center.x + (paddle.center.x - paddle.width / 2.0 - BALL_RADIUS - ball.center.x) * 2.0;
                }
                ball.velocity.x = -ball.velocity.x;
            }
        }
    }
}

fn apply_transform(mut set: QuerySet<(
    Query<(&mut Transform, &Ball)>,
    Query<(&mut Transform, &Paddle)>
)>) {
    for (mut transform,  ball) in set.q0_mut().iter_mut() {
        transform.translation.x = ball.center.x;
        transform.translation.y = ball.center.y;
    }
    for (mut transform, paddle) in set.q1_mut().iter_mut() {
        transform.translation.x = paddle.center.x;
        transform.translation.y = paddle.center.y;
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}