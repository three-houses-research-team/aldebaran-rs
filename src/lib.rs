#![feature(proc_macro_hygiene)]
#![feature(asm)]

use skyline::{hook, install_hook};
mod forge;

// Required to prevent Skyline from crashing the game
#[hook(offset = 0xAFCCF0)]
fn socket_stub() -> u32 {
    0
}

// Game::Menu::TitleScreen::GetVersionString()
#[hook(offset = 0x3E63E0)]
fn get_version_string() -> *const u64 {
    String::from(format!("Aldebaran v{}\0", env!("CARGO_PKG_VERSION"))).as_ptr() as *const u64
}

#[skyline::main(name = "aldebaran")]
pub unsafe fn main() {
    println!("Hello from Aldebaran-rs!");

    // Kill the original nn::os::SocketInitialize method
    install_hook!(socket_stub);

    // Replace the version string on the title screen
    install_hook!(get_version_string);

    // Intercept EntryId loadings to replace files on the fly
    forge::init_forge();
}
