use super::ffile::FFile;

pub struct Folder {
    pub name: String,
    parent: Option<usize>,
    pub children: Vec<usize>,
    files: Vec<FFile>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        }
    }
}

pub struct CWD {
    pub folders: Vec<Folder>,
    cwd: usize,
}

impl CWD {
    pub fn new() -> Self {
        let root = Folder::new("/");

        Self {
            folders: vec![root],
            cwd: 0,
        }
    }

    pub fn add_file(&mut self, name: &str, size: usize) {
        let file = FFile::new(name, size);

        let folder = self.folders.get_mut(self.cwd).unwrap();
        folder.files.push(file);
    }

    pub fn add_folder(&mut self, name: &str) {
        let mut folder = Folder::new(name);
        folder.parent = Some(self.cwd);

        self.folders.push(folder);
        let child_id = self.folders.len() - 1;

        let cwd = self.folders.get_mut(self.cwd).unwrap();
        cwd.children.push(child_id);
    }

    pub fn change_directory(&mut self, dir: &str) -> Result<(), &'static str> {
        let cwd = self.folders.get(self.cwd).unwrap();

        match dir {
            ".." => {
                if let Some(parent) = cwd.parent {
                    self.cwd = parent;
                } else {
                    return Err("No parent found");
                }
            }
            "/" => self.cwd = 0,
            _ => {
                let new_dir = cwd
                    .children
                    .iter()
                    .find(|&&a| self.folders.get(a).unwrap().name == dir);

                if let Some(&id) = new_dir {
                    self.cwd = id;
                } else {
                    return Err("Invalid ID");
                }
            }
        }

        Ok(())
    }

    pub fn get_size(&self, id: usize) -> usize {
        let folder = self.folders.get(id).unwrap();

        let file_size: usize = folder.files.iter().map(|f| f.get_size()).sum();

        let mut sub_size = 0usize;

        for sub_folder_id in &folder.children {
            let sub_folder = self.folders.get(*sub_folder_id).unwrap();
            sub_size += sub_folder.files.iter().map(|f| f.get_size()).sum::<usize>();
            sub_size += sub_folder
                .children
                .iter()
                .map(|&id| self.get_size(id))
                .sum::<usize>();
        }

        file_size + sub_size
    }
}
