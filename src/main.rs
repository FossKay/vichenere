use eframe::egui;

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";
const ABC_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ABC_LEN: usize = 26;

fn index_abc(x: char) -> usize {
    ABC.chars().position(|n| n == x).unwrap()
}
fn get_abc(x: usize) -> char {
    ABC.chars().nth(x).unwrap()
}

fn index_abc_upper(x: char) -> usize {
    ABC_UPPER.chars().position(|n| n == x).unwrap()
}
fn get_abc_upper(x: usize) -> char {
    ABC_UPPER.chars().nth(x).unwrap()
}

fn encrypt(plain: &str, key: &str) -> String {
    let mut res = String::new();
    let mut i = 0;
    for char in plain.chars() {
        if ABC_UPPER.contains(char) {
            let key_char = key.chars().nth(i).unwrap();
            i += 1;
            i %= key.len();
            res.push(get_abc_upper((index_abc_upper(char) + index_abc(key_char)) % ABC_LEN));
        } else if ABC.contains(char) {
            let key_char = key.chars().nth(i).unwrap();
            i += 1;
            i %= key.len();
            res.push(get_abc((index_abc(char) + index_abc(key_char)) % ABC_LEN));
        } else {
            res.push(char);
        }
    }
    res
}

fn decrypt(plain: &str, key: &str) -> String {
    let mut res = String::new();
    let mut i = 0;
    for char in plain.chars() {
        if ABC_UPPER.contains(char) {
            let key_char = key.chars().nth(i).unwrap();
            i += 1;
            i %= key.len();
            res.push(get_abc_upper((index_abc_upper(char) - index_abc(key_char) + 26) % ABC_LEN));
        } else if ABC.contains(char) {
            let key_char = key.chars().nth(i).unwrap();
            i += 1;
            i %= key.len();
            res.push(get_abc((index_abc(char) - index_abc(key_char) + 26) % ABC_LEN));
        } else {
            res.push(char);
        }
    }
    res
}

#[derive(Default)]
struct VigenereApp {
    text: String,
    key: String,
    result: String,
}

impl eframe::App for VigenereApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(20.0, 20.0);
        style.spacing.button_padding = egui::vec2(15.0, 15.0);
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::proportional(20.0),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::proportional(20.0),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::proportional(25.0),
        );
        style.visuals = egui::Visuals::dark();
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(12);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(12);
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.heading("Chiffre de Vigenère");

                ui.add(
                    egui::TextEdit::singleline(&mut self.text)
                        .hint_text("Message")
                        .desired_width(f32::INFINITY)
                        .margin(egui::vec2(10.0, 10.0))
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.key)
                        .hint_text("Clé")
                        .desired_width(f32::INFINITY)
                        .margin(egui::vec2(10.0, 10.0))
                );

                ui.horizontal(|ui| {
                    let enabled = !self.key.is_empty() && !self.text.is_empty();

                    if ui
                        .add_enabled(
                            enabled,
                            egui::Button::new("Encrypter")
                        )
                        .clicked()
                    {
                        self.result = encrypt(&self.text, &self.key);
                    }

                    if ui
                        .add_enabled(
                            enabled,
                            egui::Button::new("Décrypter")
                        )
                        .clicked()
                    {
                        self.result = decrypt(&self.text, &self.key);
                    }
                });

                if !self.result.is_empty() {
                    ui.label(&self.result);
                }
            });
    }
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 400.0])
            .with_min_inner_size([400.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Vigenère",
        options,
        Box::new(|_| Ok(Box::<VigenereApp>::default())),
    )
}
