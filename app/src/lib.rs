use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use tonic_web_wasm_client::Client;

pub mod ping {
    tonic::include_proto!("ping");
}

use ping::greeter_client::GreeterClient;
use ping::{PingRequest, PongReply};

#[wasm_bindgen(start)]
pub fn main() {
    sycamore::render(|| {
        // 1. User input signal
        let user_input = create_signal(String::new());
        
        // 2. Signal for the server response
        let response_message = create_signal("Nessun messaggio ricevuto".to_string());
        
        // 3. Signal for the loading state (UI feedback)
        let is_loading = create_signal(false);

        // Send function to call the gRPC-Web endpoint
        let handle_ping = move |_| {
            let name = user_input.get_clone();
            if name.is_empty() { return; }

            is_loading.set(true);
            
            spawn_local(async move {
                let base_url = "https://api.localhost".to_string();
                let client = Client::new(base_url);
                let mut grpc_client = GreeterClient::new(client);

                let request = tonic::Request::new(PingRequest { name });

                match grpc_client.say_ping(request).await {
                    Ok(response) => {
                        let reply: PongReply = response.into_inner();
                        response_message.set(reply.message);
                    },
                    Err(e) => {
                        response_message.set(format!("Errore: {}", e.message()));
                    }
                }
                is_loading.set(false);
            });
        };

        view! {
            div(style="font-family: sans-serif; max-width: 400px; margin: 40px auto; text-align: center; border: 1px solid #eee; padding: 20px; border-radius: 10px; shadow: 0 4px 6px rgba(0,0,0,0.1)") {
                h2 { "gRPC-Web Ping Tester" }

                // Input field
                input(
                    r#type="text", // Aggiungi r# davanti a type
                    placeholder="Scrivi qualcosa...",
                    bind:value=user_input,
                    style="padding: 10px; width: 80%; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                )

                div {
                    button(
                        on:click=handle_ping,
                        disabled=is_loading.get(),
                        style="padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                    ) {
                        (if is_loading.get() { "Invio in corso..." } else { "Invia Ping" })
                    }
                }

                // Result area
                div(style="margin-top: 30px; padding: 15px; background: #f9f9f9; border-top: 2px solid #007bff;") {
                    p(style="font-size: 0.9rem; color: #666;") { "Risposta dal Server:" }
                    h3(style="color: #333;") { (response_message.get_clone()) }
                }
            }
        }
    });
}