pub fn info_text_f32<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {:?}  |  value: {:.3}", id, value)
}

pub fn info_text_i32<ID: std::fmt::Debug>(id: ID, value: i32) -> String {
    format!("id: {:?}  |  value: {}", id, value)
}

pub fn info_text_db<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {:?}  |  value: {:.3} dB", id, value)
}

pub fn info_text_freq<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    if value < 1000.0 {
        format!("id: {:?}  |  value: {:.2} Hz", id, value)
    } else {
        format!("id: {:?}  |  value: {:.2} kHz", id, value / 1000.0)
    }
}
