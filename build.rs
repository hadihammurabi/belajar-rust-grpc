fn main() {
  let protos = &[
    "src/api/proto/hello.proto",
    "src/api/proto/calculator.proto",
  ];
  let includes = &["."];
  let out_dir = "src/api";

  tonic_build::configure()
    .build_client(true)
    .build_server(true)
    .out_dir(out_dir)
    .compile(protos, includes)
    .unwrap()
}
