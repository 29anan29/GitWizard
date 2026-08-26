use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogLine {
    pub kind: String,
    pub line: String,
}

pub fn log(app: &AppHandle, kind: &str, line: &str) {
    let _ = app.emit(
        "git-log",
        LogLine {
            kind: kind.into(),
            line: line.into(),
        },
    );
}

pub fn cmd(app: &AppHandle, line: &str) {
    log(app, "cmd", line);
}

pub fn out(app: &AppHandle, line: &str) {
    log(app, "out", line);
}

pub fn progress(app: &AppHandle, received: usize, total: usize) {
    let percent = if total > 0 {
        ((received.min(total)) * 100 / total) as u32
    } else {
        0
    };
    let _ = app.emit(
        "push-progress",
        serde_json::json!({ "received": received, "total": total, "percent": percent }),
    );
}
