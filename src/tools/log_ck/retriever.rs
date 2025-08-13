use eframe::egui::{Align, Button, Label, Layout, ScrollArea, TextEdit, Ui, text::LayoutJob};

use super::log_line::{ClickhouseResponse, LogLine, ServiceType};
use crate::style;

static HOST: &str = "http://observability-ch-prod.middleware.hlmd-prod.ch.nioint.com";
static USER: &str = "rouser";
static PASSWORD: &str = "rouser@nioad2024";
static DB: &str = "log";

pub struct LogRetriever {
    trace_id: String,
    from: String,
    to: String,
    rendered: LayoutJob,
    loading: bool,
    inbox: egui_inbox::UiInbox<Vec<LogLine>>,
}

impl crate::tools::ToolItem for LogRetriever {
    fn name(&self) -> &str {
        "Issue 日志整合查询"
    }

    fn description(&self) -> &str {
        "综合查询 issue/issue-sim 日志"
    }

    fn update(&mut self, ui: &mut Ui) {
        if self.loading {
            if let Some(lines) = self.inbox.read(ui).last() {
                let mut job = LayoutJob::default();
                for line in &lines {
                    line.append_to_layout(&mut job, ui.visuals().dark_mode);
                }
                self.rendered = job;
                self.loading = false;
            }
        }

        ui.allocate_ui_with_layout(
            (0.0, 24.0).into(),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.set_height(24.0);
                ui.label("From");
                ui.text_edit_singleline(&mut self.from);
                ui.label("To");
                ui.text_edit_singleline(&mut self.to);
                let btn_response = ui.scope(|ui| {
                    ui.spacing_mut().button_padding = (8.0, 4.0).into();
                    ui.add(
                        Button::new("🔍 查询").fill(style::primary_color(ui.visuals().dark_mode)),
                    )
                });
                if btn_response.inner.clicked() {
                    self.loading = true;
                    let tx = self.inbox.sender();
                    let url = make_ck_query(&vec![self.trace_id.clone()]);
                    let mut req = ehttp::Request::get(url);
                    req.headers.insert("X-ClickHouse-User", USER);
                    req.headers.insert("X-ClickHouse-Key", PASSWORD);
                    ehttp::fetch(req, move |result| {
                        let resp = match result {
                            Ok(resp) => {
                                if resp.status == 200 {
                                    serde_json::from_slice::<ClickhouseResponse>(&resp.bytes)
                                        .unwrap_or_else(|e| ClickhouseResponse {
                                            data: vec![LogLine {
                                                message: e.to_string(),
                                                ts: String::new(),
                                                service: ServiceType::Other("<error>".into()),
                                            }],
                                        })
                                } else {
                                    ClickhouseResponse {
                                        data: vec![LogLine {
                                            message: resp.text().unwrap_or("").into(),
                                            ts: String::new(),
                                            service: ServiceType::Other("<error>".into()),
                                        }],
                                    }
                                }
                            }
                            Err(e) => ClickhouseResponse {
                                data: vec![LogLine {
                                    message: e.to_string(),
                                    ts: String::new(),
                                    service: ServiceType::Other("<error>".into()),
                                }],
                            },
                        };
                        tx.send(resp.data).unwrap();
                    });
                }
            },
        );
        ScrollArea::vertical()
            .id_salt("rendered")
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.add(Label::new(self.rendered.to_owned()));
            });
    }
}

impl Default for LogRetriever {
    fn default() -> Self {
        Self {
            loading: false,
            inbox: egui_inbox::UiInbox::new(),
            from: String::new(),
            to: String::new(),
            trace_id: String::new(),
            rendered: LayoutJob::default(),
        }
    }
}

fn make_ck_query(keys: &[String]) -> String {
    let mut query: String = "SELECT serviceName, ts, rawLog FROM unified_log_v1 WHERE ".to_owned();
    let keyword_condition = keys
        .iter()
        .map(|k| format!("rawLog LIKE '%{}%'", k))
        .collect::<Vec<_>>()
        .join(" AND ");
    query.push_str(&keyword_condition);
    query.push_str(" ORDER BY ts DESC LIMIT 1000 FORMAT JSON");
    let mut url = url::Url::parse(HOST).unwrap();
    url.query_pairs_mut()
        .append_pair("query", &query)
        .append_pair("database", DB);
    url.to_string()
}
