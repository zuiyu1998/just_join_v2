use bevy::prelude::{Handle, Image, Resource};
use bevy_egui::egui;

use crate::staff::Staff;

use super::tool_box::tool_box;

#[derive(Debug)]
pub struct ToolBox {
    pub staff: Option<Staff>,
    pub num: usize,
    pub active: bool,
}

impl Default for ToolBox {
    fn default() -> Self {
        Self {
            staff: None,
            num: 0,
            active: false,
        }
    }
}

#[derive(Debug, Resource, Default)]
pub struct ToolBar {
    pub tools: [ToolBox; 10],
    pub active_index: usize,
}

impl ToolBar {
    pub fn active(&mut self, index: usize) {
        self.active_index = index;
        for i in 0..=9 {
            if i as usize == index {
                self.tools[i].active = true;
            } else {
                self.tools[i].active = false;
            }
        }
    }
}

pub fn tool_bar(
    ui: &mut egui::Ui,
    toolbar: &mut ToolBar,
    mut get_texture_egui: impl FnMut(&Handle<Image>) -> Option<egui::TextureId>,
    tool_box_border: Option<egui::TextureId>,
) {
    ui.horizontal_centered(|ui| {
        for index in 0..=9 {
            let tool_box_data = &mut toolbar.tools[index as usize];
            let tool_box_item = tool_box(
                ui,
                &mut tool_box_data.active,
                &mut tool_box_data.num,
                if let Some(data) = tool_box_data.staff.clone() {
                    get_texture_egui(&data.icon.clone())
                } else {
                    None
                },
                tool_box_border,
            );
            if tool_box_item.clicked() {
                toolbar.active(index as usize);
            }
        }
    });
}
