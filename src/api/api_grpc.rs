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

const METHOD_STREAMING_EXAMPLE_SERVICE_STREAM_CASE_1: ::grpcio::Method<super::api::StreamRequest, super::api::StreamItem> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/StreamingExampleService/stream_case_1",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STREAMING_EXAMPLE_SERVICE_STREAM_CASE_2: ::grpcio::Method<super::api::StreamRequest, super::api::StreamItem> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/StreamingExampleService/stream_case_2",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StreamingExampleServiceClient {
    client: ::grpcio::Client,
}

impl StreamingExampleServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StreamingExampleServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_case_1_opt(&self, req: &super::api::StreamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::api::StreamItem>> {
        self.client.server_streaming(&METHOD_STREAMING_EXAMPLE_SERVICE_STREAM_CASE_1, req, opt)
    }

    pub fn stream_case_1(&self, req: &super::api::StreamRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::api::StreamItem>> {
        self.stream_case_1_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stream_case_2_opt(&self, req: &super::api::StreamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::api::StreamItem>> {
        self.client.server_streaming(&METHOD_STREAMING_EXAMPLE_SERVICE_STREAM_CASE_2, req, opt)
    }

    pub fn stream_case_2(&self, req: &super::api::StreamRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::api::StreamItem>> {
        self.stream_case_2_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait StreamingExampleService {
    fn stream_case_1(&mut self, ctx: ::grpcio::RpcContext, req: super::api::StreamRequest, sink: ::grpcio::ServerStreamingSink<super::api::StreamItem>);
    fn stream_case_2(&mut self, ctx: ::grpcio::RpcContext, req: super::api::StreamRequest, sink: ::grpcio::ServerStreamingSink<super::api::StreamItem>);
}

pub fn create_streaming_example_service<S: StreamingExampleService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_STREAMING_EXAMPLE_SERVICE_STREAM_CASE_1, move |ctx, req, resp| {
        instance.stream_case_1(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_STREAMING_EXAMPLE_SERVICE_STREAM_CASE_2, move |ctx, req, resp| {
        instance.stream_case_2(ctx, req, resp)
    });
    builder.build()
}
