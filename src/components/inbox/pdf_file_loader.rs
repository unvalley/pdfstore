use std::{fs, path::Path};

use crate::domain::pdf_file::PdfFile;

pub struct PdfFileLoader;

impl Default for PdfFileLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfFileLoader {
    pub fn new() -> Self {
        Self {}
    }

    fn is_pdf(&mut self, entry: &fs::DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.ends_with(".pdf"))
            .unwrap_or(false)
    }

    pub fn load_files(&mut self, path: &Path) -> anyhow::Result<Vec<PdfFile>> {
        let mut result = Vec::new();
        // for entry in WalkDir::new(path).into_iter().filter_map(|entry| entry.ok()) {
        for entry in fs::read_dir(path)? {
            let e = entry?;
            if !self.is_pdf(&e) {
                continue;
            }
            let pdf_file = self.to_pdf_file(e);
            result.push(pdf_file)
        }
        Ok(result)
    }

    fn to_pdf_file(&mut self, entry: fs::DirEntry) -> PdfFile {
        let file_name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|_| "Invalid file name".to_string());
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path);

        match metadata {
            Ok(metadata) => PdfFile { file_name },
            Err(_) => todo!(),
        }
    }
}
