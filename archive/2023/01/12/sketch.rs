use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const RING_COUNT: usize = 15;

struct Model {
    _window: window::Id,
    _size: f32,
    _ring_angles: [f32; RING_COUNT],
    _ring_speeds: [f32; RING_COUNT],
    _ring_sizes: [f32; RING_COUNT],
    _ring_dots: [usize; RING_COUNT],
}

fn ring_start_angle(index: usize) -> f32 {
    (index as f32 + 0.5) * 360. / (RING_COUNT as f32)
}

fn ring_angle(index: usize, seconds: f32, rps: f32) -> f32 {
    ring_start_angle(index) + seconds * rps * 2. * 3.141592 /* REPLACE THIS */
}

fn generate_size(app: &App) -> f32 {
    let stage = app.window_rect();
    stage.w().min(stage.h()) as f32
}

fn generate_ring_angles() -> [f32; RING_COUNT] {
    let mut _ring_angles = [0.; RING_COUNT];
    for i in 0..RING_COUNT {
        _ring_angles[i] = ring_start_angle(i);
    }
    _ring_angles
}

fn generate_ring_speeds() -> [f32; RING_COUNT] {
    let mut _ring_speeds = [0.; RING_COUNT];
    for i in 0..RING_COUNT {
        _ring_speeds[i] = ((RING_COUNT - i) as f32 / RING_COUNT as f32).powf(2.);
    }
    _ring_speeds
}

fn generate_ring_sizes(scope_size: f32) -> [f32; RING_COUNT] {
    let mut _ring_sizes = [0.; RING_COUNT];
    for i in 0..RING_COUNT {
        _ring_sizes[i] = (i as f32 + 0.5) * scope_size / RING_COUNT as f32 / 2.;
    }
    _ring_sizes
}

fn generate_ring_dots() -> [usize; RING_COUNT] {
    let mut ring_dots = [0; RING_COUNT];
    for i in 0..RING_COUNT {
        ring_dots[i] = 5; //nth_prime(i + 1);
    }
    ring_dots
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let _size = generate_size(app);
    let _ring_angles = generate_ring_angles();
    let _ring_speeds = generate_ring_speeds();
    let _ring_sizes = generate_ring_sizes(_size);
    let _ring_dots = generate_ring_dots();

    Model {
        _window,
        _size,
        _ring_angles,
        _ring_speeds,
        _ring_sizes,
        _ring_dots,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for i in 0..RING_COUNT {
        _model._ring_angles[i] =
            ring_angle(i, _update.since_start.as_secs_f32(), _model._ring_speeds[i]);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let background_color = hsl(0., 0., 0.05);
    let foreground_color = hsl(0., 0., 0.95);

    let draw = app.draw();

    draw.background().color(background_color);

    for i in 0..RING_COUNT {
        let dot_count = _model._ring_dots[i];
        for j in 0..dot_count {
            draw.ellipse()
                .color(foreground_color)
                .x(_model._ring_sizes[i]
                    * (_model._ring_angles[i]
                        + j as f32 * 2. * 3.141592f32 /* REPLACE THIS */ / (dot_count as f32))
                        .cos())
                .y(_model._ring_sizes[i]
                    * (_model._ring_angles[i]
                        + j as f32 * 2. * 3.141592f32 /* REPLACE THIS */ / (dot_count as f32))
                        .sin())
                .w_h(
                    _model._size / (RING_COUNT as f32) / 2. / 4.,
                    _model._size / (RING_COUNT as f32) / 2. / 4.,
                );
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn nth_prime(n: usize) -> usize {
    let mut primes = Vec::new();
    let mut num = 2;

    while primes.len() < n as usize {
        let mut is_prime = true;

        for prime in &primes {
            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(num);
        }

        num += 1;
    }

    *primes.last().unwrap()
}
