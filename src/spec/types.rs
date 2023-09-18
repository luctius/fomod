use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub enum OrderEnum {
    #[default]
    Ascending,
    Explicit,
    Descending,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum PluginTypeEnum {
    Required,
    Optional,
    Recommended,
    NotUsable,
    CouldBeUsable,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PluginType {
    #[serde(rename = "@name")]
    pub name: PluginTypeEnum,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PluginTypeDescriptor {
    #[serde(rename = "$value")]
    pub value: PluginTypeDescriptorEnum,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum PluginTypeDescriptorEnum {
    #[serde(rename = "dependencyType")]
    DependencyType(DependencyPluginType),
    #[serde(rename = "type")]
    PluginType(PluginType),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DependencyPluginType {
    pub default_type: PluginType,
    pub patterns: DependencyPatternList,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DependencyPatternList {
    pub pattern: Vec<DependencyPattern>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DependencyPattern {
    pub dependencies: CompositeDependency,
    #[serde(rename = "type")]
    pub typ: PluginType,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct StepList {
    #[serde(rename = "@order", default)]
    pub order: OrderEnum,

    #[serde(rename = "installStep")]
    pub install_step: Vec<InstallStep>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct InstallStep {
    #[serde(rename = "@name")]
    pub name: String,

    pub visible: Option<CompositeDependency>,

    #[serde(rename = "optionalFileGroups")]
    pub optional_file_groups: GroupList,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ModuleDependency {
    #[serde(rename = "@operator")]
    pub operator: DependencyOperator,
    #[serde(rename = "$value")]
    pub list: Vec<CompositeDependency>,
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct FlagDependency {
    #[serde(rename = "@flag")]
    pub flag: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct VersionDependency {
    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FileDependency {
    #[serde(rename = "@file")]
    pub file_name: String,
    #[serde(rename = "@state")]
    pub state: DependencyState,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum DependencyState {
    Active,
    Inactive,
    Missing,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum DependencyOperator {
    And,
    Or,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FileList {
    #[serde(rename = "$value")]
    pub list: Option<Vec<FileType>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum FileListEnum {
    #[serde(rename = "file")]
    File(FileType),
    #[serde(rename = "folder")]
    Folder(FolderType),
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct GroupList {
    #[serde(rename = "@order", default)]
    pub order: OrderEnum,
    pub group: Vec<Group>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Group {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type")]
    pub typ: GroupType,

    pub plugins: PluginList,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum GroupType {
    SelectAtLeastOne,
    SelectAtMostOne,
    SelectExactlyOne,
    SelectAll,
    SelectAny,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PluginList {
    #[serde(rename = "@order", default)]
    pub order: OrderEnum,

    pub plugin: Vec<Plugin>,
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct Image {
    #[serde(rename = "@path")]
    pub path: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HeaderImage {
    #[serde(rename = "@path")]
    pub path: Option<String>,
    #[serde(rename = "@showImage", default = "false_bool")]
    pub show_image: bool,
    #[serde(rename = "@showFade", default = "false_bool")]
    pub show_fade: bool,
    pub height: Option<isize>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConditionFlagList {
    pub flag: Vec<SetConditionFlag>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SetConditionFlag {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "$value")]
    pub flag_value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConditionalFileInstallList {
    pub patterns: ConditionalInstallPatternList,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConditionalInstallPatternList {
    pub pattern: Vec<ConditionalInstallPattern>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConditionalInstallPattern {
    pub dependencies: CompositeDependency,
    pub files: FileList,
}

fn false_bool() -> bool {
    false
}
