# BQSP
Box Queue Streaming Protocol

## About
BQSP is a [presentation layer](https://en.wikipedia.org/wiki/Presentation_layer) protocol used to 
transport data in a fast and efficient manner through any stream of bytes.

The main aspect is the use of Queues. Each payload can be assigned a specific Queue number which then can be handled
by the application in parallel.

For example if Client sends 2 requests at the same time each with a different Queue number, the Server could start 
executing them in parallel. Even if the request that came second was handled first, the server can send a response with
the same Queue number, so the Client can know which response should be associated with which request.

BQSP was created as a part of the "[Hangin!](https://github.com/prty-app)" app for lesser network usage and smoother user experience.

## Packet
In the terms of BQSP "Packet" is a full structure containing both Header and Payload/Data. 
```
<PACKET> = [HEADER] + [DATA]
```

## Header
This structure contains all the metadata needed to handle the Payload/Data.

Header consists of:
- `Data Size` _(4 bytes)_ **-** size of the Payload/Data
- `Data Type` _(2 bytes)_ **-** type of the Payload/Data
- `Queue` _(1 byte)_ **-** the queue number

Header is represented in LittleEndian, so an example Header would look like this:
```
Data Size:  16     (0x10)    [10, 0, 0, 0]
Data Type:  43775  (0xAAFF)  [FF, AA]
Queue:      1      (0x1)     [1]
-------------------------------------------
[10, 0, 0, 0, FF, AA, 1]
```

## Payload/Data
After the header any data is accepted. 

In order for the second end to know what type of data was sent it's a good idea to use the Data Type attribute in a
Header of the Packet.

For example if the following JSON is the Payload/Data:
```json
{"status":"ok"}
```

And the enum to represent Data Type is as follows:
```rust
#[repr(u16)]
enum MyDataType {
    Unknown = 0,
    Text = 1,
    JSON = 2,
}
```

The whole packet would look like this:
```
Data Size:  15  (0xF)  [F, 0, 0, 0]
Data Type:  2   (0x2)  [2, 0]
Queue:      1   (0x1)  [1]
-------------------------------------------
[F, 0, 0, 0, 2, 0, 1, 7B, 22, 73, 74, 61, 74, 75, 73, 22, 3A, 22, 6F, 6B, 22, 7D]
```

## Current State
BQSP is production ready, but the crate itself lacks many features, optimizations and proper documentation.

Handling of the Queues and Data Types are decided by the system implementing the protocol and the needs of the application.
BQSP is a way to send and receive data, not to handle or qualify it.

The maximum length of Payload/Data in one Packet is `4_294_967_295 Bytes` which is roughly `4.29 GB`, 
but sending such high amount of data at once is not recommended. It's better to break it into smaller Packets.

## License
 <p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><span property="dct:title">Box Queue Streaming Protocol</span> is licensed under <a href="http://creativecommons.org/licenses/by/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC BY 4.0</a></p> 
