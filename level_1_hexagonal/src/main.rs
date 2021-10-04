mod hexagonal;

use hexagonal::Coord;
use hexagonal::Orientation;
use hexagonal::Point;
use hexagonal::Layout;
use hexagonal::Map;
use hexagonal::MapShape;

pub struct App {
    map: Map::<i32>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

            for (hexa, val) in self.map.ground.iter() {
                layout.polygon_corners(hexa).iter().map(|p| Point2::new(p.0, p.1)).collect(), Vector2::new(1.0, 1.0)
                polygon(RED, polygon, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() -> Result<(), String> {
    let layout = Layout::new(
        Orientation::POINTY,
        Point::new(30.0, -30.0),
        Point::new(0.0, 0.0)
    );
    const SIZE: i32 = 64;
    fn generator(c: Coord) -> i32 {
        // if c.x() == 0 {
        //     0xff0000
        // }
        // else if c.y() == 0 {
        //     0x00ff00
        // }
        // else if c.z() == 0 {
        //     0x0000ff
        // }
        // else {
        //     0x888888
        // }
        let red = (((c.x() + SIZE) * 255 / (SIZE * 2)) & 0xff) << 16;
        let green = (((c.y() + SIZE) * 255 / (SIZE * 2)) & 0xff) << 8;
        let blue = ((c.z() + SIZE) * 255 / (SIZE * 2)) & 0xff;
        // println!("{} {} {}", red, green, blue);
        red + green + blue
    }

    let map = Map::<i32>::new(MapShape::Hexagon { radius: SIZE, generator });
    println!("Map generated: {} hexagons", map.ground.len());

    for (hexa, val) in map.ground.iter() {
    }


    Ok(())
}


// use kiss3d::camera::ArcBall;
// use kiss3d::event::WindowEvent;
// use kiss3d::nalgebra::Point2;
// use kiss3d::nalgebra::Point3;
// use kiss3d::nalgebra::Vector2;
// use kiss3d::planar_camera::Sidescroll;
// use kiss3d::window::Window;
// use kiss3d::light::Light;

// fn main_kiss3d() -> Result<(), String> {
//     let mut window = Window::new("Kiss3d: cube");
//     // let mut c = window.add_cube(1.0, 1.0, 1.0);
//     let layout = Layout::new(
//         Orientation::POINTY,
//         Point::new(30.0, -30.0),
//         Point::new(0.0, 0.0)
//     );

//     const SIZE: i32 = 64;

//     fn generator(c: Coord) -> i32 {
//         // if c.x() == 0 {
//         //     0xff0000
//         // }
//         // else if c.y() == 0 {
//         //     0x00ff00
//         // }
//         // else if c.z() == 0 {
//         //     0x0000ff
//         // }
//         // else {
//         //     0x888888
//         // }
//         let red = (((c.x() + SIZE) * 255 / (SIZE * 2)) & 0xff) << 16;
//         let green = (((c.y() + SIZE) * 255 / (SIZE * 2)) & 0xff) << 8;
//         let blue = ((c.z() + SIZE) * 255 / (SIZE * 2)) & 0xff;
//         // println!("{} {} {}", red, green, blue);
//         red + green + blue
//     }

//     let map = Map::<i32>::new(MapShape::Hexagon { radius: SIZE, generator });

//     for (hexa, val) in map.ground.iter() {
//         let mut h = window.add_convex_polygon(layout.polygon_corners(hexa).iter().map(|p| Point2::new(p.0, p.1)).collect(), Vector2::new(1.0, 1.0));
//         h.set_color(
//             (val >> 16 & 0xff) as f32 / 255.0, 
//             (val >> 8 & 0xff) as f32 / 255.0, 
//             (val & 0xff) as f32 / 255.0
//         );
//     }
//     println!("Map generated: {} hexagons", map.ground.len());

//     // let hexa = Coord::new(0, 0, 0)?;
//     // let hexa = Coord::new(1, 0, -1)?;
//     // let mut h = window.add_convex_polygon(layout.polygon_corners(hexa).iter().map(|p| Point2::new(p.0, p.1)).collect(), Vector2::new(1.0, 1.0));
//     // h.set_color(0.0, 1.0, 0.0);

//     window.set_light(Light::StickToCamera);
//     let mut cam = ArcBall::new(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
//     let mut planar_cam = Sidescroll::new();
    

//     while window.render_with_cameras(&mut cam, &mut planar_cam) {
//         // let mut em = window.events();
//         // for event in em.iter() {
//         //     match event.value {
//         //         WindowEvent::CursorPos(x, y, o) => {
//         //             let x_off = window.width() >> 1;
//         //             let y_off = window.height() >> 1;
//         //             let at = planar_cam.at();
//         //             let zoom = planar_cam.zoom();
//         //             let x_layout = (x as f32 - x_off as f32) / zoom + at.x;
//         //             let y_layout = (y as f32 - y_off as f32) / zoom - at.y;
//         //             println!("{} {}", x_layout, y_layout);
//         //             let coord = layout.pixel_to_hex(&Point::new(x_layout, -y_layout));
//         //             let val = match map.ground.get(&Coord::from(coord)) {
//         //                 Some(&val) => val,
//         //                 None => 0
//         //             };
//         //             println!("{}: {} {} {}", 
//         //                 Coord::from(coord),
//         //                 (val >> 16 & 0xff) as f32 / 255.0, 
//         //                 (val >> 8 & 0xff) as f32 / 255.0, 
//         //                 (val & 0xff) as f32 / 255.0
//         //             );
//         //         },
//         //         _ => {}
//         //     }
//         // }
//     }

//     Ok(())
// }