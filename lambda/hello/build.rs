use std::{io::Result, fs};

fn main() -> Result<()> {
    println!("build proto code");
    // 生成先のディレクトリを作成
    let output_dir = "src/gen_proto";
    // protoファイルが入ってるディレクトリを指定
    let proto_input_dir = "proto";
    fs::create_dir_all(output_dir)?;
    protobuf_codegen::Codegen::new()
        .out_dir(output_dir)
        .include(proto_input_dir)
        // 生成するprotoファイルを指定
        .input("proto/hello_req.proto")
        .input("proto/hello_res.proto")
        .run()
        .expect("Unable to generate hello.rs");
    Ok(())
}