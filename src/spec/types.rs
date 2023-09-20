use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub enum OrderEnum {
    #[default]
    Ascending,
    Explicit,
    Descending,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PluginTypeEnum {
    Required,
    Optional,
    Recommended,
    NotUsable,
    CouldBeUsable,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginType {
    #[serde(rename = "@name")]
    pub name: PluginTypeEnum,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginTypeDescriptor {
    #[serde(rename = "$value")]
    pub value: PluginTypeDescriptorEnum,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PluginTypeDescriptorEnum {
    #[serde(rename = "dependencyType")]
    DependencyType(DependencyPluginType),
    #[serde(rename = "type")]
    PluginType(PluginType),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyPluginType {
    pub default_type: PluginType,
    pub patterns: DependencyPatternList,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyPatternList {
    pub pattern: Vec<DependencyPattern>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyPattern {
    pub dependencies: CompositeDependency,
    #[serde(rename = "type")]
    pub typ: PluginType,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StepList {
    #[serde(rename = "@order", default)]
    pub order: OrderEnum,

    #[serde(rename = "installStep")]
    pub install_step: Vec<InstallStep>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstallStep {
    #[serde(rename = "@name")]
    pub name: String,

    pub visible: Option<CompositeDependency>,

    #[serde(rename = "optionalFileGroups")]
    pub optional_file_groups: GroupList,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleDependency {
    #[serde(rename = "@operator")]
    pub operator: DependencyOperator,
    #[serde(rename = "$value")]
    pub list: Vec<CompositeDependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompositeDependency {
    #[serde(rename = "fileDependency")]
    File(FileDependency),
    #[serde(rename = "flagDependency")]
    Flag(FlagDependency),
    #[serde(rename = "gameDependency")]
    Game(VersionDependency),
    #[serde(rename = "fommDependency")]
    Fomm(VersionDependency),
    #[serde(rename = "dependencies")]
    Dependency(ModuleDependency),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlagDependency {
    #[serde(rename = "@flag")]
    pub flag: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionDependency {
    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileDependency {
    #[serde(rename = "@file")]
    pub file_name: String,
    #[serde(rename = "@state")]
    pub state: DependencyState,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyState {
    Active,
    Inactive,
    Missing,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyOperator {
    And,
    Or,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileList {
    #[serde(rename = "$value")]
    pub list: Option<Vec<FileType>>, //FIXME?
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileListEnum {
    #[serde(rename = "file")]
    File(FileType),
    #[serde(rename = "folder")]
    Folder(FolderType),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileType {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@destination")]
    pub destination: Option<String>,
    #[serde(rename = "@alwaysInstall")]
    pub always_install: Option<String>,
    #[serde(rename = "@installIfUsable", default = "false_bool")]
    pub install_if_usable: bool,
    pub priority: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FolderType {
    #[serde(rename = "@source")]
    pub source: String,
    #[serde(rename = "@destination")]
    pub destination: Option<String>,
    #[serde(rename = "@alwaysInstall")]
    pub always_install: Option<String>,
    #[serde(rename = "@installIfUsable", default = "false_bool")]
    pub install_if_usable: bool,
    pub priority: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupList {
    #[serde(rename = "@order", default)]
    pub order: OrderEnum,
    pub group: Vec<Group>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type")]
    pub typ: GroupType,

    pub plugins: PluginList,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroupType {
    SelectAtLeastOne,
    SelectAtMostOne,
    SelectExactlyOne,
    SelectAll,
    SelectAny,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginList {
    #[serde(rename = "@order", default)]
    pub order: OrderEnum,

    pub plugin: Vec<Plugin>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plugin {
    #[serde(rename = "@name")]
    pub name: String,

    pub description: String,

    pub image: Option<Image>,

    pub files: Option<FileList>,
    #[serde(rename = "conditionFlags")]
    pub condition_flags: Option<ConditionFlagList>,

    #[serde(rename = "typeDescriptor")]
    pub type_descriptor: Option<PluginTypeDescriptor>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image {
    #[serde(rename = "@path")]
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeaderImage {
    #[serde(rename = "@path")]
    pub path: Option<String>,
    #[serde(rename = "@showImage", default = "false_bool")]
    pub show_image: bool,
    #[serde(rename = "@showFade", default = "false_bool")]
    pub show_fade: bool,
    pub height: Option<isize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionFlagList {
    pub flag: Vec<SetConditionFlag>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetConditionFlag {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "$value")]
    pub flag_value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalFileInstallList {
    pub patterns: ConditionalInstallPatternList,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalInstallPatternList {
    pub pattern: Vec<ConditionalInstallPattern>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalInstallPattern {
    pub dependencies: CompositeDependency,
    pub files: FileList,
}

fn false_bool() -> bool {
    false
}
