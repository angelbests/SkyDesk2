use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

#[tauri::command]
pub fn keyinput(){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // Paste
    enigo.key(Key::Control, Press);
    enigo.key(Key::Unicode('v'), Click);
    enigo.key(Key::Control, Release);
    // Do things with the mouse
    enigo.move_mouse(500, 200, Coordinate::Abs);
    enigo.button(Button::Left, Press);
    enigo.move_mouse(100, 100, Coordinate::Rel);
    enigo.button(Button::Left, Release);
    // Enter text
    enigo.text("hello world");
}
