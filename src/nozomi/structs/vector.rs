use mlua::{MultiValue, UserData};

#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

impl UserData for Vector3 {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_set("x", |_, this, val| {
            this.x = val;
            Ok(())
        });

        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_set("y", |_, this, val| {
            this.y = val;
            Ok(())
        });

        fields.add_field_method_get("z", |_, this| Ok(this.z));
        fields.add_field_method_set("z", |_, this, val| {
            this.z = val;
            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // TODO: Add (meta)methods for adding and shit.
        methods.add_method("add", |_, _this, _args: MultiValue| Ok(()));
    }
}
