extern crate cgmath;
extern crate rand;

use cgmath::*;
use aabb::AABB;

pub struct Boid {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub acc: Vector3<f32>,
}

impl Boid {
    pub fn random_new(bbox: &AABB) -> Boid {
        use rand::Rng;
        let mut rand = rand::weak_rng();

        let x = bbox.l.x + rand.next_f32() * bbox.xlen();
        let y = bbox.l.y + rand.next_f32() * bbox.ylen();
        let z = bbox.l.z + rand.next_f32() * bbox.zlen();

        let vx = 10.0 - rand.next_f32() * 20.0;
        let vy = 10.0 - rand.next_f32() * 20.0;
        let vz = 10.0 - rand.next_f32() * 20.0;

        Boid {
            pos: Vector3::new(x, y, z),
            vel: Vector3::new(vx, vy, vz),
            acc: Vector3::zero(),
        }
    }

    pub fn update(&mut self, dt: f32, world_scale: f32) {
        // TODO: figure out where to put these speed constants
        let max_speed = 25.0 * world_scale;
        let min_speed = 4.0 * world_scale;

        self.vel = self.vel + self.acc.mul_s(dt);

        let curr_speed = self.vel.length();
        if curr_speed > max_speed {
            self.vel = self.vel.mul_s(max_speed / curr_speed);
        } else if curr_speed < min_speed {
            self.vel = self.vel.mul_s(min_speed / curr_speed);
        }

        self.pos = self.pos + self.vel.mul_s(dt);
    }

    pub fn model(&self) -> Matrix4<f32> {
        // TODO: figure out 'up' vector to get bank rotation animation

        // note: replicating cgmath's Matrix3::look_at but without the added transpose
        // this logic rotates the boid correctly, not sure if the transpose is a bug or not
        let dir = self.vel;
        let up = Vector3::unit_y();

        let dir = dir.normalize();
        let side = up.cross(&dir).normalize();
        let up = dir.cross(&side).normalize();
        let m3 = Matrix3::from_cols(side, up, dir);

        Matrix4::from_translation(&self.pos) * Matrix4::from(m3)
    }
}

