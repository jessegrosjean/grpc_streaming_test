use protoc_grpcio;
use std::env;

fn main() {
	protoc_grpcio::compile_grpc_protos(
		&["proto/api.proto"],
		&["proto"],
		&"src/api",
		None,
	)
	.expect("Failed to compile gRPC definitions!");
}
