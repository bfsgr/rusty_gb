use minifb::{Key, Window, WindowOptions};
use rustygb::Gameboy;
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const MAXCYCLES: u32 = 66576;

fn create_window() -> Window {
    let win = Window::new(
        "Rusty GB",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            resize: false,
            scale: minifb::Scale::X4,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            transparency: false,
            none: false,
            title: true,
            topmost: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    return win;
}

fn get_input(window: &Window, system: &mut Gameboy) {
    macro_rules! get_input {
        ($( $key:expr, $syskey:ident ),*) => {
            $(
                if window.is_key_down($key) {
                    system.$syskey(true);
                } else {
                    system.$syskey(false);
                }
            )*
        }
    }
    get_input!(
        Key::Down,
        down,
        Key::Up,
        up,
        Key::Left,
        left,
        Key::Right,
        right,
        Key::Z,
        btn_a,
        Key::X,
        btn_b,
        Key::F,
        start,
        Key::G,
        select
    );
}

fn main() {
    let mut system = Gameboy::default();

    //panics if a char is not valid unicode
    let args: Vec<_> = std::env::args().collect();

    system.insert(args[1].to_string());

    let debug = args.contains(&"-d".to_string());

    let mut window = create_window();

    let frame = Duration::new(0, 16600000); // 16.6 ms as nanoseconds

    system.screen = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = Instant::now();
        let mut cycles_now = 0;
        while cycles_now < MAXCYCLES {
            cycles_now += system.run(debug) as u32;
            get_input(&window, &mut system);
        }

        let elapsed = start.elapsed();
        if elapsed < frame {
            thread::sleep(frame - elapsed);
        }

        window
            .update_with_buffer(&system.screen, WIDTH, HEIGHT)
            .unwrap();
    }
}
