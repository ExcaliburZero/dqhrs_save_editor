#pragma author ExcaliburZero
#pragma description DQH: Rocket Slime save file
#pragma endian little

struct ItemEntry {
    bool obtained;
    u8 count;
    u16 unkown;
};

struct MonsterEntry {
    bool obtained;
    u8 count;
    u16 unkown;
};

enum TankUpgrade : u8 {
    Locked = 0x00,
    Unlocked = 0x01,
    Purchased = 0x03
};

char magic[10] @ 0x00;

ItemEntry items[60] @ 0x8A;
MonsterEntry monsters[20] @ 0x17A;

TankUpgrade tank_upgrades[20] @ 0x2F1;
u16 tank_health @ 0x305;
u8 ammo[30] @ 0x30E;

u32 gold @ 0x335;
u32 playtime_in_frames @ 0x339;

u8 name[0x08] @ 0x33F;

u8 fixed_0x34D[4] @ 0x34D;
u8 checksum[2] @ 0x352;