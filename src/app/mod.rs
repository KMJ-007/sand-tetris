mod particle;
use lazy_static::lazy_static;

const SCALE: usize = 100;
const GRAVITY: f64 = 0.9;
lazy_static! {
    static ref WINDOW_SIZE: [usize; 2] = [600, 600];
    static ref GRID_SIZE: [usize; 2] = [WINDOW_SIZE[0] / SCALE, WINDOW_SIZE[1] / SCALE];
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SandTetrisApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    counter: i32,
}

impl Default for SandTetrisApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello Karan! what are you doing".to_owned(),
            value: 2.7,
            counter: 0,
        }
    }
}

impl SandTetrisApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SandTetrisApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
          // grid
          egui::Grid::new("grid").show(ui, |ui| {
            // make matrix grid
            for _ in 0..GRID_SIZE[0]{
                // draw 3x3 grid
                for _ in 0..GRID_SIZE[1] {
                    ui.label("Hello Karan!");
                }
                    ui.end_row();
                }
            });
        });
    }
}
