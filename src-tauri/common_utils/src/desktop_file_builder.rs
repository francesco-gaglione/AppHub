use std::path::PathBuf;
use regex::Regex;
use log::info;

pub struct DesktopFileBuilder {
    /// The `type_` field represents the type of the application. It's usually "Application" for desktop applications.
    type_: Option<String>,

    /// The `version` field represents the version of the desktop file format.
    version: Option<String>,

    /// The `name` field represents the name of the application.
    name: Option<String>,

    /// The `comment` field represents a short comment about the application.
    comment: Option<String>,

    /// The `path` field represents the path of the application on the file system.
    path: Option<String>,

    /// The `exec` field represents the command to execute to start the application.
    exec: Option<String>,

    /// The `icon` field represents the path of the application's icon.
    icon: Option<String>,

    /// The `terminal` field indicates whether the application should be run in a terminal.
    terminal: Option<bool>,

    /// The `categories` field represents the categories the application belongs to.
    categories: Option<String>,

    /// The `no_sandbox` field indicates whether the application should be run in a sandbox.
    no_sanbox: Option<bool>,
}

impl DesktopFileBuilder {
    pub fn new() -> Self {
        Self {
            type_: None,
            version: None,
            name: None,
            comment: None,
            path: None,
            exec: None,
            icon: None,
            terminal: None,
            categories: None,
            no_sanbox: None,
        }
    }

