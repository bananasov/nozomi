use mlua::{MetaMethod, UserData};

#[derive(Copy, Clone, Debug)]
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

    pub fn add(&self, other: &Vector3) -> Vector3 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        Vector3 { x, y, z }
    }

    pub fn subtract(&self, other: &Vector3) -> Vector3 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        Vector3 { x, y, z }
    }

    pub fn multiply(&self, other: &Vector3) -> Vector3 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        Vector3 { x, y, z }
    }

    pub fn division(&self, other: &Vector3) -> Vector3 {
        let x = self.x / other.x;
        let y = self.y / other.y;
        let z = self.z / other.z;

        Vector3 { x, y, z }
    }
}

// impl<'lua> FromLua<'lua> for Vector3 {
//     fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
//         match lua_value {
//             Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
//             _ => unreachable!()
//         }
//     }
// }

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
        methods.add_method("add", |_, this, other: &Vector3| {
            let new_vec = this.add(other);
            Ok(new_vec)
        });
        methods.add_meta_function(MetaMethod::Add, |_, (left, right): (&Vector3, &Vector3)| {
            let new_vec = left.add(right);
            Ok(new_vec)
        });

        methods.add_method("sub", |_, this, other: &Vector3| {
            let new_vec = this.subtract(other);
            Ok(new_vec)
        });
        methods.add_meta_function(MetaMethod::Sub, |_, (left, right): (&Vector3, &Vector3)| {
            let new_vec = left.subtract(right);
            Ok(new_vec)
        });

        methods.add_method("mul", |_, this, other: &Vector3| {
            let new_vec = this.multiply(other);
            Ok(new_vec)
        });
        methods.add_meta_function(MetaMethod::Mul, |_, (left, right): (&Vector3, &Vector3)| {
            let new_vec = left.multiply(right);
            Ok(new_vec)
        });

        methods.add_method("div", |_, this, other: &Vector3| {
            let new_vec = this.division(other);
            Ok(new_vec)
        });
        methods.add_meta_function(MetaMethod::Div, |_, (left, right): (&Vector3, &Vector3)| {
            let new_vec = left.division(right);
            Ok(new_vec)
        });

        methods.add_meta_function(MetaMethod::Call, |_, ()| {
            Ok(Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
        });
    }
}

impl UserData for &Vector3 {}
