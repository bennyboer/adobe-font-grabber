use app_dirs::*;


const APP_INFO: AppInfo = AppInfo{name: "CoreSync", author: "Adobe"};

pub struct Config {
    /// Folder to search for font files in
    pub font_root_folder_path: String,

    /// Path to the entitlements.xml file
    pub entitlements_file_path: String,

    /// Folder to save found Adobe fonts to
    pub target_folder_path: String,
    
}

impl Config {
    pub(crate) fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 2 {
            return Err(r#"You need to indicate a save location. ex. `afg "C:\'Fonts-grabbed"`"#);
        }
        let root = get_app_dir(AppDataType::UserConfig, &APP_INFO, "plugins/livetype/r").unwrap();
        let font_root_folder_path = root.display().to_string().to_owned();

        let entitlements = get_app_dir(AppDataType::UserConfig, &APP_INFO, "plugins/livetype/c/").unwrap();
        let  mut entitlements_file_path = entitlements.display().to_string().to_owned();
        let en_xml = "/entitlements.xml".to_string();

        let target_folder_path = args[1].clone();
        
        entitlements_file_path.push_str(&en_xml);

        Ok(Config {
            font_root_folder_path,
            entitlements_file_path,
            target_folder_path,
        })
    }
}
