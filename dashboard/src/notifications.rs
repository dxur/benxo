use std::{cell::Cell, time::Duration};

use leptos::prelude::*;
use strum::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum MessageType {
    #[strum(serialize = "success")]
    Success,
    #[strum(serialize = "info")]
    Info,
    #[strum(serialize = "warn")]
    Warn,
    #[strum(serialize = "error")]
    Error,
}

#[derive(Debug, Clone)]
pub struct Message {
    id: usize,
    msg_type: MessageType,
    title: String,
    msg: String,
}

thread_local! {
    static MESSAGES: RwSignal<Vec<Message>> = Default::default();
    static COUNTER: Cell<usize> = Default::default();
}

#[component]
pub fn Notifications() -> impl IntoView {
    let messages = MESSAGES.with(|cell| cell.clone());
    view! {
        <div data-notif>
            <For
                each=move || messages.get()
                key=|msg| msg.id
                let(msg)
            >
                <div data-msg
                    data-role=msg.msg_type.to_string()
                >
                    <strong> {msg.title} </strong>
                    {msg.msg}
                    <button type="reset"
                        on:click=move |_| MESSAGES.with(|cell| cell.update(|messages| {
                            messages.retain(|m| m.id != msg.id);}))
                    >"×"</button>
                </div>
            </For>
        </div>
    }
}

pub fn notify(msg_type: MessageType, title: impl ToString, msg: impl ToString) {
    let id = COUNTER.with(|cell| cell.replace(cell.get() + 1));
    let msg = Message {
        id,
        msg_type,
        title: title.to_string(),
        msg: msg.to_string(),
    };
    
    MESSAGES.with(|cell| {
        cell.try_update(|messages| messages.push(msg));
        set_timeout(
            move || {
                MESSAGES.with(|cell| cell.update(|messages| {
                    messages.retain(|m| m.id != id);
                }));
            },
            Duration::from_secs(5),
        );
    });
}

pub fn error(msg: impl ToString) {
    notify(MessageType::Error, "Error", msg);
}

pub fn success(msg: impl ToString) {
    notify(MessageType::Success, "Success", msg);
}

pub fn info(msg: impl ToString) {
    notify(MessageType::Info, "Info", msg);
}

pub fn warn(msg: impl ToString) {
    notify(MessageType::Warn, "Warning", msg);
}

pub fn clear() {
    MESSAGES.with(|cell| cell.set(Default::default()));
}