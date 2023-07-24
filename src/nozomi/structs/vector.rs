#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn add(self, other: Vector3) -> Vector3 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        Vector3 { x, y, z }
    }

    pub fn subtract(self, other: Vector3) -> Vector3 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        Vector3 { x, y, z }
    }

    pub fn multiply(self, other: Vector3) -> Vector3 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        Vector3 { x, y, z }
    }
}