use eframe::egui;

use crate::AsyncState;

pub(crate) fn render_statistics(
    ui: &mut egui::Ui,
    snapshot: &AsyncState,
    tx_bps: f64,
    rx_bps: f64,
    api_port: &str,
) {
    let (min_timing, max_timing, avg_timing) =
        timing_stats(&snapshot.timings_ms).unwrap_or((0.0, 0.0, 0.0));

    ui.heading("Request / Response");
    ui.label(format!("Sent payload: {}", snapshot.last_payload));
    ui.label(format!("Server response: {}", snapshot.response_message));
    ui.label(format!(
        "Last payload bytes (TX/RX): {} / {}",
        snapshot.last_sent_bytes, snapshot.last_received_bytes
    ));

    let last_timing = snapshot
        .last_rtt_ms
        .map(|ms| format!("{:.2} ms", ms))
        .unwrap_or_else(|| "-".to_string());

    ui.add_space(10.0);
    ui.heading("Timing");
    ui.label(format!("Completed requests: {}", snapshot.request_count));
    ui.label(format!("In-flight requests: {}", snapshot.in_flight_requests));
    ui.label(format!("Last RTT: {}", last_timing));
    if !snapshot.timings_ms.is_empty() {
        ui.label(format!("Min RTT: {:.2} ms", min_timing));
        ui.label(format!("Max RTT: {:.2} ms", max_timing));
        ui.label(format!("Average RTT: {:.2} ms", avg_timing));
    }

    ui.add_space(10.0);
    ui.heading("Network Telemetry (App-level)");
    ui.label(format!("Total TX bytes: {}", snapshot.total_sent_bytes));
    ui.label(format!("Total RX bytes: {}", snapshot.total_received_bytes));
    ui.label(format!("TX throughput: {}", format_bps(tx_bps)));
    ui.label(format!("RX throughput: {}", format_bps(rx_bps)));
    ui.label(format!(
        "Transport: Browser -> API = gRPC-Web over HTTPS (TCP/{})",
        api_port
    ));
    ui.label("Browser parallel connection limit (typical): 6 per host (may vary by browser)");
    ui.label("Edge note: external HTTP/3 uses UDP/443 at router level.");
    ui.label("Raw TCP/UDP packet counters per port are not accessible from browser WASM.");

    ui.add_space(10.0);
    ui.heading("In-flight Packets (Estimated)");
    if snapshot.request_traces.is_empty() {
        ui.label("No active requests.");
    } else {
        // Find the earliest start_ms in the batch
        let min_start = snapshot.request_traces.iter().map(|t| t.start_ms).fold(f64::INFINITY, f64::min);
        let max_start = snapshot.request_traces.iter().map(|t| t.start_ms).fold(f64::NEG_INFINITY, f64::max);
        let span = (max_start - min_start).max(1.0);
        let bar_width = 120.0;
        let bar_height = 7.0;
        let bar_spacing = 1.5;
        for trace in snapshot
            .request_traces
            .iter()
        {
            let status = if trace.in_flight {
                "flying".to_string()
            } else if let Some(rtt) = trace.rtt_ms {
                format!("done in {:.1} ms", rtt)
            } else {
                "done".to_string()
            };
            let offset = ((trace.start_ms - min_start) / span).clamp(0.0, 1.0);
            let painter = ui.painter();
            let rect = egui::Rect::from_min_size(
                ui.cursor().min + egui::vec2(offset as f32 * bar_width, 0.0),
                egui::vec2(bar_width * 0.7, bar_height),
            );
            painter.rect_filled(rect, 1.0, trace.color);
            // Draw label and status to the right of the bar, with a very small font
            let text_pos = rect.right_top() + egui::vec2(6.0, 0.0);
            let font_id = egui::FontId::new(bar_height, egui::FontFamily::Proportional);
            painter.text(
                text_pos,
                egui::Align2::LEFT_TOP,
                format!("#{} {} | {}", trace.id, trace.label, status),
                font_id,
                trace.color,
            );
            ui.add_space(bar_height + bar_spacing);
        }
    }

    ui.add_space(10.0);
    ui.heading("Batch Average Speed");
    if let Some(last_batch) = snapshot.batch_traces.last() {
        if last_batch.completed_requests > 0 {
            ui.label(format!(
                "Last batch #{} avg speed/request: {}",
                last_batch.id,
                format_bps(last_batch.average_speed_bps)
            ));
            ui.label(format!(
                "Batch progress: {}/{} completed",
                last_batch.completed_requests, last_batch.total_requests
            ));
        }
    }
}

fn timing_stats(values: &[f64]) -> Option<(f64, f64, f64)> {
    if values.is_empty() {
        return None;
    }

    let mut min = values[0];
    let mut max = values[0];
    let mut sum = 0.0;

    for value in values {
        min = min.min(*value);
        max = max.max(*value);
        sum += *value;
    }

    Some((min, max, sum / values.len() as f64))
}

fn format_bps(bytes_per_second: f64) -> String {
    if bytes_per_second >= 1024.0 * 1024.0 {
        format!("{:.2} MB/s", bytes_per_second / (1024.0 * 1024.0))
    } else if bytes_per_second >= 1024.0 {
        format!("{:.2} KB/s", bytes_per_second / 1024.0)
    } else {
        format!("{:.2} B/s", bytes_per_second)
    }
}