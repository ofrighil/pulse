use std::io::Result;

use prost_build;

fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/gtfs-realtime.proto"], &["protos/"])?;
    Ok(())
}
