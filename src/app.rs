use chrono::DateTime;
use chrono::Local;
use eframe::App;
use eframe::CreationContext;
use eframe::Frame;
use egui::Align;
use egui::CentralPanel;
use egui::Color32;
use egui::Context;
use egui::FontData;
use egui::FontDefinitions;
use egui::FontFamily;
use egui::FontId;
use egui::Layout;
use egui::RichText;
use egui::Stroke;
use egui::Visuals;

/// The actual app.
#[derive(Default)]
pub struct CountdownApp;

impl CountdownApp {
    fn __load_fonts(fonts: &mut FontDefinitions, new_fonts: Vec<(&str, &[u8])>) {
        for (name, data) in new_fonts {
            fonts
                .font_data
                .insert(name.to_owned(), FontData::from_owned(data.to_vec()));

            // Insert this font into the family with the name, `name`
            fonts
                .families
                .entry(FontFamily::Name(name.into()))
                .or_default()
                .insert(0usize, name.to_owned());
        }
    }

    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        // Load our fonts here
        Self::__load_fonts(
            &mut fonts,
            vec![
                (
                    "Lucotecia Bold",
                    include_bytes!("../assets/Lucotecia Bold.otf"),
                ),
                ("Lucotecia", include_bytes!("../assets/Lucotecia.otf")),
            ],
        );

        cc.egui_ctx.set_fonts(fonts);

        let mut visuals = Visuals::dark();
        visuals.override_text_color = Some(Color32::WHITE);
        visuals.panel_fill = Color32::BLACK;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(2.0, Color32::RED);

        cc.egui_ctx.set_visuals(visuals);

        Self::default()
    }
}

impl App for CountdownApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.separator();

                ui.label(RichText::new("Kerbal Space Program 2").font(FontId::new(
                    80.0f32,
                    FontFamily::Name("Lucotecia Bold".into()),
                )));

                ui.separator();

                ui.label(RichText::new("Releases in...").font(FontId::new(
                    40.0f32,
                    FontFamily::Name("Lucotecia Bold".into()),
                )));

                ui.centered_and_justified(|ui| {
                    let release =
                        DateTime::parse_from_rfc3339("2023-02-24T14:00:00+00:00").unwrap();

                    let time_until = release.signed_duration_since(Local::now());

                    let seconds = time_until.num_seconds() % 60i64;
                    let minutes = (time_until.num_seconds() / 60i64) % 60i64;
                    let hours = ((time_until.num_seconds() / 60i64) / 60i64) % 24i64;
                    let days = time_until.num_days();

                    let text = if time_until.num_seconds() > 0i64 {
                        format!("{days} days {hours} hours {minutes} minutes {seconds} seconds")
                    } else {
                        "It's already out!! Go play it (or don't)!".to_owned()
                    };

                    ui.label(
                        RichText::new(text)
                            .font(FontId::new(40.0f32, FontFamily::Name("Lucotecia".into()))),
                    );
                });

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    ui.colored_label(Color32::from_rgb(4u8, 4u8, 4u8), "trans rights");
                });
            });
        });

        ctx.request_repaint();
    }
}
