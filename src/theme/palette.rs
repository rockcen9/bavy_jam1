use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct ColorPalette {
    // Fluffy8 palette — all 8 colors
    pub deep_purple: Color,  // #44387a
    pub crimson_rose: Color, // #933a63
    pub iris: Color,         // #7679db
    pub periwinkle: Color,   // #96b3fd
    pub lavender: Color,     // #d4d5f6
    pub rose_petal: Color,   // #ffc4cf
    pub blush: Color,        // #ff95b1
    pub petal: Color,        // #ffecf0

    // Semantic UI aliases
    pub screen_background: Color,
    pub header_text: Color,
    pub label_text: Color,
    pub button_background: Color,
    pub button_hovered_background: Color,
    pub button_pressed_background: Color,
    pub button_text: Color,

    // Semantic game aliases
    pub terminal_text: Color,      // bright-green terminal text for end/dead sequences
    pub panel_bg: Color,           // dark horror panel background
    pub panel_image_bg: Color,     // panel image section background
    pub panel_divider: Color,      // panel section divider
    pub panel_content_text: Color, // panel body text
    pub panel_dim_text: Color,     // dimmed/subdued panel text
    pub backpack_slot: Color,      // backpack inventory slot background
}

impl Default for ColorPalette {
    fn default() -> Self {
        let deep_purple  = Color::srgb_u8(0x44, 0x38, 0x7a);
        let crimson_rose = Color::srgb_u8(0x93, 0x3a, 0x63);
        let iris         = Color::srgb_u8(0x76, 0x79, 0xdb);
        let periwinkle   = Color::srgb_u8(0x96, 0xb3, 0xfd);
        let lavender     = Color::srgb_u8(0xd4, 0xd5, 0xf6);
        let rose_petal   = Color::srgb_u8(0xff, 0xc4, 0xcf);
        let blush        = Color::srgb_u8(0xff, 0x95, 0xb1);
        let petal        = Color::srgb_u8(0xff, 0xec, 0xf0);

        Self {
            deep_purple,
            crimson_rose,
            iris,
            periwinkle,
            lavender,
            rose_petal,
            blush,
            petal,

            screen_background:          deep_purple,
            header_text:                petal,
            label_text:                 rose_petal,
            button_background:          crimson_rose,
            button_hovered_background:  iris,
            button_pressed_background:  deep_purple,
            button_text:                petal,

            terminal_text:              Color::srgba(0.0, 0.9, 0.4, 1.0),
            panel_bg:                   deep_purple,
            panel_image_bg:             crimson_rose,
            panel_divider:              iris,
            panel_content_text:         rose_petal,
            panel_dim_text:             lavender,
            backpack_slot:              iris,
        }
    }
}

impl ColorPalette {
    pub fn get(&self, name: UiColorName) -> Color {
        match name {
            UiColorName::ScreenBackground        => self.screen_background,
            UiColorName::HeaderText              => self.header_text,
            UiColorName::LabelText               => self.label_text,
            UiColorName::ButtonBackground        => self.button_background,
            UiColorName::ButtonHoveredBackground => self.button_hovered_background,
            UiColorName::ButtonPressedBackground => self.button_pressed_background,
            UiColorName::ButtonText              => self.button_text,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UiColorName {
    ScreenBackground,
    HeaderText,
    LabelText,
    ButtonBackground,
    ButtonHoveredBackground,
    ButtonPressedBackground,
    ButtonText,
}
