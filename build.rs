fn main() {
  let protos = &[
    "src/api/proto/hello.proto",
  ];
  let includes = &["."];

  tonic_build::configure()
    .build_client(true)
    .build_server(true)
    .compile(protos, includes)
    .unwrap()
}
