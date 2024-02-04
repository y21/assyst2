use assyst_common::assyst::ThreadSafeAssyst;

use self::incoming_event::IncomingEvent;

pub mod event_handlers;
pub mod incoming_event;
pub mod message_parser;

/// Checks the enum variant of this IncomingEvent and calls the appropriate handler function
/// for further processing.
pub async fn handle_raw_event(context: ThreadSafeAssyst, event: IncomingEvent) {
    match event {
        IncomingEvent::ShardReady(event) => {
            event_handlers::ready::handle(event);
        },
        IncomingEvent::MessageCreate(event) => {
            event_handlers::message_create::handle(context, event).await;
        },
        IncomingEvent::MessageUpdate(event) => {
            event_handlers::message_update::handle(context, event).await;
        },
        IncomingEvent::MessageDelete(event) => {
            event_handlers::message_delete::handle(event);
        },
        IncomingEvent::GuildCreate(event) => {
            event_handlers::guild_create::handle(event).await;
        },
        IncomingEvent::GuildDelete(event) => {
            event_handlers::guild_delete::handle(event);
        },
    }
}