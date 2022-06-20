fn main() {
    let files = &["../proto/v1/common.proto", "../proto/v1/protocol.proto"];

    let dirs = &["../proto"];

    tonic_build::configure()
        .build_server(true)
        .compile(files, dirs)
        .unwrap();
}
