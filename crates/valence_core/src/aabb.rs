use glam::DVec3;

/// An axis-aligned bounding box. `min` is expected to be <= `max`
/// componentwise.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Aabb {
    pub min: DVec3,
    pub max: DVec3,
}

impl Aabb {
    pub fn new(p0: impl Into<DVec3>, p1: impl Into<DVec3>) -> Self {
        let p0 = p0.into();
        let p1 = p1.into();
        Self {
            min: p0.min(p1),
            max: p0.max(p1),
        }
    }

    pub fn from_bottom_size(bottom: impl Into<DVec3>, size: impl Into<DVec3>) -> Self {
        let bottom = bottom.into();
        let size = size.into();

        Self {
            min: DVec3 {
                x: bottom.x - size.x / 2.0,
                y: bottom.y,
                z: bottom.z - size.z / 2.0,
            },
            max: DVec3 {
                x: bottom.x + size.x / 2.0,
                y: bottom.y + size.y,
                z: bottom.z + size.z / 2.0,
            },
        }
    }
}
