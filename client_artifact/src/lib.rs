#[wasm_bindgen]
pub async fn start_web(canvas_id: String) -> Result<(), JsValue> {
    start(canvas_id).await
}
use eframe::egui;
use std::cell::RefCell;
use std::rc::Rc;
use tonic_web_wasm_client::Client;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod stats_panel;
mod request_utils;

pub mod ping {
    tonic::include_proto!("ping");
}

use ping::greeter_client::GreeterClient;
use ping::{PingRequest, PongReply};

#[derive(Clone)]
struct RequestTrace {
    id: u64,
    label: String,
    color: egui::Color32,
    sent_bytes: usize,
    received_bytes: usize,
    packets_estimate: usize,
    in_flight: bool,
    rtt_ms: Option<f64>,
    start_ms: f64,
}

#[derive(Clone)]
struct BatchTrace {
    id: u64,
    total_requests: usize,
    completed_requests: usize,
    accumulated_speed_bps: f64,
    average_speed_bps: f64,
}

#[derive(Default, Clone)]
struct AsyncState {
    response_message: String,
    is_loading: bool,
    last_payload: String,
    last_rtt_ms: Option<f64>,
    last_sent_bytes: usize,
    last_received_bytes: usize,
    total_sent_bytes: u64,
    total_received_bytes: u64,
    request_count: u64,
    in_flight_requests: u64,
    timings_ms: Vec<f64>,
    request_traces: Vec<RequestTrace>,
    batch_traces: Vec<BatchTrace>,
}

struct PingApp {
    user_input: String,
    parallel_input: String,
    batch_seq: u64,
    base_url: String,
    base_url_for_link: String,
    cert_enabled: bool,
    app_started_ms: f64,
    state: Rc<RefCell<AsyncState>>,
}

impl PingApp {
    const CERT_STORAGE_KEY: &'static str = "api_cert_enabled";

    fn new() -> Self {
        let host = option_env!("API_HOST").expect("API_HOST missing env var");
        let port = option_env!("API_PORT").expect("API_PORT missing env var");
        let base_url = format!("https://{}:{}", host, port);
        let cert_enabled = Self::is_cert_enabled();

        Self {
            user_input: String::new(),
            parallel_input: "1".to_string(),
            batch_seq: 0,
            base_url_for_link: base_url.clone(),
            base_url,
            cert_enabled,
            app_started_ms: request_utils::now_ms(),
            state: Rc::new(RefCell::new(AsyncState {
                response_message: "No message received".to_string(),
                is_loading: false,
                last_payload: "-".to_string(),
                last_rtt_ms: None,
                last_sent_bytes: 0,
                last_received_bytes: 0,
                total_sent_bytes: 0,
                total_received_bytes: 0,
                request_count: 0,
                in_flight_requests: 0,
                timings_ms: Vec::new(),
                request_traces: Vec::new(),
                batch_traces: Vec::new(),
            })),
        }
    }

    fn send_ping(&mut self, ctx: egui::Context) {
        let payload = self.user_input.trim().to_string();
        if payload.is_empty() {
            return;
        }

        // Defer clearing traces until after new batch is visible
        let mut had_previous_traces = false;
        {
            let shared = self.state.borrow();
            if !shared.request_traces.is_empty() {
                had_previous_traces = true;
            }
        }

        let parallel = self
            .parallel_input
            .trim()
            .parse::<usize>()
            .ok()
            .map(|value| value.clamp(1, 60))
            .unwrap_or(1);
        self.batch_seq += 1;
        let batch_id = self.batch_seq;

        {
            let mut shared = self.state.borrow_mut();
            shared.last_payload = if parallel > 1 {
                format!("{} (x{})", payload, parallel)
            } else {
                payload.clone()
            };
            shared.is_loading = true;
            shared.batch_traces.push(BatchTrace {
                id: batch_id,
                total_requests: parallel,
                completed_requests: 0,
                accumulated_speed_bps: 0.0,
                average_speed_bps: 0.0,
            });
            if shared.batch_traces.len() > 120 {
                let overflow = shared.batch_traces.len() - 120;
                shared.batch_traces.drain(0..overflow);
            }
        }

        for idx in 0..parallel {
            let request_id = idx + 1; // Reset id for each batch
            let request_name = if parallel > 1 {
                format!("{} #{}", payload, request_id)
            } else {
                payload.clone()
            };
            let sent_bytes = request_name.len();
            let request_color = request_utils::request_color(request_id as u64);
            let start_ms = request_utils::now_ms();

            {
                let mut shared = self.state.borrow_mut();
                shared.in_flight_requests += 1;
                shared.last_sent_bytes = sent_bytes;
                shared.request_traces.push(RequestTrace {
                    id: request_id as u64,
                    label: request_name.clone(),
                    color: request_color,
                    sent_bytes,
                    received_bytes: 0,
                    packets_estimate: request_utils::estimate_packets(sent_bytes, 0),
                    in_flight: true,
                    rtt_ms: None,
                    start_ms,
                });
                // No limit: show all traces for this batch
            }

            let base_url = self.base_url.clone();
            let state = Rc::clone(&self.state);
            let start_ms = request_utils::now_ms();
            let repaint_ctx = ctx.clone();

            spawn_local(async move {
                let client = Client::new(base_url);
                let mut grpc_client = GreeterClient::new(client);
                let request = tonic::Request::new(PingRequest {
                    name: request_name.clone(),
                });

                let message = match grpc_client.say_ping(request).await {
                    Ok(response) => {
                        let reply: PongReply = response.into_inner();
                        reply.message
                    }
                    Err(e) => format!("Errore: {}", e.message()),
                };

                let response_bytes = message.len();

                {
                    let mut shared = state.borrow_mut();
                    shared.response_message = message;
                    shared.request_count += 1;
                    shared.last_sent_bytes = sent_bytes;
                    shared.last_received_bytes = response_bytes;
                    shared.total_sent_bytes += sent_bytes as u64;
                    shared.total_received_bytes += response_bytes as u64;
                    shared.in_flight_requests = shared.in_flight_requests.saturating_sub(1);

                    let elapsed = (request_utils::now_ms() - start_ms).max(0.0);
                    shared.last_rtt_ms = Some(elapsed);
                    shared.timings_ms.push(elapsed);
                    if shared.timings_ms.len() > 120 {
                        let overflow = shared.timings_ms.len() - 120;
                        shared.timings_ms.drain(0..overflow);
                    }

                    if let Some(trace) = shared.request_traces.iter_mut().find(|trace| trace.id == request_id as u64)
                    {
                        trace.received_bytes = response_bytes;
                        trace.packets_estimate =
                            request_utils::estimate_packets(trace.sent_bytes, response_bytes);
                        trace.in_flight = false;
                        trace.rtt_ms = Some(elapsed);
                    }

                    if let Some(batch) = shared.batch_traces.iter_mut().find(|batch| batch.id == batch_id)
                    {
                        let elapsed_s = (elapsed / 1000.0).max(0.000_001);
                        let req_speed_bps = (sent_bytes + response_bytes) as f64 / elapsed_s;
                        batch.completed_requests += 1;
                        batch.accumulated_speed_bps += req_speed_bps;
                        batch.average_speed_bps =
                            batch.accumulated_speed_bps / batch.completed_requests as f64;
                    }

                    shared.is_loading = shared.in_flight_requests > 0;
                }

                repaint_ctx.request_repaint();
            });
        }

        // Now clear previous traces if this is not the first batch
        if had_previous_traces {
            let mut shared = self.state.borrow_mut();
            shared.request_traces.drain(0..parallel);
        }
    }

