use protoc_grpcio;
use std::env;

fn main() {
	protoc_grpcio::compile_grpc_protos(
		&["proto/test.proto"],
		&["proto"],
		&"src",
		None,
	)
	.expect("Failed to compile gRPC definitions!");
}
