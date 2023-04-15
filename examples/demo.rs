#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use freya_template::{
    use_channel::{use_channel, use_listen_channel, UseChannel},
    use_interval::use_interval,
};

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
#[inline_props]
fn Listener(cx: Scope, channel: UseChannel<String>) -> Element {
    let listener_b = use_listen_channel(cx, channel, move |msg| async move {
        println!("Listener B: {msg}");
    });

    let stop = {
        to_owned![listener_b];
        move |_: MouseEvent| {
            listener_b.stop();
        }
    };

    let resume = move |_: MouseEvent| {
        listener_b.resume();
    };

    render!(
        label {
            onclick: stop,
            "Stop B"
        }
        label {
            onclick: resume,
            "Resume B"
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

    let clear = {
        to_owned![interval];
        move |_: MouseEvent| {
            interval.clear();
        }
    };

    let resume = move |_: MouseEvent| {
        interval.resume();
    };

    render!(
        label {
            onclick: clear,
            "Clear"
        }
        label {
            onclick: resume,
            "Resume"
        }
    )
}
