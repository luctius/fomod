use serde::Deserialize;

use super::types::{ConditionalFileInstallList, FileList, HeaderImage, ModuleDependency, StepList};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Info {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "Website")]
    pub website: Option<String>,
    #[serde(rename = "CategoryId")]
    pub category_id: Option<usize>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    #[serde(rename = "moduleName")]
    pub module_name: String,

    #[serde(rename = "moduleImage")]
    pub module_image: Option<HeaderImage>,

    #[serde(rename = "moduleDependencies")]
    pub module_dependencies: Option<ModuleDependency>,

    #[serde(rename = "requiredInstallFiles")]
    pub required_install_files: Option<FileList>,

    #[serde(rename = "installSteps")]
    pub install_steps: Option<StepList>,

    #[serde(rename = "conditionalFileInstalls")]
    pub conditional_file_installs: Option<ConditionalFileInstallList>,
}
