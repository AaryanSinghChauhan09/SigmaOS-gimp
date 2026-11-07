//! gimp model — GNU Image Manipulation Program.

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub images: Vec<Image>,
}

impl Default for Model {
    fn default() -> Self { Self { images: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, image: Image) { self.images.push(image); }

    pub fn add_layer(&mut self, path: &str, layer: &str) -> bool {
        if let Some(i) = self.images.iter_mut().find(|i| i.path == path) { i.layers.push(layer.into()); true } else { false }
    }

    pub fn save(&self, path: &str) -> bool {
        self.images.iter().any(|i| i.path == path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_layer_save() {
        let mut g = Model::new();
        g.open(Image { path: "img.xcf".into(), width: 800, height: 600, layers: vec![] });
        assert!(g.add_layer("img.xcf", "background"));
        assert!(g.save("img.xcf"));
    }
}
