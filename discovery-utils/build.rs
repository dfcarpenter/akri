fn main() {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("./src/discovery")
        .compile(&["proto/discovery.proto"], &["proto"])
        .expect("failed to compile protos");
}
