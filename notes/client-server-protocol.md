# Note: Client-Server Protocol

A client can be either an individual or a server. Unlike SMTP, a client does not need
an intermediarry server to perform the operation for them, and can instead connect
directly. This is dependendant either way of the client being able to properly proove
their identity (the identity of the sender).

1. When a server first receives a connection, it should immediately accept and write
to the socket the "service advertisement". This advertisement can/should contain:
  - Network identity, such as the hostname.
  - Current Muninn version if applicable (dependent on protocol finalization).
  - Requirements of the service.
  - Additional features of the service.

2. The client is now allowed to respond, by posting the headers for their message-sending
request. This header should include:
  - Client identity, such as the hostname, or application name.
  - Current Muninn version if applicable (dependent on protocol finalization).
  - Recipient address (who the client is sending to).
  - Sender address and identity.
  - Payload description (mime types used, total message size).
  - Additional feature fields (hascash headers, delivery hooks, active read-receipt hooks, etc.)

3. The server processes and caches this information. If the provided information is satisfactory
for message transmission it responds with an OK-CONTINUE response. Otherwise additional information
may be requested, or a general failure code may be responded to drop the connection.

4. If the server accepted the request, the client may now respond with the payload.

5. After receiving the payload, the server can respond with a final OK and any closing data that
pertains to the message sent. Such as message ID, accepted hooks, SHA-1 checksums for the payload
read, etc.. After which, the server will terminate the connection.

6. Client terminates the connection after reading the final message from server.
