use ::nalgebra::{
    PerspMat3,
    Mat4,
    Iso3,
    Rot3,
    Pnt3,
    Vec3
};

/// Stateful FreeCamera for moving around in first-person.
pub struct FreeCamera {
    pub pos        : Pnt3<f32>,
    pub projection : PerspMat3<f32>,
    look           : Vec3<f32>,
    right          : Vec3<f32>,
    up             : Vec3<f32>
}

impl FreeCamera {
    /// Creates as new camera looking down the -z axis.
    pub fn new(
        aspect: f32, fov_degrees: f32, near_clip: f32, far_clip: f32
    ) -> FreeCamera {

        let fov_rad = 3.1415 / 180.0 * fov_degrees;
        FreeCamera {
            pos        : Pnt3::new(0.0, 0.0, 0.0),
            projection : PerspMat3::new(aspect, fov_rad, near_clip, far_clip),
            look       : Vec3::new(0.0, 0.0, -1.0),
            right      : Vec3::new(1.0, 0.0, 0.0),
            up         : Vec3::new(0.0, 1.0, 0.0),
        }
    }

    /// Returns the matrix representing what the camera is looking at
    pub fn get_view_matrix(&self) -> Mat4<f32> {
        ::nalgebra::to_homogeneous(&self.get_view_transform())
    }

    /// Returns the camera transform as an Iso3
    pub fn get_view_transform(&self) -> Iso3<f32> {
        let mut view = Iso3::new(::nalgebra::zero(), ::nalgebra::zero());
        view.look_at_z(
            &self.pos,
            &(self.pos + self.look),
            &self.up
        );
        ::nalgebra::inv(&view).unwrap()
    }

    /// Moves the camera in the direction of the look vector.
    pub fn advance(&mut self, dist: f32) {
        self.pos = self.pos + self.look * dist;
    }

    /// Moves the camera in the direction of the right vector.
    pub fn strafe(&mut self, dist: f32) {
        self.pos = self.pos + self.right * dist;
    }

    /// Rotates the look vector around the right vector, to look up/down
    pub fn rotate_up(&mut self, angle_in_degrees: f32) {
        let angle = angle_in_degrees * 3.1415 / 180.0;
        let rot = Rot3::new(self.right*angle);

        self.look  = rot * self.look;
        self.up    = rot * self.up;
    }

    /// Rotates the look vector around the y axis, to look left/right
    pub fn rotate_left(&mut self, angle_in_degrees: f32) {
        let angle = angle_in_degrees * 3.1415 / 180.0;
        let rot = Rot3::new(Vec3::new(0.0, angle, 0.0));

        self.look  = rot * self.look;
        self.right = rot * self.right;
        self.up    = rot * self.up;
    }
}
