// src/gui/mod.rs
pub mod gui_server;  // Asegúrate de que gui_server esté correctamente exportado

pub use gui_server::start_gui_server;  // Esto hace que start_gui_server sea accesible fuera del módulo
