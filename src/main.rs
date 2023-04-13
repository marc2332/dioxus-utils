#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod use_channel;
mod use_interval;

use std::time::Duration;

use freya::prelude::*;
use use_channel::UseChannel;

use crate::use_channel::{use_channel, use_listen_channel};
use crate::use_interval::use_interval;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
#[inline_props]
fn Listener(cx: Scope, channel: UseChannel<String>) -> Element {
    let listener_b = use_listen_channel(cx, &channel, move |msg| async move {
        println!("Listener B: {msg}");
    });

    let stop = move |_: MouseEvent| {
        listener_b.stop();
    };

    render!(
        label {
            onclick: stop,
            "Stop B"
        }
    )
}

fn app(cx: Scope) -> Element {
    let channel = use_channel::<String>(cx, 5);

    use_listen_channel(cx, &channel, move |msg| async move {
        println!("Listener A: {msg}");
    });

    let send = {
        to_owned![channel];
        move |_: MouseEvent| {
            channel.send("Hello").ok();
        }
    };

    render!(
        label {
            onclick: send,
            "Send hello"
        }
        Listener {
            channel: channel
        }
    )
}

#[allow(non_snake_case, dead_code)]
fn IntervalApp(cx: Scope) -> Element {
    let interval = use_interval(
        cx,
        Duration::from_millis(100),
        move |_interval| async move {
            println!("tick! tick!!");
        },
    );

    let onclick = move |_: MouseEvent| {
        interval.clear();
    };

    render!(
        label {
            onclick: onclick,
            "Clear"
        }
    )
}
