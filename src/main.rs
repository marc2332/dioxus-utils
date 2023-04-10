#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod use_channel;

use freya::prelude::*;

use crate::use_channel::use_channel;

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
