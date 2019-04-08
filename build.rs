use std::fs::{create_dir_all, File};
use std::io::Write;

use failure::{Error, ResultExt};

pub(crate) const PROTOBUF_DIR: &str = "protobuf";
pub(crate) const PROTOBUF_OUTPUT_DIR: &str = "src/protobuf";
pub(crate) const PROTOBUF_MODULES: [&str; 1] = ["pulsar_api"];

pub(crate) const PULSAR_PROTOBUF_FILE: &str = "pulsar_api.proto";
pub(crate) const PULSAR_PROTOBUF_VERSION: &str = "v2.3.1-candidate-1";

fn main() -> Result<(), Error> {
    create_dir_all(PROTOBUF_DIR)
        .with_context(|err| format!("could not create the folder '{}', {}", PROTOBUF_DIR, err))?;
    create_dir_all(PROTOBUF_OUTPUT_DIR)
        .with_context(|err| format!("could not create the folder '{}', {}", PROTOBUF_OUTPUT_DIR, err))?;

    let url = format!("https://raw.githubusercontent.com/apache/pulsar/{}/pulsar-common/src/main/proto/PulsarApi.proto", PULSAR_PROTOBUF_VERSION);
    let protobuf = reqwest::get(&url)
        .with_context(|err| format!("could not retrieve the protobuf at this url '{}', {}", url, err))?
        .text()
        .with_context(|err| format!("could not parse as text the protobuf '{}', {}", url, err))?;

    let file_name = format!("{}/{}", PROTOBUF_DIR, PULSAR_PROTOBUF_FILE);
    let mut file = File::create(&file_name)
        .with_context(|err| format!("could not create the file '{}', {}", file_name, err))?;

    file.write(protobuf.as_bytes())
        .with_context(|err| format!("could write the protobuf to the file '{}', {}", file_name, err))?;
    file.sync_all()
        .with_context(|err| format!("could not sync the file '{}', {}", file_name, err))?;

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: PROTOBUF_OUTPUT_DIR,
        input: &[&file_name],
        includes: &[PROTOBUF_DIR],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    })
        .with_context(|err| format!("could not generate the protobuf, {}", err))?;

    let file_name_mod = format!("{}/mod.rs", PROTOBUF_OUTPUT_DIR);
    let mut file_mod = File::create(&file_name_mod)
        .with_context(|err| format!("could not create the file '{}/mod.rs', {}", PROTOBUF_OUTPUT_DIR, err))?;
    for module in &PROTOBUF_MODULES {
        file_mod.write(format!("pub(crate) mod {};", module).as_bytes())
            .with_context(|err| format!("could not write the export of the module '{}' into '{}', {}", module, file_name_mod, err))?;
    }

    file_mod.sync_all()
        .with_context(|err| format!("could not sync the file '{}', {}", file_name_mod, err))?;

    Ok(())
}
