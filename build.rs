use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content =
        fs::read_to_string("vscode-material-icon-theme/src/core/icons/fileIcons.ts")?;
    let folder_content =
        fs::read_to_string("vscode-material-icon-theme/src/core/icons/folderIcons.ts")?;

    let mut extension_map: HashMap<String, String> = HashMap::new();
    let mut filename_map: HashMap<String, String> = HashMap::new();
    let mut folder_map: HashMap<String, String> = HashMap::new();

    parse_file_icons(&file_content, &mut extension_map, &mut filename_map)?;

    parse_folder_icons(&folder_content, &mut folder_map)?;

    generate_material_icon_file_enum(&extension_map, &filename_map)?;

    generate_material_icon_folder_enum(&folder_map)?;

    copy_icons_folder()?;

    println!("cargo:rerun-if-changed=vscode-material-icon-theme/src/core/icons/fileIcons.ts");
    println!("cargo:rerun-if-changed=vscode-material-icon-theme/src/core/icons/folderIcons.ts");

    Ok(())
}

fn parse_file_icons(
    content: &str,
    extension_map: &mut HashMap<String, String>,
    filename_map: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let object_regex = Regex::new(r#"\{\s*name:\s*['"`]([^'"`]+)['"`][^}]*?\}"#)?;
    let extensions_regex = Regex::new(r#"fileExtensions:\s*\[([^\]]*)\]"#)?;
    let filenames_regex = Regex::new(r#"fileNames:\s*\[([^\]]*)\]"#)?;
    let string_regex = Regex::new(r#"['"`]([^'"`]+)['"`]"#)?;

    for object_match in object_regex.find_iter(content) {
        let object_block = object_match.as_str();

        if let Some(name_match) =
            Regex::new(r#"name:\s*['"`]([^'"`]+)['"`]"#)?.captures(object_block)
        {
            let name = name_match.get(1).unwrap().as_str().to_string();

            if let Some(ext_match) = extensions_regex.captures(object_block) {
                let extensions_str = ext_match.get(1).unwrap().as_str();
                for ext_capture in string_regex.captures_iter(extensions_str) {
                    let extension = ext_capture.get(1).unwrap().as_str().to_string();
                    extension_map.insert(extension, name.clone());
                }
            }

            if let Some(filename_match) = filenames_regex.captures(object_block) {
                let filenames_str = filename_match.get(1).unwrap().as_str();
                for filename_capture in string_regex.captures_iter(filenames_str) {
                    let filename = filename_capture.get(1).unwrap().as_str().to_string();
                    filename_map.insert(filename, name.clone());
                }
            }
        }
    }
    Ok(())
}

fn parse_folder_icons(
    content: &str,
    folder_map: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let icon_object_regex =
        Regex::new(r#"\{\s*name:\s*['"`]([^'"`]+)['"`][^}]*?folderNames:\s*\[([^\]]*)\][^}]*?\}"#)?;
    let string_regex = Regex::new(r#"['"`]([^'"`]+)['"`]"#)?;

    for object_match in icon_object_regex.find_iter(content) {
        let object_block = object_match.as_str();
        let captures = icon_object_regex.captures(object_block).unwrap();

        let icon_name = captures.get(1).unwrap().as_str().to_string();
        let folder_names_str = captures.get(2).unwrap().as_str();

        for folder_capture in string_regex.captures_iter(folder_names_str) {
            let folder_name = folder_capture.get(1).unwrap().as_str().to_string();
            folder_map.insert(folder_name, icon_name.clone());
        }
    }
    Ok(())
}

fn generate_material_icon_file_enum(
    extension_map: &HashMap<String, String>,
    filename_map: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let dest_path = Path::new("src").join("material_icon_file.rs");
    let mut file = fs::File::create(&dest_path)?;

    let mut all_icons: std::collections::HashSet<String> = std::collections::HashSet::new();
    for icon_name in extension_map.values() {
        all_icons.insert(icon_name.clone());
    }
    for icon_name in filename_map.values() {
        all_icons.insert(icon_name.clone());
    }

    let mut icons: Vec<String> = all_icons.into_iter().collect();
    icons.sort();

    writeln!(file, "// Auto-generated file - do not edit manually")?;
    writeln!(file, "use std::fmt;")?;
    writeln!(file)?;
    writeln!(file, "#[allow(dead_code)]")?;
    writeln!(file, "#[derive(Debug, Clone, PartialEq, Eq, Hash)]")?;
    writeln!(file, "pub enum MaterialIconFile {{")?;

    for icon_name in &icons {
        let enum_name = to_enum_name(icon_name);
        writeln!(file, "    {},", enum_name)?;
    }

    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl MaterialIconFile {{")?;

    writeln!(file, "    pub fn path(&self) -> String {{")?;
    writeln!(file, "        match self {{")?;
    for icon_name in &icons {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            MaterialIconFile::{} => \"icons/{}.svg\".to_string(),",
            enum_name, icon_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file)?;

    writeln!(
        file,
        "    pub fn from_extension(ext: &str) -> Option<Self> {{"
    )?;
    writeln!(file, "        match ext.to_lowercase().as_str() {{")?;

    let mut extensions: Vec<(&String, &String)> = extension_map.iter().collect();
    extensions.sort_by_key(|(ext, _)| ext.as_str());

    for (extension, icon_name) in &extensions {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            \"{}\" => Some(MaterialIconFile::{}),",
            extension.to_lowercase(),
            enum_name
        )?;
    }
    writeln!(file, "            _ => None,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file)?;

    writeln!(
        file,
        "    pub fn from_filename(name: &str) -> Option<Self> {{"
    )?;
    writeln!(file, "        match name {{")?;

    let mut filenames: Vec<(&String, &String)> = filename_map.iter().collect();
    filenames.sort_by_key(|(name, _)| name.as_str());

    for (filename, icon_name) in &filenames {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            \"{}\" => Some(MaterialIconFile::{}),",
            filename, enum_name
        )?;
    }
    writeln!(file, "            _ => None,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl fmt::Display for MaterialIconFile {{")?;
    writeln!(
        file,
        "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )?;
    writeln!(file, "        match self {{")?;
    for icon_name in &icons {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            MaterialIconFile::{} => write!(f, \"{}\"),",
            enum_name, icon_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    println!(
        "Generated material_icon_file.rs with {} file icons, {} extensions, {} filenames",
        icons.len(),
        extensions.len(),
        filenames.len()
    );
    Ok(())
}

fn generate_material_icon_folder_enum(
    folder_map: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let dest_path = Path::new("src").join("material_icon_folder.rs");
    let mut file = fs::File::create(&dest_path)?;

    let mut all_folder_icons: std::collections::HashSet<String> = std::collections::HashSet::new();
    for icon_name in folder_map.values() {
        all_folder_icons.insert(icon_name.clone());
    }

    let mut folder_icons: Vec<String> = all_folder_icons.into_iter().collect();
    folder_icons.sort();

    writeln!(file, "// Auto-generated file - do not edit manually")?;
    writeln!(file, "use std::fmt;")?;
    writeln!(file)?;
    writeln!(file, "#[allow(dead_code)]")?;
    writeln!(file, "#[derive(Debug, Clone, PartialEq, Eq, Hash)]")?;
    writeln!(file, "pub enum MaterialIconFolder {{")?;

    for icon_name in &folder_icons {
        let enum_name = to_enum_name(icon_name);
        writeln!(file, "    {},", enum_name)?;
    }

    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl MaterialIconFolder {{")?;

    writeln!(file, "    pub fn path(&self) -> String {{")?;
    writeln!(file, "        match self {{")?;
    for icon_name in &folder_icons {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            MaterialIconFolder::{} => \"icons/{}.svg\".to_string(),",
            enum_name, icon_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file)?;

    writeln!(
        file,
        "    pub fn from_folder_name(name: &str) -> Option<Self> {{"
    )?;
    writeln!(file, "        match name {{")?;

    let mut folder_names: Vec<(&String, &String)> = folder_map.iter().collect();
    folder_names.sort_by_key(|(name, _)| name.as_str());

    for (folder_name, icon_name) in &folder_names {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            \"{}\" => Some(MaterialIconFolder::{}),",
            folder_name, enum_name
        )?;
    }
    writeln!(file, "            _ => None,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;

    writeln!(file, "impl fmt::Display for MaterialIconFolder {{")?;
    writeln!(
        file,
        "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )?;
    writeln!(file, "        match self {{")?;
    for icon_name in &folder_icons {
        let enum_name = to_enum_name(icon_name);
        writeln!(
            file,
            "            MaterialIconFolder::{} => write!(f, \"{}\"),",
            enum_name, icon_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    println!(
        "Generated material_icon_folder.rs with {} folder icons, {} folder names",
        folder_icons.len(),
        folder_names.len()
    );
    Ok(())
}

fn copy_icons_folder() -> Result<(), Box<dyn std::error::Error>> {
    let source_dir = Path::new("vscode-material-icon-theme/icons");
    let dest_dir = Path::new("assets/icons");

    if source_dir.exists() {
        if dest_dir.exists() {
            fs::remove_dir_all(dest_dir)?;
        }

        copy_dir_all(source_dir, dest_dir)?;
        println!(
            "Copied icons folder from {} to {}",
            source_dir.display(),
            dest_dir.display()
        );
    } else {
        println!(
            "Warning: Source icons directory not found at {}",
            source_dir.display()
        );
    }

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn to_enum_name(s: &str) -> String {
    let sanitized = if s.chars().next().unwrap_or('a').is_ascii_digit() {
        format!("Icon{}", s)
    } else {
        s.to_string()
    };

    sanitized
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_ascii_uppercase(),
            _ => '_',
        })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<String>>()
        .join("")
}
