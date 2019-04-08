use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;

pub(crate) const PROTOBUF_DIR: &str = "protobuf";
pub(crate) const PROTOBUF_OUTPUT_DIR: &str = "src/protobuf";
pub(crate) const PROTOBUF_MODULES: [&str; 1] = ["pulsar_api"];

pub(crate) const PULSAR_PROTOBUF_FILE: &str = "pulsar_api.proto";
pub(crate) const PULSAR_PROTOBUF_VERSION: &str = "v2.3.1-candidate-1";

fn main() -> Result<(), Box<Error>> {
    create_dir_all(PROTOBUF_DIR)?;
    create_dir_all(PROTOBUF_OUTPUT_DIR)?;

    let url = format!("https://raw.githubusercontent.com/apache/pulsar/{}/pulsar-common/src/main/proto/PulsarApi.proto", PULSAR_PROTOBUF_VERSION);
    let protobuf = reqwest::get(&url)?.text()?;

    let file_name = format!("{}/{}", PROTOBUF_DIR, PULSAR_PROTOBUF_FILE);
    let mut file = File::create(&file_name)?;

    file.write(protobuf.as_bytes())?;
    file.sync_all()?;

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: PROTOBUF_OUTPUT_DIR,
        input: &[&file_name],
        includes: &[PROTOBUF_DIR],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    })?;

    let mut file_mod = File::create(format!("{}/mod.rs", PROTOBUF_OUTPUT_DIR))?;
    for modules in &PROTOBUF_MODULES {
        file_mod.write(format!("pub(crate) mod {};", modules).as_bytes())?;
    }

    file_mod.sync_all()?;
    Ok(())
}
