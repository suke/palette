use image::imageops::FilterType;
use image::GenericImageView;
use tokio::runtime;

const MAX_SIZE: usize = 400;

pub fn fetch_image(path: &str) -> Result<image::DynamicImage, Box<dyn std::error::Error>> {
    let rt = runtime::Runtime::new()?;
    let bytes = rt.block_on(async { reqwest::get(path).await?.bytes().await })?;
    let img = image::load_from_memory(&bytes).unwrap();
    Ok(img)
}

pub fn resize(img: image::DynamicImage) -> image::DynamicImage {
    let width = img.width() as usize;
    let height = img.height() as usize;

    if width < MAX_SIZE || height < MAX_SIZE {
        return img;
    }

    let (target_width, target_height) = if width > height {
        let ratio: f32 = MAX_SIZE as f32 / width as f32;
        (MAX_SIZE, (height as f32 * ratio) as usize)
    } else {
        let ratio: f32 = MAX_SIZE as f32 / height as f32;
        ((width as f32 * ratio) as usize, MAX_SIZE)
    };

    img.resize(
        target_width as u32,
        target_height as u32,
        FilterType::Lanczos3,
    )
}
