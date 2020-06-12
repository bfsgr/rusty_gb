mod emulator;
use emulator::Gameboy;


fn main(){
    let mut system = Gameboy::default();

    let args: Vec<_> = std::env::args().collect();

    system.insert(args[1].to_string());
    
    system.start();
}