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

pub fn get_binary_mode() -> BinaryMode {
    let mode = BINARY_MODE.lock().unwrap();
    *mode
}

pub fn set_binary_mode_signed() -> Result<(), &'static str> {
    let mut mode = BINARY_MODE.lock().unwrap();
    *mode = BinaryMode::Signed;
    Ok(())
}

pub fn set_binary_mode_unsigned() -> Result<(), &'static str> {
    let mut mode = BINARY_MODE.lock().unwrap();
    *mode = BinaryMode::Unsigned;
    Ok(())
}

pub fn get_binary_width() -> BinaryWidth {
    let width = BINARY_WIDTH.lock().unwrap();
    *width
}

pub fn set_binary_width(width: BinaryWidth) -> Result<(), &'static str> {
    let mut w = BINARY_WIDTH.lock().unwrap();
    *w = width;
    Ok(())
}

pub fn set_binary_width_8() -> Result<(), &'static str> {
    set_binary_width(BinaryWidth::W8)
}

pub fn set_binary_width_16() -> Result<(), &'static str> {
    set_binary_width(BinaryWidth::W16)
}

pub fn set_binary_width_32() -> Result<(), &'static str> {
    set_binary_width(BinaryWidth::W32)
}

pub fn set_binary_width_64() -> Result<(), &'static str> {
    set_binary_width(BinaryWidth::W64)
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
