#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod use_channel;
mod use_interval;

use std::time::Duration;

use freya::prelude::*;

use crate::use_channel::use_channel;
use crate::use_interval::use_interval;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let channel = use_channel::<String>(cx, 5);

    use_effect(cx, (), {
        to_owned![channel];
        move |_| async move {
            while let Ok(msg) = channel.recv().await {
                println!("Listener A: {msg}")
            }
        }
    });

    use_effect(cx, (), {
        to_owned![channel];
        move |_| async move {
            while let Ok(msg) = channel.recv().await {
                println!("Listener B: {msg}")
            }
        }
    });

    let onclick = move |_: MouseEvent| {
        channel.send("Hello").ok();
    };

    render!(
        label {
            onclick: onclick,
            "Send hello"
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
