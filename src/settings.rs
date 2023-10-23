pub mod settings {    
    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

    use eframe::egui;
    use cpal::traits::{DeviceTrait, HostTrait};
    use super::super::audio;

    pub fn settings_window() -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(420.0, 180.0)),
            resizable: false,
            ..Default::default()
        };
        eframe::run_native(
            "Cleep",
            options,
            Box::new(|cc| {
                // This gives us image support:
                egui_extras::install_image_loaders(&cc.egui_ctx);

                Box::<SettingsWindow>::default()
            }),
        )
    }

    struct SettingsWindow {
        input_selected: usize,
        inputs_avail: Vec<String>,
        output_selected: usize,
        outputs_avail: Vec<String>,
    }

    impl Default for SettingsWindow {
        fn default() -> Self {
            Self {
                input_selected: 0,
                inputs_avail: get_input_vec(),
                output_selected: 0,
                outputs_avail: get_output_vec(),
            }
        }
    }

    fn get_input_vec() -> Vec<String> {
        let host = cpal::default_host();
        let devices: Vec<(usize, cpal::Device)> = host.input_devices().unwrap().enumerate().collect();
        let mut vecs = Vec::new();

        for (i, d) in &devices {
            vecs.push(d.name().unwrap());
        }

        return vecs;
    }

    fn get_output_vec() -> Vec<String> {
        let host = cpal::default_host();
        let devices: Vec<(usize, cpal::Device)> = host.output_devices().unwrap().enumerate().collect();
        let mut vecs = Vec::new();

        for (i, d) in &devices {
            vecs.push(d.name().unwrap());
        }

        return vecs;
    }

    impl eframe::App for SettingsWindow {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(" ");

                egui::Grid::new("device selection").show(ui, |ui| {
                    ui.label("Microphone");
                    egui::ComboBox::new("inputs", "")
                        .selected_text(format!("{}", &self.inputs_avail[self.input_selected]))
                        .show_ui(ui, |ui| {
                        for i in 0..self.inputs_avail.len() {
                            let value = ui.selectable_value(&mut &self.inputs_avail[i], &self.inputs_avail[self.input_selected], &self.inputs_avail[i]);
                            if value.clicked() {
                                self.input_selected = i;
                            }
                        }
                    });
                    ui.end_row();
                
                    ui.label("Speakers");
                    egui::ComboBox::new("outputs", "")
                        .selected_text(format!("{}", &self.outputs_avail[self.output_selected]))
                        .show_ui(ui, |ui| {
                        for i in 0..self.outputs_avail.len() {
                            let value = ui.selectable_value(&mut &self.outputs_avail[i], &self.outputs_avail[self.output_selected], &self.outputs_avail[i]);
                            if value.clicked() {
                                self.output_selected = i;
                            }
                        }
                    });
                    ui.end_row();
                });

                if ui.add(egui::Button::new("record")).clicked() {
                    let asd = audio::audio::AudioRecorder::new(cpal::default_host(), self.input_selected, self.output_selected);
                    asd.record();
                }

                //ui.image(egui::include_image!(
                //    "../../../crates/egui/assets/ferris.png"
                //));
            });
        }
    }
}