use gscriptrust::lexer::*;
use gscriptrust::error::*;
use gscriptrust::parser::*;
use gscriptrust::visitor::*;
use std::cell::RefCell;
use std::rc::Rc;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    //_temp_sdl3_test(); 


    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        GError::command_line(1, "Expected 2 command line arguments: \n'gscript [filename]'");
        std::process::exit(1);
    }
    if args[1] != "gscript" {
        GError::command_line(2, "No support for commands other than 'gscript' has been implemented");
        std::process::exit(1);
    }
    let filename = args[2].clone();
    let errorstack = Rc::new(RefCell::new(ErrorStack::new()));
    let mut lexer = Lexer::new(format!("entry/{}", filename).as_str(), Rc::clone(&errorstack));
    lexer.lex();

    let mut parser = Parser::new(&lexer.tokens, Rc::clone(&errorstack));
    let ast_compound = parser.parse_compound().unwrap();

    let mut visitor = Visitor::new(Rc::clone(&errorstack));

    //visit GScript standard libraries (string, ..)
    visitor.preload = true;
    //string library
    let string_gsc_root = gscriptrust::ast::ASTNode::new(gscriptrust::ast::AST::IMPORT{filename: "string.gsc".to_string(), object_name: String::new()}, ErrorInfo::new_empty()   );
    visitor.visit_import(&string_gsc_root);

    //end visit GScript standard libraries
    visitor.preload = false;
    
    visitor.visit(&ast_compound);

    let errorstack = visitor.errorstack;
    errorstack.borrow().print_dump();
}

fn _temp_sdl3_test() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ts pmo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    canvas.set_draw_color(sdl3::pixels::Color::RGB(0, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i =0;
    'running: loop {
        canvas.clear();
        canvas.present();
        i = i + 1;
        if i >= 255 {
            i = 0;
        }
        canvas.set_draw_color(sdl3::pixels::Color::RGB(255-i, i, i));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}