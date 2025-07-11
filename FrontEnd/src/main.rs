use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Message {
    message: String,
}

fn App() -> Element {
    let msg = use_signal(|| "Click the button to fetch!".to_string());

    rsx! {
        div {
            h1 { "Minimal Frontend RUST WebApp" }
            button {
                onclick: move |_| {
                    let mut msg = msg.clone();
                    spawn(async move {
                        let result = reqwest::get("http://0.0.0.0:3001/api/hello").await;
                        match result {
                            Ok(response) => {
                                let json_result = response.json::<Message>().await;
                                let new_msg = match json_result {
                                    Ok(parsed) => parsed.message,
                                    Err(_) => "Failed to parse JSON".to_string(),
                                };
                                msg.set(new_msg);
                            }
                            Err(_) => msg.set("Failed to fetch message".to_string()),
                        }
                    });
                },
                "Fetch Message From BackEnd"
            }

            p { "{msg}" }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    launch(App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This app is meant to run in the browser as WASM");
}
