use crate::color_cluster::ColorCluster;
use crate::colors::Colors;
use crate::image::{fetch_image, resize};
use crate::palette_options::PaletteOptions;
use crate::rgb::Rgb;
use image::GenericImageView;
use smartcore::cluster::kmeans::*;
use smartcore::linalg::naive::dense_matrix::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Palette {
    pixels: Vec<Rgb>,
    options: PaletteOptions,
}

impl Palette {
    pub fn from_local_image(
        image_path: &str,
        options: PaletteOptions,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let image = image::open(image_path)?;
        Ok(Palette::from(image, options))
    }

    pub fn from_remote_image(
        url: &str,
        options: PaletteOptions,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let image = fetch_image(url)?;
        Ok(Palette::from(image, options))
    }

    fn from(image: image::DynamicImage, options: PaletteOptions) -> Self {
        let resized_image = resize(image);
        let pixels = Palette::make_pixels(resized_image, &options);
        Palette { pixels, options }
    }

    pub fn inspect(&self) -> Result<Vec<Rgb>, Box<dyn std::error::Error>> {
        let data = self
            .pixels
            .iter()
            .map(|rgb| rgb.to_vec())
            .collect::<Vec<_>>();
        let matrix = DenseMatrix::from_2d_vec(&data);
        let k_means = KMeans::fit(
            &matrix,
            KMeansParameters::default().with_k(self.find_best_k()),
        )?;
        let ret = k_means.predict(&matrix)?;
        let mut cluster_map: HashMap<u64, ColorCluster> = HashMap::new();

        for (i, label) in ret.iter().enumerate() {
            let cluster = cluster_map
                .entry(*label as u64)
                .or_insert(ColorCluster::new(Rgb(255, 255, 255)));
            cluster.add(self.pixels[i]);
        }

        let mut colors = Colors::new();
        for cluster in cluster_map.values_mut() {
            cluster.recalculate_centroid();
            colors.add(cluster.clone());
        }

        colors.sort();
        Ok(colors.get_colors())
    }

    fn find_best_k(&self) -> usize {
        let size = self
            .make_hue_circle_clusters()
            .iter()
            .filter(|c| 0 != ((c.len() as f64 / self.pixels.len() as f64) * 100.0) as u64)
            .collect::<Vec<_>>()
            .len();

        if 2 < size {
            return size;
        } else {
            return 2 as usize;
        }
    }

    fn make_pixels(image: image::DynamicImage, options: &PaletteOptions) -> Vec<Rgb> {
        let mut data: Vec<Rgb> = vec![];
        for v in 0..image.height() {
            for u in 0..image.width() {
                let pix = image.get_pixel(u, v);
                let rgb = Rgb(pix[0], pix[1], pix[2]);
                if !options.colors.contains(&rgb) {
                    data.push(rgb);
                }
            }
        }
        data
    }

    fn make_hue_circle_clusters(&self) -> [ColorCluster; 24] {
        let mut colors: [ColorCluster; 24] = [
            ColorCluster::new(Rgb(203, 72, 41)),
            ColorCluster::new(Rgb(204, 106, 41)),
            ColorCluster::new(Rgb(213, 149, 51)),
            ColorCluster::new(Rgb(222, 180, 55)),
            ColorCluster::new(Rgb(223, 210, 56)),
            ColorCluster::new(Rgb(170, 181, 71)),
            ColorCluster::new(Rgb(147, 180, 71)),
            ColorCluster::new(Rgb(126, 181, 71)),
            ColorCluster::new(Rgb(89, 181, 71)),
            ColorCluster::new(Rgb(71, 181, 89)),
            ColorCluster::new(Rgb(0, 148, 83)),
            ColorCluster::new(Rgb(0, 151, 148)),
            ColorCluster::new(Rgb(0, 186, 207)),
            ColorCluster::new(Rgb(0, 153, 206)),
            ColorCluster::new(Rgb(19, 110, 171)),
            ColorCluster::new(Rgb(56, 80, 133)),
            ColorCluster::new(Rgb(66, 56, 133)),
            ColorCluster::new(Rgb(88, 61, 143)),
            ColorCluster::new(Rgb(116, 61, 143)),
            ColorCluster::new(Rgb(144, 62, 132)),
            ColorCluster::new(Rgb(202, 70, 132)),
            ColorCluster::new(Rgb(201, 71, 166)),
            ColorCluster::new(Rgb(201, 71, 114)),
            ColorCluster::new(Rgb(202, 71, 92)),
        ];

        for pixel in self.pixels.iter() {
            let (index, _min_value) =
                colors
                    .iter()
                    .enumerate()
                    .fold((0, f64::INFINITY), |acc, (i, c)| {
                        let distance = c.distance(pixel);
                        let min_value = distance.min(acc.1);
                        let min_value_index =
                            if min_value.partial_cmp(&acc.1).unwrap() == Ordering::Equal {
                                acc.0
                            } else {
                                i
                            };
                        (min_value_index, min_value)
                    });
            colors[index].add(pixel.clone());
        }
        colors
    }
}
