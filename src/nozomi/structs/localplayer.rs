use super::vector::Vector3;

#[derive(Debug)]
#[repr(C)]
pub struct LocalPlayer {
    _pad_0000: [i8; 4], //0x0000
    pub position: Vector3,
    _pad_0010: [i8; 220], //0x0010
    pub health: i32, //0x00EC
    pub armor: i32, //0x00F0
    #[allow(non_snake_case)]
    _pad_00F4: [i8; 20], //0x00F4
    pub pistol_reserve: i32, //0x0108
    pub carbine_reserve: i32, //0x010C
    pub shotgun_reserve: i32, //0x0110
    pub smg_reserve: i32, //0x0114
    pub sniper_reserve: i32, //0x0118
    pub ar_reserve: i32, //0x011C
    _pad_0120: [i8; 4], //0x0120
    pub dual_reserve: i32, //0x0124
    _pad_0128: [i8; 4], //0x0128
    pub pistol_ammo: i32, //0x012C
    pub carbine_ammo: i32, //0x0130
    pub shotgun_ammo: i32, //0x0134
    pub smg_ammo: i32, //0x0138
    pub sniper_ammo: i32, //0x013C
    pub ar_ammo: i32, //0x0140
    pub grenade_count: i32, //0x0144
    pub dual_ammo: i32, //0x0148
}