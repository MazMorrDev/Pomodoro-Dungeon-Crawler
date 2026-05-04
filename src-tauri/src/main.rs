// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
const MY_STRING2: &str = "Esto es una cadena de texto constante";

fn main() {
    println!("{MY_STRING2}");
    app_lib::run();
}
