
use amethyst::{
    DataDispose, Error, GameData, StateData, 
    assets::{AssetStorage, Handle, Loader}, 
    core::{ArcThreadPool, SystemBundle, Transform, math::{Vector2, Vector3}}, 
    input::{VirtualKeyCode, is_close_requested, is_key_down}, 
    prelude::*, 
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}, 
    shred::{Dispatcher, DispatcherBuilder, System, World}, 
    ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform}
};

use rand::{thread_rng, Rng};

pub mod components;
use crate::game::components::{Physics};
use crate::config::BoidConfig;

use self::components::{Boid, FpsText};

pub const GAME_WIDTH: f32 = 800.0;
pub const GAME_HEIGHT: f32 = 600.0;

struct Main;
struct Paused;

impl<'a, 'b> State<BoidGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<BoidGameData>) {
        create_paused_ui(data.world);
    }

    fn handle_event(
        &mut self,
        data: StateData<BoidGameData>,
        event: StateEvent
    ) -> Trans<BoidGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                delete_paused_ui(data.world);
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<BoidGameData>) -> Trans<BoidGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }
}

impl<'a, 'b> State<BoidGameData<'a, 'b>, StateEvent> for Main {
    fn on_start(&mut self, data: StateData<BoidGameData>) {
        init(data.world);
    }

    fn handle_event(
        &mut self,
        _: StateData<BoidGameData>,
        event: StateEvent
    ) -> Trans<BoidGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                Trans::Push(Box::new(Paused))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<BoidGameData>) -> Trans<BoidGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }
}

pub struct BoidGameData<'a, 'b> {
    core_dispatcher: Option<Dispatcher<'a, 'b>>,
    running_dispatcher: Option<Dispatcher<'a, 'b>>
}

impl<'a, 'b> BoidGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            if let Some(dispatcher) = self.running_dispatcher.as_mut() {
                dispatcher.dispatch(&world);
            }
        }
        if let Some(dispatcher) = self.core_dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }
    }
}

pub struct BoidGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>
}

impl<'a, 'b> Default for BoidGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        BoidGameDataBuilder::new()
    }
}

impl<'a, 'b> BoidGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        BoidGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new()
        }
    }

    pub fn with_base_bundle<B>(mut self, world: &mut World, bundle: B) -> Result<Self, Error>
    where
        B: SystemBundle<'a, 'b>
    {
        bundle.build(world, &mut self.core)?;
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a
    {
     self.running.add(system, name, dependencies);
     self   
    }
}

impl<'a, 'b> DataInit<BoidGameData<'a, 'b>> for BoidGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> BoidGameData<'a, 'b> {
        let pool = (*world.read_resource::<ArcThreadPool>()).clone();
        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();
        core_dispatcher.setup(world);
        running_dispatcher.setup(world);

        BoidGameData { 
            core_dispatcher: Some(core_dispatcher), 
            running_dispatcher: Some(running_dispatcher) 
        }
    }
}

impl<'a, 'b> DataDispose for BoidGameData<'a, 'b> {
    fn dispose(&mut self, world: &mut World) {
        if let Some(dispatcher) = self.core_dispatcher.take() {
            dispatcher.dispose(world);
        }
        if let Some(dispatcher) = self.running_dispatcher.take() {
            dispatcher.dispose(world);
        }
    }
}

impl<'a, 'b> DataDispose for BoidGameDataBuilder<'a, 'b> {
    fn dispose(&mut self, world: &mut World) {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct BoidsState {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    font_handle: Option<Handle<FontAsset>>
}

impl SimpleState for BoidsState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        
        // Load the spritesheet necessary to render the graphics.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        // Load font
        self.font_handle.replace(load_font(world));

        init_boids(world, self.sprite_sheet_handle.clone().unwrap());
        init_ui(world, self.font_handle.clone().unwrap());
        init_camera(world);
    }
} 

fn init(world: &World) {}
fn create_paused_ui(world: &World) {}
fn delete_paused_ui(world: &World) {}

fn init_boids(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Assign the sprite for the ball
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0); 
    let mut rng = thread_rng();
    let (boid_count, boid_scale) = {
        let config = &world.read_resource::<BoidConfig>();
        (config.boid_count, config.boid_size / 128.0)
    };

    for _i in 0..boid_count {
        println!("add boid");
        let boid = Boid {
            alignment: Vector2::zeros(),
            attraction: Vector2::zeros(),
            repulsion: Vector2::zeros()
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(
            rng.gen_range(0.0..GAME_WIDTH) - GAME_WIDTH * 0.5, 
            rng.gen_range(0.0..GAME_HEIGHT) - GAME_HEIGHT * 0.5, 
            0.0
        );
        transform.set_scale(Vector3::new(boid_scale, boid_scale, boid_scale));

        let physics = Physics { 
            velocity: (Vector2::new_random() - Vector2::new(0.5, 0.5)) * 50.0,
            acceleration: Vector2::new(0.0, 0.0) 
        };

        world.create_entity()
            .with(transform)
            .with(physics)
            .with(boid)
            .with(sprite_render.clone())
            .build();
    }
}

fn init_ui(world: &mut World, font_handle: Handle<FontAsset>) {
    let ui_transform = UiTransform::new(
        String::from("simple_button"),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.0,
        0.0,
        0.0,
        200.0,
        30.0
    );

    let ui_text = UiText::new(
        font_handle,
        String::from("Simple button"),
        [1.0, 1.0, 1.0, 0.5],
        25.0,
        LineMode::Single,
        Anchor::MiddleLeft
    );

    let fps_entity = world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .build();

    world.insert(FpsText { entity: fps_entity });
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}

fn load_font(world: &mut World) -> Handle<FontAsset> {
    let loader = world.read_resource::<Loader>();
    let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
    loader.load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &font_storage,
    )
}

/// Load the sprite sheet necessary to render the graphics.
/// The texture is the pixel data
/// `texture_handle` is a cloneable reference to the texture
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/boid_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/boid_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}