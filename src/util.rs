//! gimp model.

#[derive(Debug, Clone)]
pub struct Image {
    pub id: usize,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<String>,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub images: Vec<Image>,
    next_id: usize,
}

impl Default for Model {
    fn default() -> Self { Self { images: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, path: &str, width: u32, height: u32) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.images.push(Image { id, path: path.into(), width, height, layers: vec!["background".into()], modified: false });
        id
    }

    pub fn add_layer(&mut self, id: usize, name: &str) -> bool {
        if let Some(i) = self.images.iter_mut().find(|i| i.id == id) {
            i.layers.push(name.into());
            i.modified = true;
            true
        } else { false }
    }

    pub fn save(&mut self, id: usize) -> bool {
        if let Some(i) = self.images.iter_mut().find(|i| i.id == id) {
            i.modified = false;
            true
        } else { false }
    }

    pub fn close(&mut self, id: usize) -> bool {
        let before = self.images.len();
        self.images.retain(|i| i.id != id);
        self.images.len() < before
    }

    pub fn list(&self) -> &[Image] { &self.images }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_layer_save_close() {
        let mut m = Model::new();
        let id = m.open("/tmp/img.png", 800, 600);
        assert!(m.add_layer(id, "text"));
        assert!(m.save(id));
        assert!(m.close(id));
    }
}
