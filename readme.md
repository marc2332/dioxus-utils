# dioxus-utils

These are just experiments for myself, I will move these to https://github.com/DioxusLabs/dioxus-std eventually

- use_channel
```rust
fn app(cx: Scope) -> Element {
    let channel = use_channel::<String>(cx, 5);

    use_listen_channel(cx, &channel, move |msg| async move {
        println!("Listener: {msg}");
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
```
- use_interval
```rust
fn app(cx: Scope) -> Element {
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
```
