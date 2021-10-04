use amethyst::{core::Time, ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform}};
#[allow(unused)]
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_VELOCITY: f32 = 50.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
    pub velocity: f32
}

impl Paddle {
    fn new(side: Side, velocity: f32) -> Paddle {
        Paddle { side: side, width: PADDLE_WIDTH, height: PADDLE_HEIGHT, velocity }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32
}

impl Ball {
    fn new(velocity: [f32; 2], radius: f32) -> Ball {
        Ball {velocity, radius}
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity
}

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // wait one second before spawning the ball
        self.ball_spawn_timer.replace(1.0);
        
        // Load the spritesheet necessary to render the graphics.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
        initialise_scoreboard(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            }
            else {
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }
}

/// Setup camera in way that our screen covers whole arena and 
/// (0, 0) is in the bottom left.
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT * 0.5;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0); 

    world.create_entity()
        .with(Paddle::new(Side::Left, PADDLE_VELOCITY))
        .with(left_transform)
        .with(sprite_render.clone())
        .build();

    world.create_entity()
        .with(Paddle::new(Side::Right, PADDLE_VELOCITY))
        .with(right_transform)
        .with(sprite_render)
        .build();
}

/// Initialises one ball
fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    world.register::<Ball>();

    // Assign the sprite for the ball
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1); 

    world.create_entity()
        .with(Ball::new(
            [BALL_VELOCITY_X, BALL_VELOCITY_Y], BALL_RADIUS
        ))
        .with(local_transform)
        .with(sprite_render)
        .build();
}

fn initialise_scoreboard(world: &mut World) {
    let font = {
        let loader = world.read_resource::<Loader>();
        let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
        loader.load(
            "fonts/square.ttf",
            TtfFormat,
            (),
            &font_storage,
        )
    };

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle, 
        -50.0, -50.0, 1.0, 200.0, 50.0
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        50., -50., 1., 200., 50.,
    );

    let p1_score = world.create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
            LineMode::Single,
            Anchor::Middle
        ))
        .build();

    let p2_score = world.create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font,
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(ScoreText { p1_score, p2_score});
}

/// Load the sprite sheet necessary to render the graphics.
/// The texture is the pixel data
/// `texture_handle` is a cloneable reference to the texture
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}