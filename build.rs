fn main() {
    tonic_build::configure()
        .out_dir("./src/pb/")
        .file_descriptor_set_path("./src/pb/services_descriptor.bin")
        // .type_attribute("hello.HelloResponse","#[derive(serde::Serialize,serde::Deserialize)]")
        // .type_attribute("hello.HelloRequest","#[derive(serde::Serialize,serde::Deserialize)]")
        .compile(&["./proto/message.proto","./proto/services.proto",],&["proto"])
        .unwrap();

    // tonic_build::configure()
    //     .out_dir("./src/pb")
    //     .file_descriptor_set_path("./src/pb/hello_descriptor.bin")
    //     // .type_attribute("hello.HelloResponse","#[derive(serde::Serialize,serde::Deserialize)]")
    //     // .type_attribute("hello.HelloRequest","#[derive(serde::Serialize,serde::Deserialize)]")
    //     .compile(&["./proto/hello.proto"],&["proto"])
    //     .unwrap();
}