use eframe::egui;

pub(crate) fn now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|perf| perf.now())
        .unwrap_or(0.0)
}

pub(crate) fn estimate_packets(sent_bytes: usize, received_bytes: usize) -> usize {
    let bytes = (sent_bytes + received_bytes).max(1);
    bytes.div_ceil(1200)
}

pub(crate) fn request_color(request_id: u64) -> egui::Color32 {
    const PALETTE: [egui::Color32; 10] = [
        egui::Color32::from_rgb(239, 68, 68),
        egui::Color32::from_rgb(245, 158, 11),
        egui::Color32::from_rgb(234, 179, 8),
        egui::Color32::from_rgb(34, 197, 94),
        egui::Color32::from_rgb(16, 185, 129),
        egui::Color32::from_rgb(14, 165, 233),
        egui::Color32::from_rgb(59, 130, 246),
        egui::Color32::from_rgb(99, 102, 241),
        egui::Color32::from_rgb(168, 85, 247),
        egui::Color32::from_rgb(236, 72, 153),
    ];

    PALETTE[(request_id as usize) % PALETTE.len()]
}