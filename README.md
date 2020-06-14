# Rusty GB - Gameboy Emulator

## CPU
- [X] 256 basic instruction set
- [X] 256 sub instruction set (0xCB prefix)
- [X] Pass all [blargg-gb](https://gbdev.gg8.se/files/roms/blargg-gb-tests/) cpu_instrs tests 
- [ ] Pass [blargg-gb](https://gbdev.gg8.se/files/roms/blargg-gb-tests/) instrs timming tests
- [ ] Review hardcoded cycles and interrupts cycles

## Bus
- [X] Address Space
- [X] Redirect R/W to correct module
- [ ] Review OAM_DMA

## GPU
- [X] Basic background and window drawing (Sprites and BG priority not working properly yet)
- [X] Interrupt triggers
- [ ] BG priority
- [ ] Sprites

## Timer
- [X] Implemented completely
- [X] Interrupt triggers

## Cartrigbe
- [X] Basic MCB1 (Only without SRAM)
- [ ] SRAM support

## Sound Controller
- [ ] Basic implementation
- [X] Interrupt triggers

## Joypad
- [ ] Basic implementation
- [X] Interrupt triggers

