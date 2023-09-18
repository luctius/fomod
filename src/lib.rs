pub mod spec;

#[cfg(test)]
mod tests {
    use crate::spec::files::{Config, Info};

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

        let config: Config = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(config.module_name, "Example Mod".to_string());

        let file_list = config.required_install_files.unwrap().list.unwrap();
        assert_eq!(file_list.len(), 2);
        assert_eq!(file_list[0].source, "example.plugin");
        assert_eq!(file_list[1].source, "example2.plugin");
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

        let config: Config = quick_xml::de::from_str(&xml).unwrap();
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

        let config: Config = quick_xml::de::from_str(&xml).unwrap();
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

        let config: Config = quick_xml::de::from_str(&xml).unwrap();
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

        let config: Config = quick_xml::de::from_str(&xml).unwrap();
    }
}
