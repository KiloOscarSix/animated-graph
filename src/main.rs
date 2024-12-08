use csv::ReaderBuilder;
use ggez::graphics::{Canvas, Color, DrawParam, Image, ImageFormat};
use ggez::{event, Context, ContextBuilder, GameResult};
use image::{ImageBuffer, RgbImage};
use plotters::prelude::*;
use std::time::Instant;

const DRAW_TIME: f64 = 5.0;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct AppState {
    data_points: Vec<(u64, f64)>,
    visible_points: usize,
    start_time: Instant,
}

impl AppState {
    fn new(data_points: Vec<(u64, f64)>) -> Self {
        AppState {
            data_points,
            visible_points: 0,
            start_time: Instant::now(),
        }
    }
}

impl event::EventHandler for AppState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update the visible points based on elapsed time
        let elapsed = self.start_time.elapsed().as_secs_f64();
        self.visible_points = ((elapsed / DRAW_TIME) * self.data_points.len() as f64) as usize;
        if self.visible_points > self.data_points.len() {
            self.visible_points = self.data_points.len();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        let mut buffer = vec![0; WIDTH as usize * HEIGHT as usize * 4]; // 800x600 image, RGBA

        {
            let root = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .caption("Animated Graph", ("sans-serif", 30))
                .margin(10)
                .x_label_area_size(50)
                .y_label_area_size(30)
                .build_cartesian_2d(
                    0..self.data_points.last().unwrap().0,
                    0.0..self.data_points.last().unwrap().1,
                )
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(
                    self.data_points[..self.visible_points].iter().cloned(),
                    &BLUE,
                ))
                .unwrap();

            root.present().unwrap();
        }

        // let img: RgbImage = ImageBuffer::from_raw(WIDTH, HEIGHT, buffer).unwrap();
        // img.save("output.png").unwrap();

        let image = Image::from_pixels(ctx, &buffer, ImageFormat::Rgba8Unorm, WIDTH, HEIGHT);
        canvas.draw(&image, DrawParam::new());

        canvas.finish(ctx)
    }
}

// fn graph()

fn load_csv(file_path: &str) -> Vec<(u64, f64)> {
    let mut reader = ReaderBuilder::new()
        .from_path(file_path)
        .expect("Failed to read CSV");

    reader
        .records()
        .map(|r| r.expect("Failed to parse record"))
        .filter(|r| !r[1].is_empty())
        .map(|r| {
            let x: u64 = r[0].parse().expect("Invalid x value");
            let y: f64 = r[1].parse().expect("Invalid y value");
            (x, y)
        })
        .collect()
}

fn main() {
    let csv_path = "data.csv";
    let data_points = load_csv(csv_path);

    let (ctx, event_loop) = ContextBuilder::new("animated_graph", "Author")
        .build()
        .expect("Failed to build ggez context");

    let app_state = AppState::new(data_points);

    event::run(ctx, event_loop, app_state);
}
