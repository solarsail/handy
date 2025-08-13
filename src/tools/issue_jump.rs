use eframe::egui::{Align, Button, Frame, Label, Layout, RichText, Ui, output::OpenUrl};
use egui_inbox::UiInbox;
use serde::{Deserialize, Serialize};

use crate::style;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum IdType {
    Event,
    Triage,
    Data,
    Scenario,
}

impl IdType {
    fn field(&self) -> &str {
        match self {
            IdType::Event => "event.id",
            IdType::Scenario => "scenario.unique_id",
            IdType::Triage => "triage.id",
            IdType::Data => "data.id",
        }
    }

    fn model(&self) -> &str {
        match self {
            IdType::Event => "event",
            IdType::Scenario => "scenario",
            IdType::Triage => "triage",
            IdType::Data => "data",
        }
    }
}

enum ReqStatus {
    Idle,
    Pending,
}

pub struct IssueJump {
    id: String,
    id_type: IdType,
    warning: String,
    req_status: ReqStatus,
    inbox: UiInbox<Result<u64, String>>,
}

impl super::ToolItem for IssueJump {
    fn name(&self) -> &str {
        "Issue 快速跳转"
    }

    fn description(&self) -> &str {
        "通过 id 快速打开 issue 页面"
    }

    fn update(&mut self, ui: &mut Ui) {
        let mut response_on_change = Vec::new();
        ui.allocate_ui_with_layout(
            (200.0, 32.0).into(),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.label("ID");
                let r = ui.text_edit_singleline(&mut self.id);
                response_on_change.push(r);
                let btn_response = ui.scope(|ui| {
                    ui.spacing_mut().button_padding = (8.0, 4.0).into();
                    let btn_label = match self.req_status {
                        ReqStatus::Pending => "...取消",
                        _ => "→ 跳转",
                    };
                    let btn_color = match self.req_status {
                        ReqStatus::Pending => style::warn_color(ui.visuals().dark_mode),
                        _ => style::primary_color(ui.visuals().dark_mode),
                    };
                    ui.add(Button::new(btn_label).fill(btn_color))
                });
                ui.add(Label::new(
                    RichText::new(&self.warning).color(style::warn_color(ui.visuals().dark_mode)),
                ));
                if btn_response.inner.clicked() {
                    // 点击跳转或取消
                    if let Ok(id) = self.id.trim().parse::<u64>() {
                        match self.req_status {
                            ReqStatus::Pending => {
                                self.req_status = ReqStatus::Idle;
                            }
                            _ => {
                                // TODO: use egui_inbox
                                let query = Query::new(id, self.id_type);
                                self.req_status = ReqStatus::Pending;
                                let tx = self.inbox.sender();
                                query.execute(move |res| {
                                    tx.send(res).ok();
                                });
                            }
                        }
                    } else {
                        self.warning = "无效的ID".to_string();
                    }
                }
                if let Some(last) = self.inbox.read(ui).last() {
                    match last {
                        Ok(result) => {
                            let url = format!(
                                "https://aip.nioint.com/#/issue-scenario/issue/{}/info",
                                result
                            );
                            let open_url = OpenUrl {
                                url: url,
                                new_tab: true,
                            };
                            ui.ctx().open_url(open_url);
                        }
                        Err(err) => {
                            self.warning = err.to_string();
                        }
                    }
                    self.req_status = ReqStatus::Idle;
                }
            },
        );

        ui.allocate_ui_with_layout(
            (0.0, 32.0).into(),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.label("id 类型");
                Frame::new()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(4)
                    .corner_radius(6)
                    .show(ui, |ui| {
                        response_on_change.extend([
                            ui.selectable_value(&mut self.id_type, IdType::Event, "issue/event"),
                            ui.selectable_value(&mut self.id_type, IdType::Scenario, "unique_id"),
                            ui.selectable_value(&mut self.id_type, IdType::Triage, "triage"),
                            ui.selectable_value(&mut self.id_type, IdType::Data, "data"),
                        ]);
                    });
            },
        );
        if response_on_change.iter().any(|r| r.changed()) {
            self.warning = String::new();
        }
    }
}

impl Default for IssueJump {
    fn default() -> Self {
        Self {
            id: String::new(),
            id_type: IdType::Event,
            warning: String::new(),
            inbox: UiInbox::new(),
            req_status: ReqStatus::Idle,
        }
    }
}

struct Query {
    id: u64,
    ty: IdType,
}

impl Query {
    fn new(id: u64, ty: IdType) -> Self {
        Self { id, ty }
    }

    fn execute(&self, on_done: impl 'static + Send + FnOnce(Result<u64, String>)) {
        let query = ListRequest {
            items: vec![FilterItem {
                category: "tags",
                field: self.ty.field(),
                model: self.ty.model(),
                operator: "=",
                symbol: "and",
                ty: "number",
                value: vec![self.id.to_string()],
            }],
            page: 1,
            size: 10,
        };
        let body = serde_json::to_string(&query).unwrap();
        // Execute the query and return the result
        static BASE_URL: &str = "https://issue-prod.nioint.com/api/v1/event/list";
        let request = ehttp::Request::post(BASE_URL, body.as_bytes().to_owned());
        ehttp::fetch(request, move |result| {
            let data = result.and_then(|res| {
                if res.status != 200 {
                    return Err(format!("HTTP error: {}", res.status));
                };
                let response = serde_json::from_slice::<ListResponse>(&res.bytes)
                    .map_err(|e| e.to_string())?;
                if response.data.list.len() == 0 {
                    return Err("未找到对应 issue".to_string());
                }
                if response.code != 200 {
                    return Err(format!("API error: {}", response.message));
                }
                let issue_id = response.data.list[0].event.id;
                Ok(issue_id)
            });
            on_done(data);
        });
    }
}

#[derive(Debug, Serialize)]
struct ListRequest<'a> {
    items: Vec<FilterItem<'a>>,
    page: u32,
    size: u32,
}

#[derive(Debug, Serialize)]
struct FilterItem<'a> {
    category: &'a str,
    field: &'a str,
    model: &'a str,
    operator: &'a str,
    symbol: &'a str,
    #[serde(rename = "type")]
    ty: &'a str,
    value: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ListResponse {
    code: u32,
    data: ListData,
    message: String,
}

#[derive(Debug, Deserialize)]
struct ListData {
    list: Vec<ListItem>,
}

#[derive(Debug, Deserialize)]
struct ListItem {
    event: Event,
}

#[derive(Debug, Deserialize)]
struct Event {
    id: u64,
}
