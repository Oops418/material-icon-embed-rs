use rust_embed::Embed;

pub mod material_icon_file;
pub mod material_icon_folder;

#[derive(Embed)]
#[folder = "assets"]
pub struct Asset;

#[cfg(test)]
mod tests {
    use crate::material_icon_file::MaterialIconFile;
    use crate::material_icon_folder::MaterialIconFolder;

    #[test]
    fn test_file_extension() {
        assert_eq!(
            "icons/markdown.svg",
            MaterialIconFile::from_extension("md").unwrap().path()
        );
    }

    #[test]
    fn test_file_extension_not_found() {
        assert!(MaterialIconFile::from_extension("unknown").is_none());
    }

    #[test]
    fn test_file_name() {
        assert_eq!(
            "icons/bun.svg",
            MaterialIconFile::from_filename("bun.lock").unwrap().path()
        );
    }

    #[test]
    fn test_file_name_not_found() {
        assert!(MaterialIconFile::from_filename("unknown").is_none());
    }

    #[test]
    fn test_folder_name() {
        assert_eq!(
            "icons/folder-macos.svg",
            MaterialIconFolder::from_folder_name("DS_Store")
                .unwrap()
                .path()
        );
    }

    #[test]
    fn test_folder_name_not_found() {
        assert!(MaterialIconFolder::from_folder_name("unknown").is_none());
    }
}
