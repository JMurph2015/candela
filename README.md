# Candela
A straightforward communication framework for controlling LED lights.

## Development Warning
This crate is under active development; interfaces may change substantially.

You've been warned.

## Startup Protocol

### Client / Controller
* Listen for a broadcast packet advertising a server startup.
* Respond to the sender of the broadcast packet with controller info.
    * Broadcast packet internally specifies where to respond.
    * Locks controller until either the server responds or a timeout which
    recycles this controller to the start of the connection protocol.
* Wait for a \{connection type\} connection from the server.
* Process messages as they arrive from the server.

### Server
* Send out a broadcast message to discover clients
* Listen for a fixed amount of time for responses
* Parse the responses.
* Decide to which clients to connect.
* Initiate connections to clients.
* Send messages to clients via those connections.