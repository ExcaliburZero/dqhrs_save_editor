#pragma author ExcaliburZero
#pragma description DQH: Rocket Slime save file
#pragma endian little

struct ItemEntry {
    u16 unkown;
    bool obtained;
    u8 count;
};

struct MonsterEntry {
    u16 unkown;
    bool obtained;
    u8 count;
};

char magic[9] @ 0x00;

ItemEntry items[60] @ 0x88;
MonsterEntry monsters[20] @ 0x178;

u8 ammo[30] @ 0x30E;

u32 gold @ 0x335;
u32 playtime_in_frames @ 0x339;

u8 name[0x08] @ 0x33F;
