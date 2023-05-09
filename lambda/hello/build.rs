use std::{io::Result, fs};

fn main() -> Result<()> {
    println!("Hello, world!");
    let out_dir = "src/gen_proto";
    fs::create_dir_all(out_dir)?;
    protobuf_codegen::Codegen::new()
        .out_dir(out_dir)
        .include("proto")
        .input("proto/hello.proto")
        .run()
        .expect("Unable to generate hello.rs");
    Ok(())
}