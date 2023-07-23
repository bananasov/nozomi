local base_address = sdk.memory.get_base_address()
local local_player_address = 0x17E0A8;
local health_offset = 0xEC;

local player = sdk.memory.read_usize(base_address + local_player_address)
local health = sdk.memory.read_u8(player + health_offset)
print(health)
sdk.memory.write_u8(player + health_offset, 101)