use std::{fs::File, io::Read};

use fomod::{spec::Config as SpecConfig, Config, Info};

const FOMOD_INFO_PATH: &'static str = "./examples/fomod/info.xml";
const FOMOD_MODULECONFIG_PATH: &'static str = "./examples/fomod/ModuleConfig.xml";

fn main() {
    let info = {
        let mut file = File::open(FOMOD_INFO_PATH).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
    let config = {
        let mut file = File::open(FOMOD_MODULECONFIG_PATH).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    let info: Info = quick_xml::de::from_str(&info).unwrap();
    dbg!(&info);

    let config: SpecConfig = quick_xml::de::from_str(&config).unwrap();
    dbg!(&config);

    let config: Config = Config::from(config);
    dbg!(&config);
}
