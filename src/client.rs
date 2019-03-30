mod test;
mod test_grpc;

use futures::future::Future;
use futures::stream::Stream;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use std::time::Duration;
use test::StreamRequest;
use test_grpc::StreamServiceClient;

fn main() {
  let env = Arc::new(EnvBuilder::new().cq_count(1).build());
  let channel = ChannelBuilder::new(env).connect("127.0.0.1:50051");
  let client = StreamServiceClient::new(channel);
  let mut req = StreamRequest::new();
  req.set_client_id(0);
  let mut items_stream = client.stream(&req).unwrap();
  let mut count = 0;
  loop {
    let f = items_stream.into_future();
    match f.wait() {
      Ok((Some(item), s)) => {
        items_stream = s;
        println!("client got item:  {:?}", item);
        count += 1;
        if count > 100 {
          items_stream.cancel();
          drop(items_stream);
          break;
        }
      }
      Ok((None, _)) => break,
      Err((e, _)) => panic!("List features failed: {:?}", e),
    }
  }

  // At this point the stream is closed and dropped I think. If the server is
  // using method_1 to fill the stream then the sever stops doing work at this
  // point. But if the server is using method_2 it continues process while the
  // client sleeps. Once this client process exits, then the server stops
  // procesing even when using method_2.
  println!("server should stop processing request now!?");
  std::thread::sleep(Duration::from_millis(10000));
}
