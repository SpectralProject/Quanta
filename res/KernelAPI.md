# Ardaku API
[Based on this](https://github.com/ardaku/ardaku/blob/main/README.md)

The API for applications to communicate with Ardaku and each other is based on
channel communication inspired by kqueue (reduced syscalls) and IOCP
(completion-based).  The API consists of a single function:

```wat
(import "ardaku" "event" (func $event
    (param $size i32)   ;; Message list size
    (param $data i32)   ;; Message list reference
    (param $done i32)   ;; Reference to channels ready (size=open channel count)
    (result i32)        ;; Number of channels ready (max 16384 - size trunc.)
))
```

This function sends messages on channels in the message list, then waits until
at least one message is ready, then writes the channel IDs into `$done`, and
returns the size.  Then, usually the channel IDs are used to index and execute
functions in an array of function references. The message struct looks like
this:

```rust
#[repr(C, packed)]
struct Message {
    /// Channel to send a message on (0 is special "connector" channel)
    pub channel_id: i32,
    /// Channel ID is a user-chosen index into an array of function references.
    /// (set to 0 to disconnect `channel_id`)
    pub channel_id_new: u32,
    /// Size of message (in bytes)
    pub message_size: u32,
    /// Message data.
    pub message_data: *mut u8,
}
```

When `channel_id` is 0:

 - `log`: Receives UTF-8 log messages and saves them (stdout)
 - `prompt`: Sends UTF-8 debug commands (stdin)

 - `screen`: Receives pixels to display on the screen
 - `camera`: Sends pixels from "a source" / Receives settings commands

 - `speakers`: Receives f32 interleaved audio to play
 - `microphone`: Sends f32 interleaved audio to record

 - `save`: Receives atomic file patches
 - `load`: Sends requested sections of a file

 - `share`: Receives file for exporting to other app
 - `grab`: Sends file from other app for importing

 - `haptic`: Receives haptic events
 - `input`: Sends input events / Receives input events for subprocesses

## Notes
- Disconnecting from service 0 with empty buffer stops the program.
- Disconnecting from service with buffer is reserved for future use (crashes).

- You can disconnect a channel in the middle of it's processing as a way of
cancelling the I/O.  Usually, if not always the I/O will complete anyway - it
just won't notify you (file I/O cancelling might be added in the future).

If the channel message has not been "completed", then the program is expected
to not try sending another message until it receives a message back from the
service.  If a message is sent before the other one has been processed, the
program will crash.  It's up to the programmer to choose and implement one of:
buffering, dropping, cancelling, or blocking of messages.

- buffering: build a bounded (or unbounded) queue of messages - bounded queues
   must fall back to dropping, cancelling or blocking.
- dropping: ignore the most recent event.
- cancelling: cancel and replace the last message with most recent event.
- blocking: send message immediately upon completion of last message.