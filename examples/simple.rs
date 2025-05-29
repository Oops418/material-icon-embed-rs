use material_icon_embed_rs::{
    Asset, material_icon_file::MaterialIconFile, material_icon_folder::MaterialIconFolder,
};

fn main() {
    // using file extension
    let icon = MaterialIconFile::from_extension("rs");
    match icon {
        Some(icon) => {
            println!("Icon path: {}", icon.path());
            match Asset::get(icon.path().as_str()) {
                Some(asset) => {
                    println!("{:?}", std::str::from_utf8(asset.data.as_ref()));
                }
                None => println!("No asset found for the given icon."),
            };
        }
        None => println!("No icon found for the given extension."),
    }

    // using filename
    let icon = MaterialIconFile::from_filename("uv.toml");
    match icon {
        Some(icon) => {
            println!("Icon path: {}", icon.path());
            match Asset::get(icon.path().as_str()) {
                Some(asset) => {
                    println!("{:?}", std::str::from_utf8(asset.data.as_ref()));
                }
                None => println!("No asset found for the given icon."),
            };
        }
        None => println!("No icon found for the given filename."),
    }

    // using folder name
    let icon = MaterialIconFolder::from_folder_name("backends");
    match icon {
        Some(icon) => {
            println!("Icon path: {}", icon.path());
            match Asset::get(icon.path().as_str()) {
                Some(asset) => {
                    println!("{:?}", std::str::from_utf8(asset.data.as_ref()));
                }
                None => println!("No asset found for the given icon."),
            };
        }
        None => println!("No icon found for the given folder name."),
    }
}