    fn api_port(&self) -> String {
        self.base_url
            .rsplit(':')
            .next()
            .map(|s| s.trim_end_matches('/').to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    fn is_cert_enabled() -> bool {
        let Some(window) = web_sys::window() else {
            return false;
        };
        let Ok(storage_opt) = window.local_storage() else {
            return false;
        };
        let Some(storage) = storage_opt else {
            return false;
        };

        matches!(storage.get_item(Self::CERT_STORAGE_KEY), Ok(Some(value)) if value == "1")
    }

    fn mark_cert_enabled() {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(storage_opt) = window.local_storage() else {
            return;
        };
        let Some(storage) = storage_opt else {
            return;
        };

        let _ = storage.set_item(Self::CERT_STORAGE_KEY, "1");
    }

    fn enable_certificate_once(&mut self) {
        if self.cert_enabled {
            return;
        }

        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url_and_target(&self.base_url_for_link, "_blank");
            self.cert_enabled = true;
            Self::mark_cert_enabled();
        }
    }
}

impl eframe::App for PingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let snapshot = self.state.borrow().clone();
        let uptime_s = ((request_utils::now_ms() - self.app_started_ms) / 1000.0).max(0.001);
        let tx_bps = snapshot.total_sent_bytes as f64 / uptime_s;
        let rx_bps = snapshot.total_received_bytes as f64 / uptime_s;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("gRPC-Web Rust Dashboard");
            ui.add_space(8.0);

            ui.label("Send a message...");
            ui.text_edit_singleline(&mut self.user_input);
            ui.horizontal(|ui| {
                ui.label("Parallel requests (max limits 60): ");
                ui.add(egui::TextEdit::singleline(&mut self.parallel_input).desired_width(70.0));
            });
            ui.add_space(8.0);

            let button_text = if snapshot.is_loading {
                "Invio in corso..."
            } else {
                "Invia Ping"
            };

            if ui
                .add_enabled(!snapshot.is_loading, egui::Button::new(button_text))
                .clicked()
            {
                // Clear previous traces only when user clicks the button
                {
                    let mut shared = self.state.borrow_mut();
                    shared.request_traces.clear();
                }
                self.send_ping(ctx.clone());
            }

            ui.add_space(16.0);
            ui.separator();
            stats_panel::render_statistics(ui, &snapshot, tx_bps, rx_bps, &self.api_port());

            if !self.cert_enabled && ui.link("Enable API Server certificates").clicked() {
                self.enable_certificate_once();
            }
        });
    }
}

#[wasm_bindgen]
pub async fn start(canvas_id: String) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window not available"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document not available"))?;
    let canvas = document
        .get_element_by_id(&canvas_id)
        .ok_or_else(|| JsValue::from_str("Canvas element not found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Element is not a HtmlCanvasElement"))?;

    let options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            canvas,
            options,
            Box::new(|_cc| Ok(Box::new(PingApp::new()))),
        )
        .await?;
    Ok(())
}