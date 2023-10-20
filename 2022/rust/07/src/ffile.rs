#[allow(dead_code)]
pub struct FFile {
    name: String,
    size: usize,
}

impl FFile {
    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: size,
        }
    }
}
