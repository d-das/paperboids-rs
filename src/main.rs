extern crate native;
extern crate kiss3d;
extern crate nalgebra;

use std::rc::Rc;
use std::cell::RefCell;
use std::rand;
use nalgebra::na::Vec3;
use nalgebra::na;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use kiss3d::resource::Mesh;
use kiss3d::scene::SceneNode;
use kiss3d::light;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

//TODO: check out recorder demo to create mpg's of simulations

struct Plane {
    pos: Vec3<f32>,
    vel: Vec3<f32>,
    acc: Vec3<f32>,

    node: SceneNode,
}

impl Plane {
    fn gen_mesh() -> Rc<RefCell<Mesh>> {
        let vertices = vec!(
            Vec3::new(0.0, 0.0, 1.0), // front / nose
            Vec3::new(0.75, 0.0, -1.0), // left wing - 'port'
            Vec3::new(-0.75, 0.0, -1.0), // right wing - 'starboard'
            Vec3::new(0.0, 0.0, -1.0), // back midpoint between wings
            Vec3::new(0.0, -0.4, -1.0), // back bottom fin
        );

        let indices = vec!(
            Vec3::new(0u32, 1, 3),
            Vec3::new(0u32, 3, 2),
            Vec3::new(0u32, 4, 3),
        );

        Rc::new(RefCell::new(Mesh::new(vertices, indices, None, None, false)))
    }

    fn new(sn: SceneNode) -> Plane {
        let mut node = sn;
        node.set_color(1.0, 1.0, 1.0);
        node.enable_backface_culling(false);
        let (x, z) = (rand::random::<f32>() * 5.0 - 2.5, rand::random::<f32>() * 5.0 - 2.5);
        node.set_local_translation(Vec3::new(x, 2.0, z));
        node.set_points_size(1.0); //wireframe mode for plane
        node.set_lines_width(1.0);
        node.set_surface_rendering_activation(false);

        let (vx, vz) = (rand::random::<f32>() * 4.0 - 2.0, rand::random::<f32>() * 4.0 - 2.0);

        Plane {
            pos: Vec3::new(0.0, 2.0, 0.0),
            vel: Vec3::new(vx, 0.0, vz),
            acc: Vec3::new(0.0, 0.0, 0.0),
            node: node,
        }
    }

    fn update(&mut self, dt: f32) {
        let dtv = Vec3::new(dt, dt, dt);
        self.vel = self.vel + self.acc * dtv;
        self.pos = self.pos + self.vel * dtv;
        self.node.look_at_z(&self.pos, &(self.pos + self.vel), &Vec3::y());
    }
}



fn main() {
    // have camera start from higher position
    let eye = Vec3::new(5.0, 5.0, 10.0);
    let at = na::one();
    let mut arc_ball = ArcBall::new(eye, at);

    let mut window = Window::new("Kiss3d: cube");
    window.set_framerate_limit(Some(60));
    window.set_light(light::StickToCamera);

    let mut ground = window.add_quad(10.0, 10.0, 1, 1);
    ground.set_local_rotation(Vec3::new((90.0f32).to_radians(), 0.0, 0.0));
    ground.set_color(0.2, 0.2, 0.2);

    let pmesh = Plane::gen_mesh();

    let mut ps = Vec::new();
    for i in range(0i, 20) {
        ps.push(Plane::new(window.add_mesh(pmesh.clone(), Vec3::new(0.2, 0.2, 0.2))));
    }

    let mut last_time = window.context().get_time() as f32;
    let mut curr_time;
    while window.render_with_camera(&mut arc_ball) {
        curr_time = window.context().get_time() as f32;

        draw_axis(&mut window);

        for p in ps.mut_iter() {
            p.update(curr_time - last_time);

            p.acc.x =
                if p.pos.x > 5.0 {
                    -2.0
                } else if p.pos.x < -5.0 {
                    2.0
                } else {
                    0.0
                };

            p.acc.z =
                if p.pos.z > 5.0 {
                    -2.0
                } else if p.pos.z < -5.0 {
                    2.0
                } else {
                    0.0
                };
        }

        last_time = curr_time;
    }
}

fn draw_axis(w: &mut Window) {
    let o: Vec3<f32> = na::zero();
    let x = Vec3::x();
    let y = Vec3::y();
    let z = Vec3::z();

    w.draw_line(&o, &x, &x);
    w.draw_line(&o, &y, &y);
    w.draw_line(&o, &z, &z);
}
