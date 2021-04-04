use crate::color_cluster::ColorCluster;
use crate::rgb::Rgb;

#[derive(Debug)]
pub struct Colors {
    data: Vec<ColorCluster>,
}

impl Colors {
    pub fn new() -> Self {
        Colors { data: vec![] }
    }

    pub fn add(&mut self, color_cluster: ColorCluster) {
        self.data.push(color_cluster);
    }

    pub fn get_colors(&self) -> Vec<Rgb> {
        self.data.iter().map(|colors| colors.centroid).collect()
    }

    pub fn sort(&mut self) {
        self.data.sort_by(|x, y| y.len().cmp(&x.len()));
    }
}
