#### [Basic implementation of CPU, RAM and Cartrigbe]  
Commit: 2f0aa415f5a61e4cdddae40058cc3b265619a97c

First commit, next one will push this changelog

Current directory structure:
```'
├── src
│   ├── cartrigbe
│   │   ├── banks.rs                    => Defines a bank
│   │   ├── header.rs                   => Defines a cartrigbe header
│   │   └── sram.rs                     => Will be used to define RAM banks in cartrigbe
│   ├── cpu
│   │   ├── decoder.rs                  => Open address table used to translate opcode in a 
|   |   |                               Instruction object
│   │   └── registers.rs                => Defines Registers object and its operations
│   ├── io 
│   ├── memory
│   │   └── mmu.rs                      => Defines an MMU that deals directly with the address space
|   |
│   ├── cartrigbe.rs                    => Defines a Cartrigbe and its functions
│   ├── cpu.rs                          => Defines a Core structure that corresponds to CPU and MMU
|   |                                   together since they work very closely
|   |
│   ├── memory.rs                       => Defines a MMU operations that need to work with registers
|   |
│   └── main.rs                         => Defines a Gameboy object
|
├── Cargo.lock
├── Cargo.toml                          => Current
├── changelog
└── DMG_ROM.bin                         => Bootstrap Gameboy ROM
````
 
