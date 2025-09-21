fn main() {
    tonic_build::configure()
        .build_server(true)
        .type_attribute("EncryptedMessage", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["../proto/message.proto"], &["../proto"])
        .unwrap();
}
