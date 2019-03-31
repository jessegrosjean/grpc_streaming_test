mod api;

use api::api::StreamRequest;
use api::api_grpc::StreamingExampleServiceClient;
use futures::future::Future;
use futures::stream::Stream;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use std::time::Duration;

fn main() {
  let env = Arc::new(EnvBuilder::new().cq_count(1).build());
  let channel = ChannelBuilder::new(env).connect("127.0.0.1:50051");
  let client = StreamingExampleServiceClient::new(channel);
  let items_stream = client.stream_case_2(&StreamRequest::new()).unwrap();
  let f = items_stream.take(100).for_each(|item| {
    println!("received  {}", item.get_item_id());
    Ok(())
  });
  f.wait().unwrap();
  println!("\n\nClient has received all items that it wants. It will now sleep for 10 seconds. The server should stop generating/sending items soon!\n\n");
  std::thread::sleep(Duration::from_millis(10000));
}
