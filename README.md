# Faxanadu Patcher

Right now this is a basic collection of scripts in Rust for patching the NES game Faxanadu.

Feel free to make use of it however you wish!

## To Run

```
cargo build --release
```

Then take your Faxanadu ROM and point the compiled program at it:

```
target/release/faxanadu_patcher roms/Faxanadu.nes roms/FaxanduPatched.nes
```

You can add `--ips` at the end to also generate an IPS patch file.

Right now it's a whole bunch of patches that are turned on and off by commenting out patch commands in main.rs and elsewhere.

## Plans

Right now mostly just trying to make a collection of scripts that allow for the building of a speedrun practice ROM, and for testing new features forthe Faxanadu randomizer.

That said here's a list of aspirations:

- modular architecture that allows for quick scripting of new patches
- modular command line that allows for patches to be toggled on and off
- ability to ingest config files to rearrage and substitute items, dialog, and more

## Thanks

Tons of thanks to those who have contributed to hacking this game - [Notlobb](https://github.com/Notlobb/Randumizer), [ChipX86](https://github.com/chipx86/faxanadu), [Sebastian Porst](http://www.the-interweb.com/serendipity/index.php?/archives/7-Faxanadu-level-data-Part-I.html) and more!