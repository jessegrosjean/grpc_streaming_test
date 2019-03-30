// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_STREAM_SERVICE_STREAM: ::grpcio::Method<super::test::StreamRequest, super::test::StreamItem> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/StreamService/stream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StreamServiceClient {
    client: ::grpcio::Client,
}

impl StreamServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StreamServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_opt(&self, req: &super::test::StreamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::test::StreamItem>> {
        self.client.server_streaming(&METHOD_STREAM_SERVICE_STREAM, req, opt)
    }

    pub fn stream(&self, req: &super::test::StreamRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::test::StreamItem>> {
        self.stream_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait StreamService {
    fn stream(&mut self, ctx: ::grpcio::RpcContext, req: super::test::StreamRequest, sink: ::grpcio::ServerStreamingSink<super::test::StreamItem>);
}

pub fn create_stream_service<S: StreamService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_STREAM_SERVICE_STREAM, move |ctx, req, resp| {
        instance.stream(ctx, req, resp)
    });
    builder.build()
}
