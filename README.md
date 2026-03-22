# Faxanadu Patcher

A Rust-based ROM patcher for the NES game Faxanadu. Applies a collection of bug fixes, quality-of-life improvements, and new features to the Faxandu ROM (EN version).

An effort has been made to ensure each patch is relatively readable by any programmer who is unfamilar with Rust but knows some 6502. Comments have been added extensively.

## Prerequisites

This tool is written in Rust. If you don't have Rust installed, the easiest way is via [rustup](https://rustup.rs/):

- **macOS / Linux** — open a terminal and run:
  ```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Then restart your terminal (or run `source ~/.cargo/env`).

- **Windows** — download and run the installer from [rustup.rs](https://rustup.rs/). You may also need the [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) if prompted.

You can verify the install worked by running `cargo --version`.

## Usage

1. **Build the patcher:**
   ```
   cargo build --release
   ```
   This compiles the tool into `target/release/faxanadu_patcher` (or `.exe` on Windows).

2. **Patch a ROM:**
   ```
   target/release/faxanadu_patcher roms/Faxanadu.nes roms/FaxanaduPatched.nes
   ```
   The first argument is your original ROM, the second is where the patched ROM will be written.

3. **Optional — generate an IPS patch file:**
   ```
   target/release/faxanadu_patcher roms/Faxanadu.nes roms/FaxanaduPatched.nes --ips
   ```

In this early version, patches are toggled by commenting/uncommenting calls in `main.rs`.

Patches that require new code will dynamically allocate to avoid overwriting each other, and will check for free space, accounting for differences that may be present from using [fax-edit](https://github.com/kaimitai/faxedit).

## Patches

### Bug Fixes
- **Hourglass fix** — no longer costs 50% of player health
- **Shield-ointment fix** — magic passes through shields when ointment is active
- **Ointment vs Sugata** — ointment now protects against Sugata's screenwide flash
- **Fire spell animation** — fixes a wrong frame definition
- **Studded Mail climb tile** — fixes a missing tile when climbing without a shield

### Indoor Improvements
- **Equip items indoors** — removes the restriction on equipping items in buildings
- **Use items indoors** — weapons stay wielded when entering buildings
- **Draw weapon indoors** — weapons are shown immediately upon equipping indoors

### New Items (requires changes with [faxiscripts](https://github.com/kaimitai/faxiscripts) for item-related dialogs)
- **Mana Potion** — the unused Black Potion now restores MP
- **Crystal (warp)** — the unused Crystal item warps the player to their last Guru spawn point
- **Crystal (overworld pickup)** — place Crystals in the game world via entity 0x4C
- **Fire Crystal (screen damage)** — deals 50 damage to all on-screen enemies with a screen flash
- **Fire Crystal (overworld pickup)** — place Fire Crystals in the game world via entity 0x4B
- **Poison to Black Potion** — overworld poison pickups become Black Potions instead

### Player & Combat
- **No knockback on ladders** — player holds onto ladders when hit
- **Allow lower respawn** — talking to an earlier Guru updates your spawn point
- **Killswitch** — pause, press Select, unpause to instantly die (for testing/softlocks)
- **Pendant quest** — pendant is cursed (-25% damage) until a quest flag is set (+25% damage), with a 50% variant available
- **Menu on first screen** — Select button works on the first screen of Eolis

### Shops
- **Sell anything** — all items can be sold to any vendor (unsellable items go for 100 golds)

### Sprites
- **Clone sprite 0x1D → 0x25** — creates a second Skeleton Knight entity with independent behavior

### Text
- **Faster text** — text renders twice as fast with corrected sound cadence

### Music
- **Pause music** — plays music during the pause screen

## Project Structure

```
src/
├── main.rs              # Patch orchestration
├── rom.rs               # ROM read/write helpers
├── allocator.rs         # Free space allocator with guard bytes
├── consts.rs            # Constants and helper functions
├── patches/
│   ├── bugfixes.rs      # Vanilla bug fixes
│   ├── indoor.rs        # Indoor restriction removals
│   ├── items.rs         # New/repurposed items
│   ├── player.rs        # Player mechanics & combat
│   ├── shops.rs         # Shop behavior changes
│   ├── sprites.rs       # Entity sprite cloning
│   ├── text.rs          # Text speed patches
│   └── music.rs         # Music-related patches
└── ...
```

## Thanks

Tons of thanks to those who have contributed to hacking this game — [Notlobb](https://github.com/Notlobb/Randumizer), [ChipX86](https://github.com/chipx86/faxanadu), [Sebastian Porst](http://www.the-interweb.com/serendipity/index.php?/archives/7-Faxanadu-level-data-Part-I.html), [Kaimitai](https://github.com/kaimitai/faxedit), Ascended_Mermaid, Ok Impala!, and likely others I'm forgetting! (Please let me know if I'm missing anyone!)

## Discord

Check out the **Faxanadu Randomizer & Romhacking** Discord server if you have questions or further contributions!

[![Discord](https://img.shields.io/badge/Faxanadu%20Randomizer%20%26%20Romhacking-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/K65uxXhA)
