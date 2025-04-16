use ratatui::style::Color;

/// Provides access to the Gruvbox color palette.
#[derive(Debug, Clone, Copy)]
pub struct Gruvbox;

impl Gruvbox {
    // Dark colors
    pub const fn dark0_hard() -> Color { Color::from_u32(0x1d2021) }
    pub const fn dark0() -> Color { Color::from_u32(0x282828) }
    pub const fn dark0_soft() -> Color { Color::from_u32(0x32302f) }
    pub const fn dark1() -> Color { Color::from_u32(0x3c3836) }
    pub const fn dark2() -> Color { Color::from_u32(0x504945) }
    pub const fn dark3() -> Color { Color::from_u32(0x665c54) }
    pub const fn dark4() -> Color { Color::from_u32(0x7c6f64) }

    // Gray colors
    pub const fn gray245() -> Color { Color::from_u32(0x928374) }
    pub const fn gray244() -> Color { Color::from_u32(0x928374) }

    // Light colors
    pub const fn light0_hard() -> Color { Color::from_u32(0xf9f5d7) }
    pub const fn light0()      -> Color { Color::from_u32(0xfbf1c7) }
    pub const fn light0_soft() -> Color { Color::from_u32(0xf2e5bc) }
    pub const fn light1()      -> Color { Color::from_u32(0xebdbb2) }
    pub const fn light2()      -> Color { Color::from_u32(0xd5c4a1) }
    pub const fn light3()      -> Color { Color::from_u32(0xbdae93) }
    pub const fn light4()      -> Color { Color::from_u32(0xa89984) }

    // Bright colors
    pub const fn red_bright()    -> Color { Color::from_u32(0xfb4934) }
    pub const fn green_bright()  -> Color { Color::from_u32(0xb8bb26) }
    pub const fn yellow_bright() -> Color { Color::from_u32(0xfabd2f) }
    pub const fn blue_bright()   -> Color { Color::from_u32(0x83a598) }
    pub const fn purple_bright() -> Color { Color::from_u32(0xd3869b) }
    pub const fn aqua_bright()   -> Color { Color::from_u32(0x8ec07c) }
    pub const fn orange_bright() -> Color { Color::from_u32(0xfe8019) }

    // Normal colors
    pub const fn red() -> Color { Color::from_u32(0xcc241d) }
    pub const fn green() -> Color { Color::from_u32(0x98971a) }
    pub const fn yellow() -> Color { Color::from_u32(0xd79921) }
    pub const fn blue() -> Color { Color::from_u32(0x458588) }
    pub const fn purple() -> Color { Color::from_u32(0xb16286) }
    pub const fn aqua() -> Color { Color::from_u32(0x689d6a) }
    pub const fn orange() -> Color { Color::from_u32(0xd65d0e) }

    // Dim colors
    pub const fn red_dim() -> Color { Color::from_u32(0x9d0006) }
    pub const fn green_dim() -> Color { Color::from_u32(0x79740e) }
    pub const fn yellow_dim() -> Color { Color::from_u32(0xb57614) }
    pub const fn blue_dim() -> Color { Color::from_u32(0x076678) }
    pub const fn purple_dim() -> Color { Color::from_u32(0x8f3f71) }
    pub const fn aqua_dim() -> Color { Color::from_u32(0x427b58) }
    pub const fn orange_dim() -> Color { Color::from_u32(0xaf3a03) }
}