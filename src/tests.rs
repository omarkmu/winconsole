use super::console::*;
use cgmath::Vector2;

#[test]
fn beep() {
    Console::beep(440, 1000).unwrap();
}

#[test]
fn cursor_position() {
    let position = Console::get_cursor_position().unwrap();
    Console::set_cursor_position(10, 10).unwrap();
    assert_eq!(Console::get_cursor_position().unwrap(), Vector2::new(10, 10));
    Console::set_cursor_position(position.x, position.y).unwrap();
}
#[test]
fn cursor_size() {
    let old_size = Console::get_cursor_size().unwrap();
    Console::set_cursor_size(35).unwrap();
    assert_eq!(Console::get_cursor_size().unwrap(), 35);
    Console::set_cursor_size(old_size).unwrap();
}
#[test] #[should_panic]
fn cursor_size_fail() {
    Console::set_cursor_size(101).unwrap();
}
#[test]
fn cursor_visible() {
    let visible = Console::is_cursor_visible().unwrap();
    Console::set_cursor_visible(false).unwrap();
    assert_eq!(Console::is_cursor_visible().unwrap(), false);

    Console::set_cursor_visible(visible).unwrap();
}

#[test]
fn background_color() {
    let old_color = Console::get_background_color().unwrap();
    Console::set_background_color(ConsoleColor::DarkRed).unwrap();
    println!("A message with a dark red background.");

    let background_color = Console::get_background_color().unwrap();
    assert_eq!(background_color, ConsoleColor::DarkRed);
    Console::set_background_color(old_color).unwrap();
}
#[test]
fn foreground_color() {
    let old_color = Console::get_foreground_color().unwrap();
    Console::set_foreground_color(ConsoleColor::DarkBlue).unwrap();
    println!("A dark blue message.");

    let foreground_color = Console::get_foreground_color().unwrap();
    assert_eq!(foreground_color, ConsoleColor::DarkBlue);
    Console::set_foreground_color(old_color).unwrap();
}

#[test]
fn input_mode() {
    let input_mode_orig = Console::get_input_mode().unwrap();
    let mut input_mode = input_mode_orig.clone();

    input_mode.WindowInput = !input_mode.WindowInput;
    Console::set_input_mode(input_mode).unwrap();
    assert_eq!(Console::get_input_mode().unwrap(), input_mode);

    Console::set_input_mode(input_mode_orig).unwrap();
}
#[test] #[should_panic]
fn input_mode_fail() {
	let mut input_mode = Console::get_input_mode().unwrap();
	input_mode.LineInput = false;
	input_mode.EchoInput = true;
    Console::set_input_mode(input_mode).unwrap();
}

#[test]
fn title() {
    let original_title = Console::get_original_title().unwrap();

    Console::set_title("Some New Console Title").unwrap();
    assert_eq!(Console::get_title().unwrap(), "Some New Console Title");
    
    Console::set_title(&original_title).unwrap();
    assert_eq!(Console::get_title().unwrap(), original_title)
}
#[test]
fn title_empty() {
    let original_title = Console::get_original_title().unwrap();

    Console::set_title("").unwrap();
    assert_eq!(Console::get_title().unwrap(), "");

    Console::set_title(&original_title).unwrap();
    assert_eq!(Console::get_title().unwrap(), original_title)
}
