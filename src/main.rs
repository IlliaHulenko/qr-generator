use dioxus::prelude::*;
use qrcode::{render::svg, QrCode};

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut input_value = use_signal(|| "".to_string());

    let qr_code_svg = use_memo(move || {
        QrCode::new(input_value())
            .unwrap()
            .render::<svg::Color>()
            .build()
    });

    rsx! {
        document::Title {"QR code Generator"}
        document::Meta {name: "description", content: "Instantly generate QR code in your browser"}
        document::Meta {name: "viewport", content: "width=device-width, initial-scale=1.0"}
        document::Meta {name: "theme-color", content: "#0F1116"}
        document::Link {rel: "icon", href: "/favicon.png"}
        document::Stylesheet { href: CSS }

        div {
            class: "container",

            div {
                class: "input",

                input {
                    type: "text",
                    placeholder: "Type something to generate QR code...",
                    oninput: move |e| input_value.set(e.value())
                }
            }

            div {
                class: "qr",
                dangerous_inner_html: qr_code_svg
            }

        }

    }
}
