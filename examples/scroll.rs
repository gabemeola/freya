#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Scroll example", (400.0, 400.0));
}

fn app(cx: Scope) -> Element {
    render!(
        rect {
            height: "100%",
            width: "100%",
            padding: "50",
            background: "white",
            color: "white",
            ScrollView {
                show_scrollbar: true,
                rect {
                    height: "200",
                    width: "400",
                    background: "rgb(214, 40, 40)",
                    padding: "15",
                    rect {
                        height: "100%",
                        width: "100%",
                        background: "rgb(27, 38, 59)",
                        padding: "15",
                        label {  "Scroll support :)" }
                    }
                }
                rect {
                    height: "200",
                    width: "400",
                    background: "rgb(214, 40, 40)",
                    padding: "15",
                    rect {
                        height: "100%",
                        width: "100%",
                        background: "rgb(27, 38, 59)",
                        padding: "15",
                        label { "Scroll support :-)" }
                    }
                }
                rect {
                    height: "200",
                    width: "400",
                    background: "rgb(214, 40, 40)",
                    padding: "15",
                    rect {
                        height: "100%",
                        width: "100%",
                        background: "rgb(27, 38, 59)",
                        padding: "15",
                        label { "Scroll support >:D" }
                    }
                }
            }
        }
    )
}
