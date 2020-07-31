mod emulator;
use emulator::Gameboy;
use glium::{
    glutin::{
        event_loop::{
            EventLoop,
            ControlFlow,
        },
        event::{*},
        window::{
            WindowBuilder
        },
        ContextBuilder,
        dpi::LogicalSize,
    },
    Surface,
    Display,
    index::{
        IndexBuffer,
        PrimitiveType,
    },
};
use std::time::{Instant, Duration};
#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

struct Show {
    vtx: Vec<Vertex>,
    indices: [u16; 6]
}

fn create_program(display: &Display) -> glium::Program {
    let vertex_shader_src = include_str!("shaders/vertex.glsl");
    let fragment_shader_src = include_str!("shaders/frag.glsl");
    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

fn create_show() -> Show {
    implement_vertex!(Vertex, position, tex_coord);
    
    let vertex1 = Vertex { position: [-1.0, -1.0], tex_coord: [0.0, 1.0] }; 
    let vertex2 = Vertex { position: [ 1.0,  1.0], tex_coord: [1.0, 0.0] }; 
    let vertex3 = Vertex { position: [ 1.0, -1.0], tex_coord: [1.0, 1.0] }; 
    let vertex4 = Vertex { position: [ -1.0, 1.0], tex_coord: [0.0, 0.0] }; 

    let quad: [u16; 6] = [
        1, 2, 3,
        2, 0, 3
    ];

    Show {vtx: vec![vertex1, vertex2, vertex3, vertex4], indices: quad }
}


fn main(){
    let mut system = Gameboy::default();

    //panics if a char is not valid unicode
    let args: Vec<_> = std::env::args().collect();
    system.insert(args[1].to_string());
    let debug = args.contains(&"-d".to_string());

    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(640,576));
    let cb = ContextBuilder::new();
    
    let display = match Display::new(wb,cb, &event_loop) {
        Ok(dp) => dp,
        Err(err) => panic!("Failed to create display:\n{}", err), 
    };

    let show = create_show();

    let program = create_program(&display);
    let vertex_buffer = glium::VertexBuffer::new(&display, &show.vtx).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &show.indices).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent{event, ..} => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return ();
                }
                _ => {}
            }
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return (),
            }
            _ => {}
        }

        //render loop
        system.start(debug);

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        let image = glium::texture::RawImage2d{
            data: std::borrow::Cow::Borrowed(&system.screen[0..160*144]),
            height: 144,
            width: 160,
            format: glium::texture::ClientFormat::U8U8U8U8
        };
        
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

        let mut target = display.draw();

        let uni = uniform!{ tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest) };

        target.draw(&vertex_buffer, &indices, &program, &uni,
            &Default::default()).unwrap();

        target.finish().unwrap();
        
    });



}
