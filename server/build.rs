fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile(&["../proto/message.proto"], &["../proto"])
        .unwrap();
}
