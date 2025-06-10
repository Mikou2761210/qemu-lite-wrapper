// QMP message dispatcher
// -------------------------------------------------------------
// This module provides a flexible, dynamically‑registered dispatcher
// for handling QMP messages (Event / Reply / Error / Unknown).
//
// Design highlights
// -----------------
// * Dynamic registration with HashMap so users can register handlers
//   at runtime without modifying the library code.
// * Event handlers are keyed by the QMP event name (e.g. "SHUTDOWN").
// * Reply / Error handlers are keyed by the opaque `id` field that the
//   caller supplied when issuing the QMP command.  This matches the
//   QMP spec where `id` is echoed back in replies and errors.
// * An optional catch‑all handler for `Unknown` messages.
// * Thread‑safe (`Send + Sync`) handler trait objects so the dispatcher
//   can be shared across tasks if desired.
//
// Usage example
// -------------
// ```rust
// let mut dispatcher = QmpDispatcher::new();
// dispatcher.register_event_handler("SHUTDOWN", |ev| {
//     println!("Guest shutdown: {:?}", ev);
// });
// dispatcher.register_reply_handler(42, |rep| {
//     println!("query-status reply: {:?}", rep);
// });
//
// // inside an async loop that yields QmpMessage values …
// dispatcher.dispatch(&msg);
// ```
// -------------------------------------------------------------

use std::collections::HashMap;

use crate::qmp::{
    messages::{QmpError, QmpEvent, QmpMessage, QmpReply, QmpUnknown},
    types::QmpId,
};

/// Type alias for convenience – a boxed, thread‑safe, immutable handler fn.
/// We use `&T` so the handler cannot mutate the message; if users need that
/// they can clone before mutating.
type EvHandler<T> = Box<dyn Fn(&T) + Send + Sync + 'static>;

pub struct QmpDispatcher {
    event_handlers: HashMap<String, EvHandler<QmpEvent>>, // keyed by `event` name
    reply_handlers: HashMap<QmpId, EvHandler<QmpReply>>,  // keyed by `id`
    error_handlers: HashMap<QmpId, EvHandler<QmpError>>,  // keyed by `id`
    unknown_handler: Option<EvHandler<QmpUnknown>>,       // optional catch‑all
}

impl Default for QmpDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl QmpDispatcher {
    /// Create an empty dispatcher.
    pub fn new() -> Self {
        Self {
            event_handlers: HashMap::new(),
            reply_handlers: HashMap::new(),
            error_handlers: HashMap::new(),
            unknown_handler: None,
        }
    }

    // ---------------------------------------------------------
    // Registration helpers
    // ---------------------------------------------------------

    /// Register handler for a specific QMP event (by `event` name).
    pub fn register_event_handler<F>(&mut self, event_name: impl Into<String>, handler: F)
    where
        F: Fn(&QmpEvent) + Send + Sync + 'static,
    {
        self.event_handlers
            .insert(event_name.into(), Box::new(handler));
    }

    /// Register handler for a specific command `id` – successful reply.
    pub fn register_reply_handler<F>(&mut self, id: QmpId, handler: F)
    where
        F: Fn(&QmpReply) + Send + Sync + 'static,
    {
        self.reply_handlers.insert(id, Box::new(handler));
    }

    /// Register handler for a specific command `id` – error reply.
    pub fn register_error_handler<F>(&mut self, id: QmpId, handler: F)
    where
        F: Fn(&QmpError) + Send + Sync + 'static,
    {
        self.error_handlers.insert(id, Box::new(handler));
    }

    /// Register a catch‑all handler for `Unknown` messages.
    pub fn register_unknown_handler<F>(&mut self, handler: F)
    where
        F: Fn(&QmpUnknown) + Send + Sync + 'static,
    {
        self.unknown_handler = Some(Box::new(handler));
    }

    // ---------------------------------------------------------
    // Dispatch entry point
    // ---------------------------------------------------------

    /// Dispatch a single `QmpMessage` to the appropriate handler (if any).
    ///
    /// The dispatcher never panics; unhandled messages are simply ignored
    /// (or sent to the catch‑all unknown handler if provided).
    pub fn dispatch(&self, message: &QmpMessage) {
        match message {
            QmpMessage::Greeting(_) => {
                log::info!("QEMU greeted us");
            }
            QmpMessage::Event(ev) => {
                if let Some(handler) = self.event_handlers.get(&ev.name) {
                    handler(ev);
                }
            }
            QmpMessage::Reply(rep) => {
                if let Some(id) = &rep.id {
                    if let Some(handler) = self.reply_handlers.get(&id) {
                        handler(rep);
                    }
                }
            }
            QmpMessage::Error(err) => {
                if let Some(id) = &err.id {
                    if let Some(handler) = self.error_handlers.get(&id) {
                        handler(err);
                    }
                }
            }
            QmpMessage::Unknown(u) => {
                if let Some(handler) = &self.unknown_handler {
                    handler(u);
                }
            }
        }
    }
}
