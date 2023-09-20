pub mod spec;

use std::io::BufReader;

use quick_xml::DeError;

pub use crate::spec::{
    types::{
        FileDependency, FileType, FlagDependency, HeaderImage, PluginTypeEnum, SetConditionFlag,
        VersionDependency,
    },
    Info,
};

use crate::spec::Config as SpecConfig;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub module_name: String,
    pub module_image: Option<HeaderImage>,
    pub module_dependencies: Option<DependencyOperator<Dependency>>,
    pub required_install_files: Vec<FileType>,
    pub install_steps: OrderEnum<InstallStep>,
    pub conditional_file_installs: Vec<ConditionalInstallPattern>,
}
impl From<SpecConfig> for Config {
    fn from(spec: SpecConfig) -> Self {
        let mut conditional_file_installs = Vec::new();

        conditional_file_installs.extend(
            spec.conditional_file_installs
                .map(|cfi| {
                    cfi.patterns
                        .pattern
                        .iter()
                        .map(|cfi| ConditionalInstallPattern::from(cfi.clone()))
                        .collect::<Vec<ConditionalInstallPattern>>()
                })
                .unwrap_or_default(),
        );

        Self {
            module_name: spec.module_name,
            module_image: spec.module_image,
            module_dependencies: spec
                .module_dependencies
                .map(|md| DependencyOperator::from(md)),
            required_install_files: spec
                .required_install_files
                .map(|rif| rif.list)
                .flatten()
                .unwrap_or_default(),
            install_steps: spec
                .install_steps
                .map(|is| OrderEnum::from(is))
                .unwrap_or_default(),
            conditional_file_installs,
        }
    }
}
impl TryFrom<&str> for Config {
    type Error = DeError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(SpecConfig::try_from(string)?))
    }
}
impl<T> TryFrom<BufReader<T>> for Config
where
    T: std::io::Read,
{
    type Error = DeError;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        Ok(Self::from(SpecConfig::try_from(reader)?))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dependency {
    File(FileDependency),
    Flag(FlagDependency),
    Game(VersionDependency),
    Fomm(VersionDependency),
    Dependency(DependencyOperator<Self>),
}
impl From<crate::spec::types::CompositeDependency> for Dependency {
    fn from(comp_dep: crate::spec::types::CompositeDependency) -> Self {
        use crate::spec::types::CompositeDependency;

        match comp_dep {
            CompositeDependency::File(f) => Self::File(f),
            CompositeDependency::Flag(f) => Self::Flag(f),
            CompositeDependency::Game(v) => Self::Game(v),
            CompositeDependency::Fomm(v) => Self::Fomm(v),
            CompositeDependency::Dependency(f) => Self::Dependency(DependencyOperator::from(f)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyOperator<T> {
    And(Vec<T>),
    Or(Vec<T>),
}
impl From<crate::spec::types::ModuleDependency> for DependencyOperator<Dependency> {
    fn from(mod_dep: crate::spec::types::ModuleDependency) -> Self {
        use crate::spec::types::DependencyOperator as DepOp;

        let mut list = Vec::new();
        for cd in mod_dep.list {
            list.push(Dependency::from(cd));
        }

        match mod_dep.operator {
            DepOp::And => DependencyOperator::And(list),
            DepOp::Or => DependencyOperator::Or(list),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderEnum<T> {
    Ascending(Vec<T>),
    Explicit(Vec<T>),
    Descending(Vec<T>),
}
impl<T> OrderEnum<T>
where
    T: Ord,
    T: Clone,
{
    pub fn vec_sorted(&self) -> Vec<T> {
        match self {
            Self::Ascending(v) => {
                let mut v = v.clone();
                v.sort();
                v
            }
            Self::Explicit(v) => v.clone(),
            Self::Descending(v) => {
                //FIXME Sort Descending
                let mut v = v.clone();
                v.sort();
                v
            }
        }
    }
    pub fn vec_sorted_mut(&mut self) -> &mut Vec<T> {
        match self {
            Self::Ascending(v) => {
                v.sort();
                v
            }
            Self::Explicit(v) => v,
            Self::Descending(v) => {
                //FIXME Sort Descending
                v.sort();
                v
            }
        }
    }
}
impl<T> Default for OrderEnum<T> {
    fn default() -> Self {
        Self::Ascending(Vec::new())
    }
}
impl From<spec::types::StepList> for OrderEnum<InstallStep> {
    fn from(step_list: spec::types::StepList) -> Self {
        let mut list = Vec::new();
        list.extend(
            step_list
                .install_step
                .iter()
                .map(|is| InstallStep::from(is.clone())),
        );

        use spec::types::OrderEnum;
        match step_list.order {
            OrderEnum::Ascending => Self::Ascending(list),
            OrderEnum::Explicit => Self::Explicit(list),
            OrderEnum::Descending => Self::Descending(list),
        }
    }
}
impl From<spec::types::GroupList> for OrderEnum<Group> {
    fn from(group_list: spec::types::GroupList) -> Self {
        let mut list = Vec::new();
        list.extend(group_list.group.iter().map(|is| Group::from(is.clone())));

        use spec::types::OrderEnum;
        match group_list.order {
            OrderEnum::Ascending => Self::Ascending(list),
            OrderEnum::Explicit => Self::Explicit(list),
            OrderEnum::Descending => Self::Descending(list),
        }
    }
}
impl From<spec::types::PluginList> for OrderEnum<Plugin> {
    fn from(plugin_list: spec::types::PluginList) -> Self {
        let mut list = Vec::new();
        list.extend(plugin_list.plugin.iter().map(|is| Plugin::from(is.clone())));

        use spec::types::OrderEnum;
        match plugin_list.order {
            OrderEnum::Ascending => Self::Ascending(list),
            OrderEnum::Explicit => Self::Explicit(list),
            OrderEnum::Descending => Self::Descending(list),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstallStep {
    pub name: String,
    pub visible: Option<Dependency>,
    pub optional_file_groups: OrderEnum<Group>,
}
impl PartialOrd for InstallStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.as_str().partial_cmp(other.name.as_str())
    }
}
impl Ord for InstallStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.as_str().cmp(other.name.as_str())
    }
}
impl From<spec::types::InstallStep> for InstallStep {
    fn from(install_step: spec::types::InstallStep) -> Self {
        Self {
            name: install_step.name,
            visible: install_step.visible.map(|v| Dependency::from(v)),
            optional_file_groups: OrderEnum::from(install_step.optional_file_groups),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GroupType<T> {
    SelectAtLeastOne(T),
    SelectAtMostOne(T),
    SelectExactlyOne(T),
    SelectAll(T),
    SelectAny(T),
}
impl From<(spec::types::GroupType, spec::types::PluginList)> for GroupType<OrderEnum<Plugin>> {
    fn from((gt, pl): (spec::types::GroupType, spec::types::PluginList)) -> Self {
        let oe = OrderEnum::from(pl);

        use spec::types::GroupType;
        match gt {
            GroupType::SelectAtLeastOne => Self::SelectAtLeastOne(oe),
            GroupType::SelectAtMostOne => Self::SelectAtMostOne(oe),
            GroupType::SelectExactlyOne => Self::SelectExactlyOne(oe),
            GroupType::SelectAll => Self::SelectAll(oe),
            GroupType::SelectAny => Self::SelectAny(oe),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Group {
    pub name: String,
    pub plugins: GroupType<OrderEnum<Plugin>>,
}
impl From<spec::types::Group> for Group {
    fn from(group: spec::types::Group) -> Self {
        Self {
            name: group.name,
            plugins: GroupType::from((group.typ, group.plugins)),
        }
    }
}
impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.as_str().partial_cmp(other.name.as_str())
    }
}
impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.as_str().cmp(other.name.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub image: Option<String>,

    pub files: Vec<FileType>,
    pub condition_flags: Vec<SetConditionFlag>,
    pub type_descriptor: Option<PluginTypeDescriptorEnum>,
}
impl From<spec::types::Plugin> for Plugin {
    fn from(plugin: spec::types::Plugin) -> Self {
        Self {
            name: plugin.name,
            description: plugin.description,
            image: plugin.image.map(|i| i.path),
            files: plugin.files.map(|fl| fl.list).flatten().unwrap_or_default(),
            condition_flags: plugin
                .condition_flags
                .map(|cfl| cfl.flag)
                .unwrap_or_default(),
            type_descriptor: plugin
                .type_descriptor
                .map(|td| PluginTypeDescriptorEnum::from(td)),
        }
    }
}
impl PartialOrd for Plugin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.as_str().partial_cmp(other.name.as_str())
    }
}
impl Ord for Plugin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.as_str().cmp(other.name.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PluginTypeDescriptorEnum {
    DependencyType(Vec<DependencyPattern>),
    PluginType(PluginTypeEnum),
}
impl From<spec::types::PluginTypeDescriptorEnum> for PluginTypeDescriptorEnum {
    fn from(ptde: spec::types::PluginTypeDescriptorEnum) -> Self {
        use spec::types::PluginTypeDescriptorEnum;
        match ptde {
            PluginTypeDescriptorEnum::DependencyType(dpt) => {
                //FIXME: DependencyPluginType::default_type not accounted for!!
                let mut list = Vec::new();
                list.extend(
                    dpt.patterns
                        .pattern
                        .iter()
                        .map(|dp| DependencyPattern::from(dp.clone())),
                );

                Self::DependencyType(list)
            }
            PluginTypeDescriptorEnum::PluginType(pt) => Self::PluginType(pt.name),
        }
    }
}
impl From<spec::types::PluginTypeDescriptor> for PluginTypeDescriptorEnum {
    fn from(ptd: spec::types::PluginTypeDescriptor) -> Self {
        Self::from(ptd.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyPattern {
    pub dependencies: Dependency,
    pub typ: PluginTypeEnum,
}
impl From<spec::types::DependencyPattern> for DependencyPattern {
    fn from(dp: spec::types::DependencyPattern) -> Self {
        Self {
            dependencies: Dependency::from(dp.dependencies),
            typ: dp.typ.name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalInstallPattern {
    pub dependencies: Dependency,
    pub files: Vec<FileType>,
}
impl From<crate::spec::types::ConditionalInstallPattern> for ConditionalInstallPattern {
    fn from(spec: crate::spec::types::ConditionalInstallPattern) -> Self {
        Self {
            dependencies: Dependency::from(spec.dependencies),
            files: spec.files.list.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::spec::Config as SpecConfig;
    use crate::{Config, Info};

    #[test]
    pub fn info() {
        let xml = r#"
        <?xml version="1.0"?>
        <fomod xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
          <Name>StarUI Inventory</Name>
          <Version>2.1</Version>
          <Author>m8r98a4f2</Author>
          <Website>https://www.nexusmods.com/starfield/mods/773</Website>
          <CategoryId>37</CategoryId>
        </fomod>
       "#;

        let info: Info = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(info.name, Some("StarUI Inventory".to_string()));
        assert_eq!(info.version, Some("2.1".to_string()));
        assert_eq!(info.author, Some("m8r98a4f2".to_string()));
        assert_eq!(
            info.website,
            Some("https://www.nexusmods.com/starfield/mods/773".to_string())
        );
        assert_eq!(info.category_id, Some(37));
    }

    #[test]
    pub fn required_files() {
        let xml = r#"
        <config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">

            <moduleName>Example Mod</moduleName>

            <requiredInstallFiles>
                <file source="example.plugin"/>
                <file source="example2.plugin"/>
            </requiredInstallFiles>
        </config>
        "#;

        let config: SpecConfig = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(config.module_name, "Example Mod".to_string());

        let file_list = config
            .required_install_files
            .as_ref()
            .unwrap()
            .list
            .as_ref()
            .unwrap();
        assert_eq!(file_list.len(), 2);
        assert_eq!(file_list[0].source, "example.plugin");
        assert_eq!(file_list[1].source, "example2.plugin");

        let config = Config::from(config);
    }

    #[test]
    pub fn module_deps() {
        let xml = r#"
        <config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">

            <moduleName>Example Mod</moduleName>

            <moduleDependencies operator="And">
                <fileDependency file="depend1.plugin" state="Active"/>
            </moduleDependencies>

            <requiredInstallFiles>
                <file source="example.plugin"/>
            </requiredInstallFiles>
        </config>
        "#;

        let config: SpecConfig = quick_xml::de::from_str(&xml).unwrap();

        let config = Config::from(config);
    }

    #[test]
    pub fn module_deps2() {
        let xml = r#"
        <config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">

            <moduleName>Example Mod</moduleName>

            <moduleDependencies operator="And">
                <fileDependency file="depend1.plugin" state="Active"/>
                <dependencies operator="Or">
                    <fileDependency file="depend2v1.plugin" state="Active"/>
                    <fileDependency file="depend2v2.plugin" state="Active"/>
                </dependencies>
            </moduleDependencies>

            <requiredInstallFiles>
                <file source="example.plugin"/>
            </requiredInstallFiles>

        </config>
        "#;

        let config: SpecConfig = quick_xml::de::from_str(&xml).unwrap();

        let config = Config::from(config);
    }

    #[test]
    pub fn install_steps() {
        let xml = r#"
        <config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">

            <moduleName>Example Mod</moduleName>

            <moduleDependencies operator="And">
                <fileDependency file="depend1.plugin" state="Active"/>
                <dependencies operator="Or">
                    <fileDependency file="depend2v1.plugin" state="Active"/>
                    <fileDependency file="depend2v2.plugin" state="Active"/>
                </dependencies>
            </moduleDependencies>

            <installSteps order="Explicit">
                <installStep name="Choose Option">
                    <optionalFileGroups order="Explicit">
                        <group name="Select an option:" type="SelectExactlyOne">
                            <plugins order="Explicit">
                                <plugin name="Option A">
                                    <description>Select this to install Option A!</description>
                                    <image path="fomod/option_a.png"/>
                                    <files>
                                        <folder source="option_a"/>
                                    </files>
                                    <typeDescriptor>
                                        <type name="Recommended"/>
                                    </typeDescriptor>
                                </plugin>
                                <plugin name="Option B">
                                    <description>Select this to install Option B!</description>
                                    <image path="fomod/option_b.png"/>
            						<files />
                                    <typeDescriptor>
                                        <type name="Optional"/>
                                    </typeDescriptor>
                                </plugin>
                            </plugins>
                        </group>
                    </optionalFileGroups>
                </installStep>
            </installSteps>

        </config>
        "#;

        let config: SpecConfig = quick_xml::de::from_str(&xml).unwrap();

        let config = Config::from(config);
    }

    #[test]
    pub fn install_matrix() {
        let xml = r#"
        <config xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:noNamespaceSchemaLocation="http://qconsulting.ca/fo3/ModConfig5.0.xsd">

            <moduleName>Example Mod</moduleName>

            <moduleDependencies operator="And">
                <fileDependency file="depend1.plugin" state="Active"/>
                <dependencies operator="Or">
                    <fileDependency file="depend2v1.plugin" state="Active"/>
                    <fileDependency file="depend2v2.plugin" state="Active"/>
                </dependencies>
            </moduleDependencies>

            <installSteps order="Explicit">
                <installStep name="Choose Option">
                    <optionalFileGroups order="Explicit">

                        <group name="Select an option:" type="SelectExactlyOne">
                            <plugins order="Explicit">

                                <plugin name="Option A">
                                    <description>Select this to install Option A!</description>
                                    <image path="fomod/option_a.png"/>
                                    <conditionFlags>
                                        <flag name="option_a">selected</flag>
                                    </conditionFlags>
                                    <typeDescriptor>
                                        <type name="Recommended"/>
                                    </typeDescriptor>
                                </plugin>

                                <plugin name="Option B">
                                    <description>Select this to install Option B!</description>
                                    <image path="fomod/option_b.png"/>
                                    <conditionFlags>
                                        <flag name="option_b">selected</flag>
                                    </conditionFlags>
                                    <typeDescriptor>
                                        <type name="Optional"/>
                                    </typeDescriptor>
                                </plugin>

                            </plugins>
                        </group>

                        <group name="Select a texture:" type="SelectExactlyOne">
                            <plugins order="Explicit">

                                <plugin name="Texture Blue">
                                    <description>Select this to install Texture Blue!</description>
                                    <image path="fomod/texture_blue.png"/>
                                    <conditionFlags>
                                        <flag name="texture_blue">selected</flag>
                                    </conditionFlags>
                                    <typeDescriptor>
                                        <type name="Optional"/>
                                    </typeDescriptor>
                                </plugin>

                                <plugin name="Texture Red">
                                    <description>Select this to install Texture Red!</description>
                                    <image path="fomod/texture_red.png"/>
                                    <conditionFlags>
                                        <flag name="texture_red">selected</flag>
                                    </conditionFlags>
                                    <typeDescriptor>
                                        <type name="Optional"/>
                                    </typeDescriptor>
                                </plugin>

                            </plugins>
                        </group>

                    </optionalFileGroups>
                </installStep>
            </installSteps>

            <conditionalFileInstalls>
                <patterns>
                    <pattern>
                        <dependencies operator="And">
                            <flagDependency flag="option_a" value="selected"/>
                            <flagDependency flag="texture_blue" value="selected"/>
                        </dependencies>
                        <files>
                            <folder source="option_a"/>
                            <folder source="texture_blue_a"/>
                        </files>
                    </pattern>
                    <pattern>
                        <dependencies operator="And">
                            <flagDependency flag="option_a" value="selected"/>
                            <flagDependency flag="texture_red" value="selected"/>
                        </dependencies>
                        <files>
                            <folder source="option_a"/>
                            <folder source="texture_red_a"/>
                        </files>
                    </pattern>
                    <pattern>
                        <dependencies operator="And">
                            <flagDependency flag="option_b" value="selected"/>
                            <flagDependency flag="texture_blue" value="selected"/>
                        </dependencies>
                        <files>
                            <folder source="option_b"/>
                            <folder source="texture_blue_b"/>
                        </files>
                    </pattern>
                    <pattern>
                        <dependencies operator="And">
                            <flagDependency flag="option_b" value="selected"/>
                            <flagDependency flag="texture_red" value="selected"/>
                        </dependencies>
                        <files>
                            <folder source="option_b"/>
                            <folder source="texture_red_b"/>
                        </files>
                    </pattern>
                </patterns>
            </conditionalFileInstalls>

        </config>
        "#;

        let config: SpecConfig = quick_xml::de::from_str(&xml).unwrap();

        let config = Config::from(config);
    }
}
