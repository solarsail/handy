use eframe::egui::{Align, Button, Frame, Layout, Ui, output::OpenUrl};

use crate::style;

#[derive(PartialEq, Eq)]
pub enum GraphType {
    ReproduceTask,
    RetestTask,
    ReproduceResult,
    RetestResult,
    TaskTrigger,
}

impl GraphType {
    fn as_str(&self) -> &'static str {
        match self {
            GraphType::ReproduceTask => "task_reproduce",
            GraphType::RetestTask => "task_retest",
            GraphType::ReproduceResult => "result_reproduce",
            GraphType::RetestResult => "result_retest",
            GraphType::TaskTrigger => "trigger_task",
        }
    }
}

pub struct TaskGraphJump {
    graph_type: GraphType,
    exec_id: String,
}

impl super::ToolItem for TaskGraphJump {
    fn name(&self) -> &str {
        "任务图快速跳转"
    }

    fn description(&self) -> &str {
        "通过 id 快速打开任务图页面"
    }

    fn update(&mut self, ui: &mut Ui) {
        ui.allocate_ui_with_layout(
            (300.0, 32.0).into(),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.label("ID");
                ui.text_edit_singleline(&mut self.exec_id);
                let btn_response = ui.scope(|ui| {
                    ui.spacing_mut().button_padding = (8.0, 4.0).into();
                    ui.add(Button::new("→ 跳转").fill(style::primary_color(ui.visuals().dark_mode)))
                });
                if btn_response.inner.clicked() {
                    let url = format!(
                        "https://aip.nioint.com/api/issue-sim-beta/v1/taskgraph/render?type={}&exec_id={}&redirect=true",
                        self.graph_type.as_str(),
                        self.exec_id
                    );
                    let open_url = OpenUrl {
                        url: url,
                        new_tab: true,
                    };
                    ui.ctx().open_url(open_url);
                }
            }
        );
        ui.allocate_ui_with_layout(
            (0.0, 32.0).into(),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.label("流程类型");
                Frame::new()
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .inner_margin(4)
                    .corner_radius(6)
                    .show(ui, |ui| {
                        ui.selectable_value(
                            &mut self.graph_type,
                            GraphType::ReproduceTask,
                            "复现任务",
                        );
                        ui.selectable_value(
                            &mut self.graph_type,
                            GraphType::RetestTask,
                            "复测任务",
                        );
                        ui.selectable_value(
                            &mut self.graph_type,
                            GraphType::ReproduceResult,
                            "复现结果回流",
                        );
                        ui.selectable_value(
                            &mut self.graph_type,
                            GraphType::RetestTask,
                            "复测结果回流",
                        );
                        ui.selectable_value(
                            &mut self.graph_type,
                            GraphType::TaskTrigger,
                            "触发任务",
                        );
                    });
            },
        );
    }
}

impl Default for TaskGraphJump {
    fn default() -> Self {
        Self {
            graph_type: GraphType::ReproduceTask,
            exec_id: String::new(),
        }
    }
}
