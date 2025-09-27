// src/patches/shops.rs
use crate::rom::Rom;

/// Shop catalog IDs used by shop tables (not the in-world sprite IDs).
/// Mirrors ShopRandomizer.Id (values shown in hex).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopItemId {
    // Weapons
    Dagger = 0x00,
    Longsword = 0x01,
    Giantblade = 0x02,
    Dragonslayer = 0x03,

    // Armor
    LeatherArmour = 0x20,
    StuddedMail = 0x21,
    FullPlate = 0x22,
    BattleSuit = 0x23,

    // Shields & helm
    SmallShield = 0x40,
    LargeShield = 0x41,
    MagicShield = 0x42,
    BattleHelmet = 0x43,

    // Spells
    Deluge = 0x60,
    Thunder = 0x61,
    Fire = 0x62,
    Death = 0x63,
    Tilte = 0x64,

    // Rings / keys / quest-ish
    ElfRing = 0x80,
    RubyRing = 0x81,
    DworfRing = 0x82,
    DemonRing = 0x83,
    AceKey = 0x84,
    KingKey = 0x85,
    QueenKey = 0x86,
    JackKey = 0x87,
    JokerKey = 0x88,
    Mattock = 0x89,
    Rod = 0x8A,
    Crystal = 0x8B,
    Lamp = 0x8C,
    Hourglass = 0x8D,
    Book = 0x8E,
    Wingboots = 0x8F,

    // Consumables / misc
    RedPotion = 0x90,
    BlackPotion = 0x91,
    Elixir = 0x92,
    Pendant = 0x93,
    BlackOnyx = 0x94,
    FireCrystal = 0x95,
}

impl ShopItemId {
    pub fn from_byte(b: u8) -> Option<Self> {
        use ShopItemId::*;
        Some(match b {
            0x00 => Dagger,
            0x01 => Longsword,
            0x02 => Giantblade,
            0x03 => Dragonslayer,
            0x20 => LeatherArmour,
            0x21 => StuddedMail,
            0x22 => FullPlate,
            0x23 => BattleSuit,
            0x40 => SmallShield,
            0x41 => LargeShield,
            0x42 => MagicShield,
            0x43 => BattleHelmet,
            0x60 => Deluge,
            0x61 => Thunder,
            0x62 => Fire,
            0x63 => Death,
            0x64 => Tilte,
            0x80 => ElfRing,
            0x81 => RubyRing,
            0x82 => DworfRing,
            0x83 => DemonRing,
            0x84 => AceKey,
            0x85 => KingKey,
            0x86 => QueenKey,
            0x87 => JackKey,
            0x88 => JokerKey,
            0x89 => Mattock,
            0x8A => Rod,
            0x8B => Crystal,
            0x8C => Lamp,
            0x8D => Hourglass,
            0x8E => Book,
            0x8F => Wingboots,
            0x90 => RedPotion,
            0x91 => BlackPotion,
            0x92 => Elixir,
            0x93 => Pendant,
            0x94 => BlackOnyx,
            0x95 => FireCrystal,
            _ => return None,
        })
    }
}

/// A single shop item entry: catalog id + 16-bit little-endian price.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShopItem {
    pub id: ShopItemId,
    pub price: u16,
}

impl ShopItem {
    pub fn from_rom(rom: &Rom, file_off: usize) -> Option<Self> {
        let idb = rom.read_byte(file_off);
        let lo = rom.read_byte(file_off + 1) as u16;
        let hi = rom.read_byte(file_off + 2) as u16;
        let price = lo | (hi << 8);
        let id = ShopItemId::from_byte(idb)?;
        Some(Self { id, price })
    }

    pub fn write_rom(&self, rom: &mut Rom, file_off: usize) {
        rom.write_byte(file_off, self.id as u8);
        rom.write_byte(file_off + 1, (self.price & 0xFF) as u8);
        rom.write_byte(file_off + 2, (self.price >> 8) as u8);
    }
}

