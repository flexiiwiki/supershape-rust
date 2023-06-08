use nannou::prelude::*;

//TOTAL VERTEX COUNT PER LATITUDE
//i still get like 20-30 fps when this is at 500 but ymmv
const TOTAL: usize = 100;

struct Model {
    globe_colored: Vec<(Vec3, Hsl)>,
    globe_indices: Vec<usize>,
    #[allow(dead_code)]
    update_count: usize,
    m: f32,
    fps: f32,
    hue_offset: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .always_on_top(true)
        .size(512, 512)
        .view(view)
        .build()
        .unwrap();
    app.set_exit_on_escape(false);
    app.new_window()
        .size(100, 50)
        .always_on_top(true)
        .view(view_two)
        .build()
        .unwrap();

    Model {
        globe_indices: Vec::new(),
        globe_colored: Vec::new(),
        update_count: 0,
        m: 7.0,
        fps: 0.0,
        hue_offset: 0.0,
    }
}

fn view_two(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.text(&format!["FPS: {}\n ", model.fps.round()])
        .font_size(10)
        .x_y(0.0, 0.0);

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // framework fps limitations doesnt work, so this is if you want to limit the update time and manually lock framerate
    //if (model.update_count % 10) != 0 {
    //    model.update_count += 1;
    //    return
    //} else {
    //    model.update_count = 0;
    //}
    model.fps = 1.0 / update.since_last.as_secs_f32();

    let mut globe = vec![vec![vec3(0.0, 0.0, 0.0); TOTAL]; TOTAL];

    let mut globe_indices = Vec::with_capacity(TOTAL);

    globe.iter_mut().enumerate().for_each(|(i, outer)| {
        let lat = map_range(i, 0, TOTAL - 1, -PI / 2.0, PI / 2.0);
        let r2 = supershape(lat, model.m, 0.2, 1.7, 1.7);
        outer.iter_mut().enumerate().for_each(|(j, inner)| {
            let radius = 150.0;
            let lon = map_range(j, 0, TOTAL - 1, -PI, PI);
            let r1 = supershape(lon, model.m, 0.2, 1.70, 1.70);
            let x = radius * r1 * lon.cos() * r2 * lat.cos();
            let y = radius * r1 * lon.sin() * r2 * lat.cos();
            let z = radius * r2 * lat.sin();
            *inner = vec3(x, y, z);
            if j > 0 && i > 0 {
                let cursor = j + (i * TOTAL);
                let back_one = cursor - 1;
                let down_back = cursor + TOTAL - 1;
                let down = cursor + TOTAL;
                globe_indices.push(back_one);
                globe_indices.push(cursor);
                globe_indices.push(down_back);
                globe_indices.push(down);
                globe_indices.push(down_back);
                globe_indices.push(cursor);
                if back_one % TOTAL == 0 && back_one != 0 {
                    globe_indices.push(back_one);
                    globe_indices.push(back_one - 1);
                    globe_indices.push(down_back);
                    globe_indices.push(down_back - 1);
                    globe_indices.push(back_one - 1);
                    globe_indices.push(down_back);
                }
            }
        })
    });

    let mut globe_colored = vec![vec![(vec3(0.0, 0.0, 0.0), hsl(0.0, 0.0, 0.0)); TOTAL]; TOTAL];

    globe.iter().enumerate().for_each(|(i, outer)| {
        outer.iter().enumerate().for_each(|(j, inner)| {
            let hue = map_range(i, 0, TOTAL, 0.0, 1.0);
            let color = hsl(hue + model.hue_offset, 0.5, 0.5);

            globe_colored[i][j] = (*inner, color);
        })
    });
    model.globe_indices = globe_indices;
    model.globe_colored = globe_colored.into_iter().flatten().collect();

    model.hue_offset += 0.01;
    if model.hue_offset >= 1.0 {
        model.hue_offset = 0.0;
    }
    if model.m > 14.0 {
        model.m = -14.0;
    }
    if model.m > -2.0 && model.m < 2.0 {
        model.m = 2.0
    } else {
        model.m += 0.03;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let rotation =
        Quat::from_rotation_y(app.time.sin()) * Quat::from_rotation_x(app.time.sin() * 2.0);

    // draw with points (cannot do at same time as model due to framework limitations with indices)
    //for &pt in model.globe_colored.iter() {
    //    draw.ellipse()
    //        .radius(1.0)
    //        .color(WHITE)
    //        .xyz(rotation.mul_vec3(pt.0));
    //}

    draw.mesh()
        .indexed_colored(model.globe_colored.clone(), model.globe_indices.clone())
        .quaternion(rotation);

    draw.to_frame(app, &frame).unwrap();
}

fn supershape(theta: f32, m: f32, n1: f32, n2: f32, n3: f32) -> f32 {
    let a = 1.0;
    let b = 1.0;

    let t1 = ((1.0 / a) * (m * theta / 4.0).cos()).abs().powf(n2);
    let t2 = ((1.0 / b) * (m * theta / 4.0).sin()).abs().powf(n3);
    (t1 + t2).powf(-1.0 / n1)
}
