pub struct RazerARGBReport {
    report_id: u8,
    channel_1: u8,
    channel_2: u8,
    pad: u8,
    last_idx: u8,
    color_data: [u8; 315],
}