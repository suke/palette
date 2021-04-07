use crate::rgb::Rgb;
use std::collections::HashSet;

#[derive(Debug)]
pub struct PaletteOptions {
    pub(crate) colors: HashSet<Rgb>,
}

impl PaletteOptions {
    pub fn default() -> Self {
        PaletteOptions {
            colors: HashSet::new(),
        }
    }

    pub fn ignore_colors(mut self, colors: Vec<Rgb>) -> Self {
        for color in colors {
            self.colors.insert(color);
        }
        self
    }
}
