# Crikey IRC

An IRC library implemented in Rust. This is a learning project for me, so the
priority is correctly implementing the IRC protocol in idiomatic Rust.

The ultimate plan for this project is to split it into multiple crates as
follows:

* crikey-irc-common: A library providing protocol and state tracking services,
  to be consumed by both the client and server
* crikey-irc-client: A library for writing IRC clients, bots, and services.
  * crikey-irc-bot: Basically just an example script for crikey-irc-client.
* crikey-irc-server: A library + binary implementing the IRC server protocol.

## Getting started (for users)

There's really nothing to use, yet. Sorry.

## Getting started (for developers)

To get up and running with a local environment, you will need
[Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/) (if it's not already
provided with your Docker install). You do not need Rust installed on your host
machine, though it's obviously useful if you intend to write code.

Once Docker is set up, you can run crikey-irc using:

    docker-compose build
    docker-compose run --rm crikey-irc

The "command line" provided by crikey-irc allows you to send commands directly
to the IRC server using the client's connection. To quit the client, close the
connection by typing `QUIT`.

The IRC server will continue running in the background after the client has been
shut down. This is a limitation with Docker Compose. It can be shut down using

    docker-compose down

## Current state of development

The bot opens a TCP connection to an IRC server and performs the authentication
handshake (NICK/USER). Incoming messages are parsed and output to the terminal
using the various structs and enums defined by the connection. The user can type
additional raw commands at the command line, which will be transmitted to the
server verbatim. The client automatically responds to PING messages received
from the server.

Almost all documented client commands have been implemented in the parsing
library, and the examples from RFC 2812 have all been implemented as unit tests.

## Modules

### connection

**State of development: done, some polish needed**

Handles the basic reading and writing of strings to/from the TCP connection with
the IRC server. The IRC protocol consists of
[messages](https://tools.ietf.org/html/rfc2812#section-2.3) and
[replies](https://tools.ietf.org/html/rfc2812#section-2.4), which are
respectively converted by the `Connection` into `Command` and `ReplyType` enums.

### state

**State of development: not started (issue #21)**

Persists the known state of the IRC network. This includes known users,
channels, etc.

### client

**State of development: started (issue #3)**

The `Client` applies a semantic layer on top of the `Connection`. While the
`Connection` sends and receives commands, it makes no attempt to understand
their meaning. The `Client` will expose methods that represent actual IRC
actions, such as `send_message()` or `join_channel()`. It will also
handle replying to `PING` messages received from the server.

It may also be the responsibility of the `Client` to maintain a persistent set
of value objects for things like Channels and Users.

### bot

**State of development: not started (issue #4)**

The `Bot` will interact with the `Client` to define automated behaviour flows,
such as responding to user messages or watching for particular keywords in a
channel.

### terminal

**State of development: done**

The Terminal provides an asynchronous interface for the command line for testing
purposes. It allows raw IRC commands to be typed directly at the command line
and sent to the server.
