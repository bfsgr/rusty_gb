mod emulator;
use emulator::Gameboy;


fn main(){
    let mut system = Gameboy::default();

    system.insert("Tetris2.gb".to_string());
    
    system.start();
}