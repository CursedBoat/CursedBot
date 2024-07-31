use std::str::FromStr;

#[derive(Debug)]
pub struct Color {
    pub color_vec: Vec<u8>
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure the hex string is valid (6 characters long)
        if s.len() != 6 {
            return Err(format!("Invalid hex color length: {}", s));
        }

        // Parse the hex string into a number
        let hex_value = u32::from_str_radix(s, 16).map_err(|e| format!("Invalid hex color: {}", e))?;

        // Extract RGB components
        let r = ((hex_value >> 16) & 0xFF) as u8;
        let g = ((hex_value >> 8) & 0xFF) as u8;
        let b = (hex_value & 0xFF) as u8;

        Ok(Color { color_vec: vec![r, g, b]})
    }
}