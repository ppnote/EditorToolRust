#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod fonts;

use eframe::egui;
use egui::{Event, TextBuffer as _, TextEdit};
use egui::text_edit::TextEditState;

const EDITOR_ID: &str = "main_editor";

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 450.0])
            .with_title("文本编辑器"),
        ..Default::default()
    };

    eframe::run_native(
        "文本编辑器",
        options,
        Box::new(|cc| {
            fonts::setup_cjk_fonts(&cc.egui_ctx);
            Ok(Box::new(EditorApp::default()))
        }),
    )
}

#[derive(Default)]
struct EditorApp {
    text: String,
}

impl EditorApp {
    fn trim_blank_lines(&mut self) {
        self.text = remove_blank_lines(&self.text);
    }

    fn copy_to_clipboard(&self, ctx: &egui::Context) {
        let editor_id = egui::Id::new(EDITOR_ID);

        if let Some(selected) = selected_text(ctx, editor_id, &self.text) {
            ctx.copy_text(selected);
            return;
        }

        if !self.text.is_empty() {
            ctx.copy_text(self.text.clone());
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        sanitize_paste_events(ctx);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("删除空白行").clicked() {
                    self.trim_blank_lines();
                }

                if ui.button("复制").clicked() {
                    self.copy_to_clipboard(ctx);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            let rows = (available.y / row_height).max(1.0) as usize;

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .max_width(available.x)
                .max_height(available.y)
                .id_salt("editor_scroll")
                .show(ui, |ui| {
                    ui.add_sized(
                        available,
                        TextEdit::multiline(&mut self.text)
                            .id(egui::Id::new(EDITOR_ID))
                            .desired_width(f32::INFINITY)
                            .desired_rows(rows)
                            .font(egui::TextStyle::Body),
                    );
                });
        });
    }
}

fn sanitize_paste_events(ctx: &egui::Context) {
    ctx.input_mut(|input| {
        for event in &mut input.events {
            if let Event::Paste(text) = event {
                *text = remove_blank_lines(text);
            }
        }
    });
}

fn selected_text(ctx: &egui::Context, editor_id: egui::Id, text: &str) -> Option<String> {
    let state = TextEditState::load(ctx, editor_id)?;
    let range = state.cursor.char_range()?;
    let [start, end] = range.sorted();
    let char_range = start.index..end.index;

    if char_range.is_empty() {
        None
    } else {
        Some(text.char_range(char_range).to_string())
    }
}

pub fn remove_blank_lines(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    text.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::remove_blank_lines;

    #[test]
    fn removes_blank_lines() {
        let input = "a\n\n  \n\t\nb\r\n\r\nc";
        assert_eq!(remove_blank_lines(input), "a\nb\nc");
    }

    #[test]
    fn empty_input_stays_empty() {
        assert_eq!(remove_blank_lines(""), "");
    }
}