    /// Read a .desktop file from a given path and return a DesktopFileBuilder instance.
    pub fn from_desktop_entry_path(path: &PathBuf, is_app_hub_app: bool) -> Result<Self, &'static str> {
        // Read the file content
        let file_content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to read file: {}", e);
                return Err("Failed to read .desktop file");
            }
        };

        // Check if the file content contains the AppHub specific field
        if is_app_hub_app && !file_content.contains("X-AppHub=true") {
            return Err("Invalid .desktop file");
        }

        // Parse the file content
        let mut desktop_file_builder = DesktopFileBuilder::new();

        let re_type = Regex::new(r"(?m)Type=(.*)$").unwrap();
        let re_version = Regex::new(r"(?m)Version=(.*)$").unwrap();
        let re_name = Regex::new(r"(?m)Name=(.*)$").unwrap();
        let re_comment = Regex::new(r"(?m)Comment=(.*)$").unwrap();
        let re_path = Regex::new(r"(?m)Path=(.*)$").unwrap();
        let re_exec = Regex::new(r"(?m)Exec=(.*)$").unwrap();
        let re_icon = Regex::new(r"(?m)Icon=(.*)$").unwrap();
        let re_terminal = Regex::new(r"(?m)Terminal=(.*)$").unwrap();
        let re_categories = Regex::new(r"(?m)Categories=(.*)$").unwrap();

        if let Some(cap) = re_type.captures(&file_content) {
            info!("Setting 'Type' to '{}'", &cap[1]);
            desktop_file_builder.set_type(cap[1].to_string());
        }
        if let Some(cap) = re_version.captures(&file_content) {
            info!("Setting 'Version' to '{}'", &cap[1]);
            desktop_file_builder.set_version(cap[1].to_string());
        }
        if let Some(cap) = re_name.captures(&file_content) {
            info!("Setting 'Name' to '{}'", &cap[1]);
            desktop_file_builder.set_name(cap[1].to_string());
        }
        if let Some(cap) = re_comment.captures(&file_content) {
            info!("Setting 'Comment' to '{}'", &cap[1]);
            desktop_file_builder.set_comment(cap[1].to_string());
        }
        if let Some(cap) = re_path.captures(&file_content) {
            info!("Setting 'Path' to '{}'", &cap[1]);
            desktop_file_builder.set_path(cap[1].to_string());
        }
        if let Some(cap) = re_exec.captures(&file_content) {
            info!("Setting 'Exec' to '{}'", &cap[1]);
            desktop_file_builder.set_exec(cap[1].to_string());
        }
        if let Some(cap) = re_icon.captures(&file_content) {
            info!("Setting 'Icon' to '{}'", &cap[1]);
            desktop_file_builder.set_icon(cap[1].to_string());
        }
        if let Some(cap) = re_terminal.captures(&file_content) {
            info!("Setting 'Terminal' to '{}'", &cap[1]);
            desktop_file_builder.set_terminal(cap[1].eq("true"));
        }
        if let Some(cap) = re_categories.captures(&file_content) {
            info!("Setting 'Categories' to '{}'", &cap[1]);
            desktop_file_builder.set_categories(cap[1].split(";").map(|s| s.to_string()).collect());
        }

        info!("Successfully parsed .desktop file from path: {:?}", path);
        Ok(desktop_file_builder)
    }

    // Mandatory fields
    pub fn set_type(&mut self, type_: String) -> &mut Self {
        self.type_ = Some(type_);
        self
    }

    // Mandatory fields
    pub fn set_version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);
        self
    }

    // Mandatory fields
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = Some(path);
        self
    }

    // Mandatory fields
    pub fn set_exec(&mut self, exec: String) -> &mut Self {
        self.exec = Some(exec);
        self
    }

    pub fn set_icon(&mut self, icon: String) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn set_terminal(&mut self, terminal: bool) -> &mut Self {
        self.terminal = Some(terminal);
        self
    }

    pub fn set_categories(&mut self, categories: Vec<String>) -> &mut Self {
        let categories = categories.join(";");
        self.categories = Some(categories);
        self
    }

    pub fn set_no_sandbox(&mut self, no_sandbox: bool) -> &mut Self {
        self.no_sanbox = Some(no_sandbox);
        self
    }

    /// Generate the content of the .desktop file as a string.
    pub fn generate_content_string(self) -> Result<String, &'static str> {
        // Check mandatory fields
        if self.type_.is_none() {
            return Err("Type field is mandatory");
        }
        if self.name.is_none() {
            return Err("Name field is mandatory");
        }
        if self.exec.is_none() {
            return Err("Exec field is mandatory");
        }

        // Create the file content
        let mut desktop_file_content = String::from("[Desktop Entry]\n");

        if let Some(type_) = self.type_ {
            desktop_file_content.push_str(&format!("Type={}\n", type_));
        }

        if let Some(version) = self.version {
            desktop_file_content.push_str(&format!("Version={}\n", version));
        }

        if let Some(name) = self.name {
            desktop_file_content.push_str(&format!("Name={}\n", name));
        }

        if let Some(comment) = self.comment {
            desktop_file_content.push_str(&format!("Comment={}\n", comment));
        }

        if let Some(path) = self.path {
            desktop_file_content.push_str(&format!("Path={}\n", path));
        }

        if let Some(exec) = self.exec {
            if self.no_sanbox.is_some() && self.no_sanbox.unwrap() {
                desktop_file_content.push_str(&format!("Exec={} --no-sandbox\n", exec));
            } else {
                desktop_file_content.push_str(&format!("Exec={}\n", exec));
            }
        }

        if let Some(icon) = self.icon {
            desktop_file_content.push_str(&format!("Icon={}\n", icon));
        }

        if let Some(terminal) = self.terminal {
            desktop_file_content.push_str(&format!("Terminal={}\n", terminal));
        }

        if let Some(categories) = self.categories {
            desktop_file_content.push_str(&format!("Categories={}\n", categories));
        }

        // AppHub specific fields
        desktop_file_content.push_str("X-AppHub=true\n");

        Ok(desktop_file_content)
    }

    pub fn type_(&self) -> Option<String> {
        self.type_.clone()
    }

    pub fn version(&self) -> Option<String> {
        self.version.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn exec(&self) -> Option<String> {
        self.exec.clone()
    }

    pub fn comment(&self) -> Option<String> {
        self.comment.clone()
    }

    pub fn icon(&self) -> Option<String> {
        self.icon.clone()
    }

    pub fn terminal(&self) -> Option<bool> {
        self.terminal
    }

    pub fn categories(&self) -> Option<String> {
        self.categories.clone()
    }

    pub fn no_sanbox(&self) -> Option<bool> {
        self.no_sanbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = DesktopFileBuilder::new();
        assert!(builder.type_().is_none());
        assert!(builder.version().is_none());
        assert!(builder.name().is_none());
        assert!(builder.comment().is_none());
        assert!(builder.path().is_none());
        assert!(builder.exec().is_none());
        assert!(builder.icon().is_none());
        assert!(builder.terminal().is_none());
        assert!(builder.categories().is_none());
        assert!(builder.no_sanbox().is_none());
    }

    #[test]
    fn test_setters() {
        let mut builder = DesktopFileBuilder::new();
        builder.set_type("Application".to_string());
        builder.set_version("1.0".to_string());
        builder.set_name("Test App".to_string());
        builder.set_comment("This is a test".to_string());
        builder.set_path("/usr/bin/test".to_string());
        builder.set_exec("test".to_string());
        builder.set_icon("/usr/share/icons/test.png".to_string());
        builder.set_terminal(true);
        builder.set_categories(vec!["Utility".to_string()]);
        builder.set_no_sandbox(true);

        assert_eq!(builder.type_(), Some("Application".to_string()));
        assert_eq!(builder.version(), Some("1.0".to_string()));
        assert_eq!(builder.name(), Some("Test App".to_string()));
        assert_eq!(builder.comment(), Some("This is a test".to_string()));
        assert_eq!(builder.path(), Some("/usr/bin/test".to_string()));
        assert_eq!(builder.exec(), Some("test".to_string()));
        assert_eq!(builder.icon(), Some("/usr/share/icons/test.png".to_string()));
        assert_eq!(builder.terminal(), Some(true));
        assert_eq!(builder.categories(), Some("Utility".to_string()));
        assert_eq!(builder.no_sanbox(), Some(true));
    }


    #[test]
    fn test_generate_content_string() {
        let mut builder = DesktopFileBuilder::new();
        builder.set_type("Application".to_string());
        builder.set_version("1.0".to_string());
        builder.set_name("Test App".to_string());
        builder.set_comment("This is a test".to_string());
        builder.set_path("/usr/bin/test".to_string());
        builder.set_exec("test".to_string());
        builder.set_icon("/usr/share/icons/test.png".to_string());
        builder.set_terminal(true);
        builder.set_categories(vec!["Utility".to_string()]);
        builder.set_no_sandbox(true);

        let content = builder.generate_content_string().unwrap();
        let expected_content = "[Desktop Entry]\nType=Application\nVersion=1.0\nName=Test App\nComment=This is a test\nPath=/usr/bin/test\nExec=test --no-sandbox\nIcon=/usr/share/icons/test.png\nTerminal=true\nCategories=Utility\nX-AppHub=true\n";
        assert_eq!(content, expected_content);
    }
}
