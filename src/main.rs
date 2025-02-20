use charming::{
    component::{Axis, Legend, Title},
    element::{ItemStyle, Padding, TextStyle},
    series::Scatter,
    theme::Theme,
    Chart, ImageFormat, ImageRenderer,
};

fn main() {
    let mut s = 2000 as f64;
    let mut i = 1 as f64;
    let mut r = 0 as f64;

    let c = 0.0004;
    let g = 0.2;
    let dt = 0.1;

    let mut sim_data_s = Vec::new();
    let mut sim_data_i = Vec::new();
    let mut sim_data_r = Vec::new();
    let mut t = 0 as f64;
    while t < 100.0 {
        let s_new = s - c * s * i * dt;
        let i_new = i + c * s * i * dt - g * i * dt;
        let r_new = r + g * i * dt;

        sim_data_s.push(vec![t, s]);
        sim_data_i.push(vec![t, i]);
        sim_data_r.push(vec![t, r]);

        s = s_new;
        i = i_new;
        r = r_new;

        t += dt;
    }

    plot(sim_data_s, sim_data_i, sim_data_r, "output.png".to_string());
}

fn plot(
    sim_data_1: Vec<Vec<f64>>,
    sim_data_2: Vec<Vec<f64>>,
    sim_data_3: Vec<Vec<f64>>,
    out_path: String,
) {
    let chart = Chart::new()
        .title(
            Title::new()
                .text("Simulation Output")
                .text_style(TextStyle::new().font_size(32).font_weight("bold"))
                .padding(Padding::Double(10.0, 100.0)),
        )
        .x_axis(Axis::new())
        .y_axis(Axis::new())
        .legend(Legend::new().data(vec!["S", "I", "R"]))
        .series(
            Scatter::new()
                .symbol_size(4)
                .name("S")
                .item_style(ItemStyle::new().opacity(0.5))
                .data(sim_data_1),
        )
        .series(
            Scatter::new()
                .symbol_size(4)
                .name("I")
                .item_style(ItemStyle::new().opacity(0.5))
                .data(sim_data_2),
        )
        .series(
            Scatter::new()
                .symbol_size(4)
                .name("R")
                .item_style(ItemStyle::new().opacity(0.5))
                .data(sim_data_3),
        );

    ImageRenderer::new(1080, 720)
        .theme(Theme::PurplePassion)
        .save_format(ImageFormat::Png, &chart, out_path)
        .expect("Failed to save image");
}
