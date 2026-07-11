use std::io::Cursor;

use binrw::{BinRead, BinResult, BinWrite};

#[derive(BinRead, BinWrite, Debug, PartialEq)]
#[brw(little)]
pub struct ItemEntry {
    obtained: u8,
    count: u8,
    unknown: u16,
}

impl ItemEntry {
    pub fn new(unknown: u16, obtained: u8, count: u8) -> ItemEntry {
        ItemEntry {
            unknown,
            obtained,
            count,
        }
    }
}

#[derive(BinRead, BinWrite, Debug, PartialEq)]
#[brw(little)]
pub struct MonsterEntry {
    obtained: u8,
    count: u8,
    unknown: u16,
}

impl MonsterEntry {
    pub fn new(unknown: u16, obtained: u8, count: u8) -> MonsterEntry {
        MonsterEntry {
            unknown,
            obtained,
            count,
        }
    }
}

#[derive(BinRead, BinWrite, Debug, PartialEq)]
#[brw(repr(u8))]
pub enum TankUpgrade {
    Locked = 0,
    Unlocked = 1,
    Purchased = 3,
}

#[derive(BinRead, BinWrite, Debug, PartialEq)]
#[brw(magic = b"hiyama_v1", little)]
pub struct SaveFile {
    unknown_a: [u8; 0x81],
    pub items: [ItemEntry; 60],
    pub monsters: [MonsterEntry; 20],
    unknown_ba: [u8; 295],
    pub tank_upgrades: [TankUpgrade; 20],
    pub tank_health: u16,
    unknown_bba: [u8; 1],
    pub crew: [u8; 3],
    unknown_bbb: [u8; 3],
    pub ammo: [u8; 30],
    unknown_c: [u8; 9],
    pub gold: u32,
    pub playtime_in_frames: u32,
    unknown_d: [u8; 2],
    pub name: [u8; 8],
    unknown_e: [u8; 11],
    pub checksum: u16,
    pub unknown_f: [u8; 0x1CAC],
}

impl SaveFile {
    pub fn calculate_checksum(&self) -> BinResult<u16> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(&mut bytes);
        self.write(&mut writer)?;

        Ok(get_crc16(0x11, &bytes[0..0x352]))
    }

    pub fn update_checksum(&mut self) -> BinResult<()> {
        self.checksum = self.calculate_checksum()?;

        Ok(())
    }
}

/// Reimplementation of the Nintendo DS BIOS SWI 0x0E (GetCRC16).
///
/// https://www.problemkaputt.de/gbatek-bios-misc-functions.htm
fn get_crc16(initial_crc: u16, data: &[u8]) -> u16 {
    let mut crc = initial_crc;

    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use binrw::BinRead;
    use rstest::rstest;

    use super::*;

    fn read_save_from_file(filepath: &str) -> SaveFile {
        let mut reader = File::open(filepath).unwrap();
        SaveFile::read(&mut reader).unwrap()
    }

    #[test]
    fn test_read_save_file() {
        let actual = read_save_from_file("test/data/01 - First save opportunity.sav");

        let expected_items: [ItemEntry; 60] = std::array::from_fn(|i| match i {
            0 | 1 => ItemEntry::new(0, 1, 3),
            _ => ItemEntry::new(0, 0, 0),
        });
        assert_eq!(expected_items, actual.items);

        let expected_monsters: [MonsterEntry; 20] =
            std::array::from_fn(|_| MonsterEntry::new(0, 0, 0));

        assert_eq!(expected_monsters, actual.monsters);

        let expected_tank_upgrades: [TankUpgrade; 20] =
            std::array::from_fn(|_| TankUpgrade::Locked);
        assert_eq!(expected_tank_upgrades, actual.tank_upgrades);

        let expected_tank_health = 100;
        assert_eq!(expected_tank_health, actual.tank_health);

        let expected_crew = [255, 255, 255];
        assert_eq!(expected_crew, actual.crew);

        let expected_ammo = [
            101, 101, 13, 101, 101, 13, 101, 101, 13, 101, 101, 13, 101, 101, 13, 101, 101, 13,
            101, 101, 13, 101, 101, 13, 101, 101, 13, 101, 101, 13,
        ];
        assert_eq!(expected_ammo, actual.ammo);

        let expected_gold = 76;
        assert_eq!(expected_gold, actual.gold);

        let expected_playtime = 56027;
        assert_eq!(expected_playtime, actual.playtime_in_frames);

        let expected_name = [28, 51, 39, 47, 41, 56, 0, 0]; // Rocket
        assert_eq!(expected_name, actual.name);
    }

    #[rstest]
    #[case("test/data/01 - First save opportunity.sav")]
    fn test_calculate_checksum(#[case] filepath: &str) {
        let save_file = read_save_from_file(filepath);

        let expected = save_file.checksum;
        let actual = save_file.calculate_checksum().unwrap();

        assert_eq!(expected, actual);
    }
}
