use crate::CmdResult;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub enum BinaryMode {
    Signed,
    Unsigned,
}

#[derive(Clone, Copy)]
pub enum BinaryWidth {
    W8,
    W16,
    W32,
    W64,
}

static BINARY_MODE: Mutex<BinaryMode> = Mutex::new(BinaryMode::Unsigned);
static BINARY_WIDTH: Mutex<BinaryWidth> = Mutex::new(BinaryWidth::W32);

fn set_binary_mode_signed() -> CmdResult {
    let mut mode = BINARY_MODE.lock().unwrap();
    *mode = BinaryMode::Signed;
    CmdResult::Success
}

fn set_binary_mode_unsigned() -> CmdResult {
    let mut mode = BINARY_MODE.lock().unwrap();
    *mode = BinaryMode::Unsigned;
    CmdResult::Success
}

fn set_binary_width(width: BinaryWidth) -> CmdResult {
    let mut w = BINARY_WIDTH.lock().unwrap();
    *w = width;
    CmdResult::Success
}

fn set_binary_width_8() -> CmdResult {
    set_binary_width(BinaryWidth::W8)
}

fn set_binary_width_16() -> CmdResult {
    set_binary_width(BinaryWidth::W16)
}

fn set_binary_width_32() -> CmdResult {
    set_binary_width(BinaryWidth::W32)
}

fn set_binary_width_64() -> CmdResult {
    set_binary_width(BinaryWidth::W64)
}

pub fn get_binary_mode() -> BinaryMode {
    let mode = BINARY_MODE.lock().unwrap();
    *mode
}

pub fn get_binary_width() -> BinaryWidth {
    let width = BINARY_WIDTH.lock().unwrap();
    *width
}

pub fn get_modes_string() -> String {
    let mode = BINARY_MODE.lock().unwrap();
    let width = BINARY_WIDTH.lock().unwrap();
    let mut modes = String::new();

    modes += match *width {
        BinaryWidth::W8 => "8",
        BinaryWidth::W16 => "16",
        BinaryWidth::W32 => "32",
        BinaryWidth::W64 => "64",
    };

    modes += "bit ";

    modes += match *mode {
        BinaryMode::Signed => "Signed",
        BinaryMode::Unsigned => "Unsigned",
    };

    modes
}

pub fn commands(cmd: &str) -> CmdResult {
    match cmd {
        "signed" => set_binary_mode_signed(),
        "unsigned" => set_binary_mode_unsigned(),
        "width8" => set_binary_width_8(),
        "width16" => set_binary_width_16(),
        "width32" => set_binary_width_32(),
        "width64" => set_binary_width_64(),
        _ => CmdResult::NoMatch,
    }
}
