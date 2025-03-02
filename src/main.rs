use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use minifb::{Window, WindowOptions, Scale, Key, MouseMode};

struct ViewState {
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    drag_start: Option<(f32, f32)>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename.cif>", args[0]);
        return;
    }
    
    let (orig_width, orig_height, original_buffer) = load_cif_file(&args[1]);
    
    let mut window = Window::new(
        "Cifir",
        orig_width as usize,
        orig_height as usize,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    ).unwrap();

    let mut view_state = ViewState {
        scale: 1.0,
        offset_x: 0.0,
        offset_y: 0.0,
        drag_start: None,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&mut window, &mut view_state);
        
        let (width, height) = window.get_size();
        let scaled_buffer = scale_buffer(
            &original_buffer,
            orig_width as usize,
            orig_height as usize,
            &view_state,
            width,
            height,
        );
        
        window.update_with_buffer(&scaled_buffer, width, height).unwrap();
    }
}

fn handle_input(window: &mut Window, view: &mut ViewState) {
    // Mouse scrolling
    let (_, scroll_y) = window.get_scroll_wheel().unwrap_or((0.0, 0.0));
    if scroll_y != 0.0 {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            let old_scale = view.scale;
            let scale_factor = if scroll_y > 0.0 { 1.1 } else { 0.9 };
            view.scale = (view.scale * scale_factor).clamp(0.1, 20.0);
            
            // Adjust offset to zoom at mouse position
            let orig_x = (mouse_x - view.offset_x) / old_scale;
            let orig_y = (mouse_y - view.offset_y) / old_scale;
            view.offset_x = mouse_x - orig_x * view.scale;
            view.offset_y = mouse_y - orig_y * view.scale;
        }
    }

    // Mouse dragging
    if window.get_mouse_down(minifb::MouseButton::Left) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
            match view.drag_start {
                Some((start_x, start_y)) => {
                    view.offset_x += x - start_x;
                    view.offset_y += y - start_y;
                }
                None => view.drag_start = Some((x, y)),
            }
        }
    } else {
        view.drag_start = None;
    }
}

fn scale_buffer(
    original: &[u32],
    orig_width: usize,
    orig_height: usize,
    view: &ViewState,
    win_width: usize,
    win_height: usize,
) -> Vec<u32> {
    let mut buffer = vec![0; win_width * win_height];
    
    for y in 0..win_height {
        for x in 0..win_width {
            let src_x = ((x as f32 - view.offset_x) / view.scale).floor() as i32;
            let src_y = ((y as f32 - view.offset_y) / view.scale).floor() as i32;
            
            if src_x >= 0 && src_x < orig_width as i32 && src_y >= 0 && src_y < orig_height as i32 {
                buffer[y * win_width + x] = original[src_y as usize * orig_width + src_x as usize];
            }
        }
    }
    
    buffer
}

fn load_cif_file(filename: &str) -> (u32, u32, Vec<u32>) {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let comment_re = Regex::new(r"<--.*?-->").unwrap();
    let mut lines = reader.lines().map(|l| l.unwrap());

    // Parse resolution
    let resolution_line = lines.next().expect("No resolution line");
    let (width, height) = parse_resolution(&resolution_line).expect("Invalid resolution");

    // Parse colors
    let mut pixels = Vec::new();
    for line in lines {
        let cleaned_line = comment_re.replace_all(&line, "").replace(" ", "");
        let colors: Vec<&str> = cleaned_line.split('#').skip(1).collect();
        
        for color_str in colors {
            let (r, g, b) = parse_hex_color(color_str).unwrap_or_else(|| {
                eprintln!("Invalid color: #{}", color_str);
                std::process::exit(1);
            });
            pixels.push((r, g, b));
        }
    }

    let buffer: Vec<u32> = pixels
        .iter()
        .map(|&(r, g, b)| (r as u32) << 16 | (g as u32) << 8 | b as u32)
        .collect();

    (width, height, buffer)
}

fn parse_resolution(s: &str) -> Option<(u32, u32)> {
    let re = Regex::new(r"<(\d+)x(\d+)>").unwrap();
    let caps = re.captures(s)?;
    Some((caps[1].parse().ok()?, caps[2].parse().ok()?))
}

fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    let s = s.trim();
    if s.len() != 6 { return None; }
    Some((
        u8::from_str_radix(&s[0..2], 16).ok()?,
        u8::from_str_radix(&s[2..4], 16).ok()?,
        u8::from_str_radix(&s[4..6], 16).ok()?,
    ))
}