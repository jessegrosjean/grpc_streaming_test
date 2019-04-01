mod api;
mod log_util;

use api::api::{StreamItem, StreamRequest};
use api::api_grpc;
use futures::future::Future;
use futures::stream::Stream;
use futures::sync::mpsc::*;
use futures::sync::oneshot;
use futures::*;
use grpcio::{
  Environment, Error, RpcContext, RpcStatus, ServerBuilder, ServerStreamingSink, WriteFlags,
};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

#[derive(Clone)]
pub struct StreamingExampleService {}

impl api::api_grpc::StreamingExampleService for StreamingExampleService {
  fn stream_case_1(
    &mut self,
    ctx: RpcContext,
    _req: StreamRequest,
    sink: ServerStreamingSink<StreamItem>,
  ) {
    let stream = ItemStream::new()
      .map(|item| (item, WriteFlags::default()))
      .map_err(|_| Error::RpcFailure(RpcStatus::ok()));
    ctx.spawn(
      sink
        .send_all(stream)
        .map(|_| println!("completed"))
        .map_err(|e| println!("failed to reply: {:?}", e)),
    );
  }

  fn stream_case_2(
    &mut self,
    ctx: RpcContext,
    _req: StreamRequest,
    sink: ServerStreamingSink<StreamItem>,
  ) {
    let (mut sender, receiver) = channel(10);
    let receiver = receiver
      .map(|e| (e, WriteFlags::default()))
      .map_err(|_| grpcio::Error::RemoteStopped);

    thread::spawn(move || {
      let mut id = 0;
      loop {
        let mut item = StreamItem::new();
        item.set_item_id(id);
        match sender.try_send(item) {
          Ok(()) => {
            println!("sent {}", id);
            id += 1;
          }
          Err(e) => {
            if e.is_disconnected() {
              println!("is_disconnected");
              return;
            } else if e.is_full() {
              // If I comment out this println then I get the bad behavior
              // again. The server keeps sending stream even after client is
              // done reading it. On the other hand when I DO enable this
              // println then the server stops sending soon after the client is
              // done. I'm not doing something right for sure!
              println!("is_full");
            } else {
              panic!("unexpected case!")
            }
          }
        }
      }
    });

    ctx.spawn(
      sink
        .send_all(receiver)
        .map(|_| println!("completed"))
        .map_err(|e| println!("failed to reply: {:?}", e)),
    );
  }
}

pub struct ItemStream {
  curr: u64,
}

impl ItemStream {
  fn new() -> ItemStream {
    ItemStream { curr: 1 }
  }
}

impl Stream for ItemStream {
  type Item = StreamItem;
  type Error = ();

  fn poll(&mut self) -> Poll<Option<StreamItem>, ()> {
    println!("generating {}", self.curr);
    let mut item = StreamItem::new();
    item.set_item_id(self.curr);
    self.curr += 1;
    Ok(Async::Ready(Some(item)))
  }
}

fn main() {
  env_logger::init();
  grpcio::redirect_log();

  let env = Arc::new(Environment::new(2));
  let instance = StreamingExampleService {};
  let service = api_grpc::create_streaming_example_service(instance);
  let mut server = ServerBuilder::new(env)
    .register_service(service)
    .bind("127.0.0.1", 50_051)
    .build()
    .unwrap();
  server.start();
  for &(ref host, port) in server.bind_addrs() {
    println!("listening on {}:{}", host, port);
  }
  let (tx, rx) = oneshot::channel();
  thread::spawn(move || {
    println!("Press ENTER to exit...");
    let _ = io::stdin().read(&mut [0]).unwrap();
    tx.send(())
  });
  let _ = rx.wait();
  let _ = server.shutdown().wait();
}
