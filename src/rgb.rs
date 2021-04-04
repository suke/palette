#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    pub fn to_vec(&self) -> Vec<f64> {
        vec![self.0 as f64, self.1 as f64, self.2 as f64]
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[test]
fn test_to_hex() {
    let rgb = Rgb(255, 255, 255);
    assert_eq!(rgb.to_hex(), "#FFFFFF");

    let rgb = Rgb(0, 0, 0);
    assert_eq!(rgb.to_hex(), "#000000");

    let rgb = Rgb(10, 10, 10);
    assert_eq!(rgb.to_hex(), "#0A0A0A");

    let rgb = Rgb(255, 0, 0);
    assert_eq!(rgb.to_hex(), "#FF0000");
}
