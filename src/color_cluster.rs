use crate::rgb::Rgb;
use smartcore::math::distance::{Distance, Distances};

#[derive(Debug, Clone)]
pub struct ColorCluster {
    pub centroid: Rgb,
    data: Vec<Rgb>,
}

impl ColorCluster {
    pub fn new(centroid: Rgb) -> Self {
        ColorCluster {
            centroid,
            data: vec![],
        }
    }

    pub fn distance(&self, rgb: &Rgb) -> f64 {
        Distances::euclidian().distance(&self.centroid.to_vec(), &rgb.to_vec())
    }

    pub fn add(&mut self, rgb: Rgb) {
        self.data.push(rgb);
    }

    pub fn recalculate_centroid(&mut self) {
        let len = self.data.len();
        let rgb_sum = self.data.iter().fold(vec![0.0, 0.0, 0.0], |mut ret, rgb| {
            ret[0] = ret[0] + rgb.0 as f32;
            ret[1] = ret[1] + rgb.1 as f32;
            ret[2] = ret[2] + rgb.2 as f32;
            ret
        });
        let r_mean = (rgb_sum[0] / len as f32).round() as u8;
        let g_mean = (rgb_sum[1] / len as f32).round() as u8;
        let b_mean = (rgb_sum[2] / len as f32).round() as u8;
        self.centroid = Rgb(r_mean, g_mean, b_mean)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
