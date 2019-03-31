This repository contains a [grpc-rs](https://github.com/pingcap/grpc-rs) server
and two clients. To build:

```
cargo build
```

## Server

The server implements two streaming rpcs (`stream_case_1` and `stream_case_2`).
They are two different ways to implement an infinite stream of results.

The intention is that the server continues pushing items into the stream as long
as the client is listening. When the client hangs up the server should stop
generating items and sending them to the stream.

Start the server:

```
./target/debug/server
```

The server will log each item generated.

## Clients

The two clients (`client_case_1` and `client_case_2`) are the same, the only
difference being that `client_case_1` calls the `stream_case_1` rpc while
`client_case_2` calls the `stream_case_2` rpc.

Each client reads the first 100 items from the stream and then sleeps for 10
seconds before quitting. The intention is that the server stops sending items
soon after the client has received it's 100 items. The reason for the 10 second
sleep when the server actually stops sending items... the server seems to always
stop "right away" if the client process exists.

To run a client 1:

```
./target/debug/client_case_1
```

To run a client 2:

```
./target/debug/client_case_2
```

Each client logs each item received. 

## Case 1 Problems

Case 1 is the biggest problem case. Notice that after client 1 receives the
first 100 items and sleeps the server often keep generating items all the way
until until the client process exits. So for example on my computer the server
ends up working to generate over 200,000 items each time.

Note that this isn't always the case. Sometimes (especially just after
restarting the server) the server will stop generating items pretty quickly
after the client has it's first 100. That that isn't the normal behavior that I
see.

## Case 2 Problems

Case 2 behaves better. The server always stops generating items soon after the
client has received the first 100.

But to get this good behavior required a rather messy implementation on the
server. I need to setup a channel and then use a separate thread to call
`try_send` on it to send the items. Probably this can be improved?

A second problem is that the server still generates more items then I would
expect. Around 1000 for each call. This is surprising to me because the the
channel that I'm writing into only has a buffer of 10 items. I don't quite
understand why the server is still generating so many.