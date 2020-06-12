mod emulator;
use emulator::Gameboy;


fn main(){
    let mut system = Gameboy::default();

    system.insert("cpu_instrs.gb".to_string());
    
    system.start();
}