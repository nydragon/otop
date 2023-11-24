# OTOP - Gateway

## Gateway Events


Payload Structure
---
Gateway event payloads have a common structure, but the contents of the associated data (d) varies between the different events.

| Field | Type                    | Description                                                          |
| ----- | ----------------------- | -------------------------------------------------------------------- |
| op    | integer                 | Gateway opcode, which indicates the payload type                     |
| d     | ?mixed(any JSON value)* | Event data                                                           |
| s     | ?integer*               | Sequence number of event used for resuming sessions and heartbeating |
| t     | ?string*                | Event name                                                           |

> * s and t are null when op is not 0 (Gateway Dispatch opcode).

Example:
```json
{
  "op": 0,
  "d": {},
  "s": 42,
  "t": "GATEWAY_EVENT_NAME"
}
```

## Opcodes and Status Code
---
All gateway events in Otop are tagged with an opcode that denotes the payload type. Your connection to our gateway may also sometimes close. When it does, you will receive a close code that tells you what happened.

### Gateway Opcodes

| Code | Name          | Client Action | Description                                                                        |
| ---- | ------------- | ------------- | ---------------------------------------------------------------------------------- |
| 0    | Dispatch      | Receive       | An event was dispatched                                                            |
| 1    | Heartbeat     | Send/Receive  | Fired periodically by the client to keep the connection alive                      |
| 3    | Ask Data      | Send          | Ask for data                                                                       |
| 10   | Hello         | Receive       | Sent immediately after connecting, contains the `heartbeat_interval` to use        |
| 11   | Heartbeat ACK | Receive       | Sent in response to receiving a heartbeat to acknowledge that it has been received |

### Gateway Close Event Codes
---
In order to prevent broken reconnect loops, you should consider some close codes as a signal to stop reconnecting. This table explains what the application defined close codes for the gateway are.

| Code | Description         | Explanation                                                                                                      | Reconnect |
| ---- | ------------------- | ---------------------------------------------------------------------------------------------------------------- | --------- |
| 4000 | Unknown error       | We're not sure what went wrong. Try reconnecting?                                                                | true      |
| 4001 | Unknown opcode      | You sent an invalid Gateway opcode or an invalid payload for an opcode. Don't do that!                           | true      |
| 4002 | Decode error        | You sent an invalid payload to Otop. Don't do that!                                                              | true      |
| 4008 | Rate limit exceeded | Woah nelly! You're sending payloads to us too quickly. Slow it down! You will be disconnected on receiving this. | true      |
| 4012 | Invalid api version | You sent an invalid version for the gateway.                                                                     | false     |

### HTTP
---
Our API will return semantically valid HTTP response codes based on the success of your request. The following table can be used as a reference for response codes it will return.

| Code                      | Meaning                                                                          |
| ------------------------- | -------------------------------------------------------------------------------- |
| 200 (OK)                  | The request completed successfully.                                              |
| 201 (CREATED)             | The entity was successfully created.                                             |
| 204 (NO CONTENT)          | The request completed successfully but returned no content.                      |
| 304 (NOT MODIFIED)        | The entity was not modified (no action was taken).                               |
| 400 (BAD REQUEST)         | The request was improperly formatted, or the server couldn't understand it.      |
| 404 (NOT FOUND)           | The resource at the location specified doesn't exist.                            |
| 405 (METHOD NOT ALLOWED)  | The HTTP method used is not valid for the location specified.                    |
| 429 (TOO MANY REQUESTS)   | You are being rate limited, see Rate Limits.                                     |
| 502 (GATEWAY UNAVAILABLE) | There was not a gateway available to process your request. Wait a bit and retry. |
| 5xx (SERVER ERROR)        | The server had an error processing your request (these are rare).                |