/// Shop IDs matching the randomizer’s `Shop.Id`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShopId {
    EolisKeyShop = 0,
    EolisItemShop = 1,
    ApoluneKeyShop = 2,
    ApoluneSecretShop = 3,
    ApoluneItemShop = 4,
    ForepawKeyShop = 5,
    ForepawItemShop = 6,
    MasconKeyShop = 7,
    MasconItemShop = 8,
    MasconSecretShop = 9,
    VictimKeyShop = 10,
    VictimItemShop = 11,
    ConflateItemShop = 12,
    DaybreakKeyShop = 13,
    DaybreakItemShop = 14,
    DartmoorKeyShop = 15,
    DartmoorItemShop = 16,
}

/// For each shop, the exact **file offsets** of its item entries (not CPU).
/// These come directly from ShopRandomizer.cs (e.g., `new ShopItem(0x3243E, content)`).
fn shop_item_offsets(shop: ShopId) -> &'static [usize] {
    match shop {
        ShopId::EolisKeyShop => &[0x3258A],
        ShopId::EolisItemShop => &[0x3243E, 0x32441, 0x32444, 0x32447],

        ShopId::ApoluneKeyShop => &[0x3258E],
        ShopId::ApoluneSecretShop => &[0x32458, 0x3245B],
        ShopId::ApoluneItemShop => &[0x3244B, 0x3244E, 0x32451, 0x32454],

        ShopId::ForepawKeyShop => &[0x32592, 0x32595],
        ShopId::ForepawItemShop => &[0x3245F, 0x32462, 0x32465, 0x32468, 0x3246B],

        ShopId::MasconKeyShop => &[0x32599, 0x3259C],
        ShopId::MasconItemShop => &[0x3246F, 0x32472, 0x32475, 0x32478],
        ShopId::MasconSecretShop => &[0x3247C, 0x3247F, 0x32482, 0x32485],

        ShopId::VictimKeyShop => &[0x325A0, 0x325A3],
        ShopId::VictimItemShop => &[0x32489, 0x3248C, 0x3248F],

        ShopId::ConflateItemShop => &[0x32493, 0x32496, 0x32499, 0x3249C],

        ShopId::DaybreakKeyShop => &[0x325A7, 0x325AA],
        // Note: randomizer uses order: A0, A6, A3
        ShopId::DaybreakItemShop => &[0x324A0, 0x324A6, 0x324A3],

        ShopId::DartmoorKeyShop => &[0x325AE],
        ShopId::DartmoorItemShop => &[0x324AA, 0x324AD],
    }
}

/// Read all items from a given shop using fixed file offsets.
pub fn read_shop(rom: &Rom, shop: ShopId) -> Vec<ShopItem> {
    let mut out = Vec::new();
    for &off in shop_item_offsets(shop) {
        if let Some(it) = ShopItem::from_rom(rom, off) {
            out.push(it);
        } else {
            // If any entry is invalid (unknown catalog id), stop early to avoid spewing junk
            break;
        }
    }
    out
}

/// Write all items back to a shop (must match the number of entries for that shop).
pub fn write_shop(rom: &mut Rom, shop: ShopId, items: &[ShopItem]) {
    let offs = shop_item_offsets(shop);
    assert_eq!(
        offs.len(),
        items.len(),
        "write_shop: item count mismatch for {:?} (have {}, expected {})",
        shop,
        items.len(),
        offs.len()
    );
    for (it, &off) in items.iter().zip(offs.iter()) {
        it.write_rom(rom, off);
    }
}

/// Convenience: dump a shop to stdout for verification.
pub fn debug_print_shop(rom: &Rom, shop: ShopId) {
    let offs = shop_item_offsets(shop);
    println!("Shop {:?} ({} entries):", shop, offs.len());
    for (idx, &off) in offs.iter().enumerate() {
        let b0 = rom.read_byte(off);
        let lo = rom.read_byte(off + 1);
        let hi = rom.read_byte(off + 2);
        let price = (lo as u16) | ((hi as u16) << 8);
        match ShopItemId::from_byte(b0) {
            Some(id) => {
                println!(
                    "  Slot {:2} @0x{:05X}: id=0x{:02X} ({:?}), price={}",
                    idx, off, b0, id, price
                );
            }
            None => {
                println!("  Slot {:2} @0x{:05X}: id=0x{:02X} (UNKNOWN), raw_price_lo=0x{:02X}, hi=0x{:02X}", idx, off, b0, lo, hi);
            }
        }
    }
}
