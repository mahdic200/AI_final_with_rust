use plotters::prelude::*;
pub fn draw(points: Vec<(f32, f32)>, maxiter: f32) -> Result<(), Box<dyn std::error::Error>> {
    let mut width = 680;
    if ((maxiter * 3.0) as u32) < 1220 {
        width = 1220;
    }
    let root = BitMapBackend::new("genetic_plot.png", (width as u32, 680)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-2.0f32..1000f32, -0.1f32..1.0f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            points,
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}