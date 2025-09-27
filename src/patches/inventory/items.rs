/// Item identifiers in Faxanadu, as used in the ROM.
/// These values match the in-game sprite IDs for items.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemId {
    // Consumables
    RedPotion1 = 0x28,
    RedPotion2 = 0x29,
    Ointment1 = 0x2A,
    Ointment2 = 0x2B,
    Poison1 = 0x2C,
    Poison2 = 0x2D,
    Hourglass1 = 0x2E,
    Hourglass2 = 0x2F,
    Bread = 0x30,
    Elixir = 0x31,

    // Keys
    KeyJack = 0x32,
    KeyQueen = 0x33,
    KeyKing = 0x34,
    KeyAce = 0x35,
    JokerKey = 0x36,

    // Quest Items
    Mattock = 0x37,
    MattockBossLocked = 0x38,
    Wingboots = 0x39,
    WingbootsBossLocked = 0x3A,
    Pendant = 0x3B,
    Rod = 0x3C,
    RubyRing = 0x3D,
    BlackOnyx = 0x3E,
    BattleHelmet = 0x3F,
    BattleSuit = 0x40,
    DragonSlayer = 0x41,

    // Equipment
    Dagger = 0x42,
    LongSword = 0x43,
    GiantBlade = 0x44,
    SmallShield = 0x46,
    LargeShield = 0x47,
    MagicShield = 0x48,
    Glove = 0x49,
    Glove2 = 0x4A,

    // Spells
    SpellDeluge = 0x4B,
    SpellThunder = 0x4C,
    SpellFire = 0x4D,
    SpellDeath = 0x4E,
    SpellTilte = 0x4F,

    // Gold
    Gold = 0x50,

    // Special / unused mappings (from randomizer logic)
    RingDwarf = 0x59,
    RingDemon = 0x5A,
}

impl ItemId {
    /// Convert a raw byte to an `ItemId` if valid.
    pub fn from_byte(b: u8) -> Option<Self> {
        use ItemId::*;
        match b {
            0x28 => Some(RedPotion1),
            0x29 => Some(RedPotion2),
            0x2A => Some(Ointment1),
            0x2B => Some(Ointment2),
            0x2C => Some(Poison1),
            0x2D => Some(Poison2),
            0x2E => Some(Hourglass1),
            0x2F => Some(Hourglass2),
            0x30 => Some(Bread),
            0x31 => Some(Elixir),

            0x32 => Some(KeyJack),
            0x33 => Some(KeyQueen),
            0x34 => Some(KeyKing),
            0x35 => Some(KeyAce),
            0x36 => Some(JokerKey),

            0x37 => Some(Mattock),
            0x38 => Some(MattockBossLocked),
            0x39 => Some(Wingboots),
            0x3A => Some(WingbootsBossLocked),
            0x3B => Some(Pendant),
            0x3C => Some(Rod),
            0x3D => Some(RubyRing),
            0x3E => Some(BlackOnyx),
            0x3F => Some(BattleHelmet),
            0x40 => Some(BattleSuit),
            0x41 => Some(DragonSlayer),

            0x42 => Some(Dagger),
            0x43 => Some(LongSword),
            0x44 => Some(GiantBlade),
            0x46 => Some(SmallShield),
            0x47 => Some(LargeShield),
            0x48 => Some(MagicShield),
            0x49 => Some(Glove),
            0x4A => Some(Glove2),

            0x4B => Some(SpellDeluge),
            0x4C => Some(SpellThunder),
            0x4D => Some(SpellFire),
            0x4E => Some(SpellDeath),
            0x4F => Some(SpellTilte),

            0x50 => Some(Gold),

            0x59 => Some(RingDwarf),
            0x5A => Some(RingDemon),
            _ => None,
        }
    }
}
