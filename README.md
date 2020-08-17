# IRustC Bot

An IRC bot implemented in Rust. This is a learning project for me, so the
priority is correctly implementing the IRC protocol in idiomatic Rust. Maybe
once that's done I'll want to continue development into a fully-functional bot,
or make this into one or more Cargo crates.

## Modules

### connection.rs

**State of development: done**

Handles the basic reading and writing of strings to/from the TCP connection with
the IRC server. The IRC protocol consists of
[messages](https://tools.ietf.org/html/rfc2812#section-2.3) and
[replies](https://tools.ietf.org/html/rfc2812#section-2.4), which are
respectively converted by the `Connection` into `Command` and `ReplyType` enums.

### dispatcher.rs

**State of development: mostly done, might need tweaks**

The IRC protocol is inherently asynchronous and non-blocking, so callbacks are
used to notify interested parts of the application of particular incoming
messages. As IRC divides its protocol into messages and replies, the dispatcher
maintains sets of command listeners and of reply listeners. Owing to differences
in use cases, these listeners behave differently:

**Command listeners** (`register_command_listener`/`handle_command`) are
permanent callbacks that are registered for a given `CommandType`, eg.
`Privmsg`. Every time a `Command` is received from the server, all registered
listeners for that type are invoked.

**Reply listeners** (`register_reply_listener`/`handle_reply`) are intended to
be ephemeral and are called for _all_ received replies. It is then the
responsibility of the listener to decide 1) what action, if any, to take for a
given reply, and 2) whether or not it wishes to remain registered, which it
communicates to the dispatcher by returning `true` (remain registered) or
`false` (deregister).

For instance, if I run the
[WHOIS command](https://tools.ietf.org/html/rfc2812#section-3.6.2), I expect the
server to respond with a series of `ReplyType`s including `RplWhoIsUser`,
`RplWhoIsServer`, etc., concluding with `RplEndOfWhoIs`. An implementation of
this would register a callback that ignores all replies except for the types
that it is interested in, collates the relevant data from those replies, then
deregisters itself once it receives `RplEndOfWhoIs`.

TODO: There is currently no timeout for reply listeners, but they should
probably automatically deregister after a period of time, with the assumption
that the anticipated message has been dropped.

TODO: It may make sense to have the dispatcher only execute the first interested
listener, then stop evaluating further callbacks. This might alleviate (or maybe
exacerbate) race conditions where we're waiting on responses to multiple
commands at once. Unfortunately, the protocol doesn't provide for "I am replying
to the command ID 123, please route my message accordingly".

### client.rs

**State of development: started**

The `Client` applies a semantic layer on top of the `Connection`. While the
`Connection` sends and receives enums, it makes no attempt to understand their
meaning. The `Client` will expose methods that represent actual IRC actions, such
as `send_message()` or `join_channel()`. It will also handle replying to `PING`
messages received from the server.

It may also be the responsibility of the `Client` to maintain a persistent set of
value objects for things like Channels and Users.

### bot.rs

**State of development: not started**

The `Bot` will interact with the `Client` to define automated behaviour flows, such
as responding to user messages or watching for particular keywords in a channel.

### terminal.rs

**State of development: done**

The Terminal provides an asynchronous interface for the command line for testing
purposes. It allows raw IRC commands to be typed directly at the command line
and sent to the server.
