mod test;
mod test_grpc;

use futures::future::Future;
use futures::stream::Stream;
use futures::sync::mpsc::*;
use futures::sync::oneshot;
use futures::*;
use grpcio::{EnvBuilder, RpcContext, ServerBuilder, ServerStreamingSink, WriteFlags};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};
use test::{StreamItem, StreamRequest};

#[derive(Clone)]
pub struct StreamService {}

fn method_1(ctx: RpcContext, req: StreamRequest, sink: ServerStreamingSink<StreamItem>) {
  let client_id = req.get_client_id();
  let (mut sender, receiver) = channel(10);
  let receiver = receiver
    .map(|e| (e, WriteFlags::default()))
    .map_err(|_| grpcio::Error::RemoteStopped);

  ctx.spawn(
    sink
      .send_all(receiver)
      .map(|_| {})
      .map_err(|e| println!("failed to reply: {:?}", e)),
  );

  thread::spawn(move || {
    let mut id = 0;
    loop {
      let mut item = StreamItem::new();
      item.set_item_id(id);
      println!("sending to client {}-{:?}", client_id, item);
      id += 1;
      match sender.try_send(item) {
        Ok(()) => (),
        Err(e) => {
          if e.is_disconnected() {
            return;
          }
          if e.is_full() {
          } else {
            panic!("What case is this?")
          }
        }
      }
    }
  });
}

fn method_2(ctx: RpcContext, req: StreamRequest, sink: ServerStreamingSink<StreamItem>) {
  use futures::future::Future;
  use futures::*;
  use grpcio::{Error, WriteFlags};

  let client_id = req.get_client_id();
  struct ForeverIter {
    client_id: u64,
    item_id: u64,
  };
  impl Iterator for ForeverIter {
    type Item = StreamItem;
    fn next(&mut self) -> Option<Self::Item> {
      let mut item = StreamItem::new();
      item.set_item_id(self.item_id);
      println!("creating to client {}-{:?}", self.client_id, item);
      self.item_id += 1;
      Some(item)
    }
  }

  let forever_iter = ForeverIter {
    client_id,
    item_id: 0,
  };
  let iter = forever_iter.map(|e| (e, WriteFlags::default()));
  let f = sink
    .send_all(stream::iter_ok::<_, Error>(iter))
    .map(|_| {})
    .map_err(|e| println!("failed to handle stream_test: {:?}", e));
  ctx.spawn(f)
}

impl test_grpc::StreamService for StreamService {
  fn stream(&mut self, ctx: RpcContext, req: StreamRequest, sink: ServerStreamingSink<StreamItem>) {
    // With this method of streaming when I close the stream on the client then
    // the ForeverIter iterator immeditaly stops being asked to yeiled items.
    // This is the expected behavior.

    //method_1(ctx, req, sink)

    // With this method of streaming when I close the stream on the client the
    // ForeverIter iterator keeps getting asked to yeiled items. This continues
    // until the client process exits.

    method_2(ctx, req, sink)
  }
}

fn main() {
  let env = Arc::new(EnvBuilder::new().cq_count(1).build());
  let instance = StreamService {};
  let service = test_grpc::create_stream_service(instance);
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
