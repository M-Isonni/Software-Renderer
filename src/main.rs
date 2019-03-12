use sdl2;
use gl;
mod shaders_func;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe{
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'game,
                _ => {},
            }

        }

        unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    
    window.gl_swap_window();

    }
}
// use winit;
// use std::io;
// use std::fs;
// use winit::os::windows;

//abporting the use of winit, can't find documentation on winit::os::windows
// fn main(){
//     let mut events_loop = winit::EventsLoop::new();
//     let mut builder = winit::WindowBuilder::new();
//     let size = winit::dpi::LogicalSize{
//         width: 800.0,
//         height: 600.0,
//     };
//     builder.window.dimensions= Some(size);
//     let _win = match builder.build(&events_loop){        
//         Ok(window)=>window,
//         Err(creation_error)=>panic!("unable to crate window. Error: {:?}",creation_error),    
//     };  
    
//     let stormtrooper = fs::read_to_string("Stormtrooper.obj").expect("unable to find the file");

//     impl windows::WindowExt for _win{};
    
//     //println!("file: {}",stormtrooper);
    
//     let mut running = true;  
//     while running {
//         events_loop.poll_events(|event| {
//         match event {
//             winit::Event::WindowEvent {
//                 event: winit::WindowEvent::Resized(winit::dpi::LogicalSize { width, height }),
//                 ..
//             } => {
//                 println!("The window was resized to {}x{}", width, height);
//                 },
//             winit::Event::WindowEvent {
//                     event: winit::WindowEvent::CloseRequested,
//                 ..
//                 }=>running = false,                
//                 _ => ()
//             }
//         });
//     }    
// }
