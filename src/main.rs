use names::Generator;
use nannou::{
    glam::Vec2,
    noise::{BasicMulti, NoiseFn, Perlin, Seedable},
    prelude::*,
};

const NOISE_STEP: f32 = 500.0;

struct Model {
    noise: BasicMulti,
    points: Vec<Vec2>,
    frame_start: u64,
}
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1200, 630)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    Model {
        noise: BasicMulti::new(),
        points: vec![],
        frame_start: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window_rect();

    let currnt_step = (app.elapsed_frames() - model.frame_start) as f32 / NOISE_STEP;
    // O zero como segundo paramentro força esse modelo a ser de 1D
    let y = model.noise.get([currnt_step.into(), 0.]);

    let mapped_y = map_range(y, -1.0, 1.0, win_rect.top(), win_rect.bottom());
    model.points.push(pt2(currnt_step, mapped_y));
}

// Controle da forma que vai ser renderido o grafico

fn view(app: &App, model: &Model, frame: Frame) {
    let background = rgb(0.439, 0.039, 0.467);
    let foreground = rgb(0.855, 0.310, 0.671);

    // Definido a janela
    let win_rect = app.window_rect();
    let win_p = win_rect.pad(25.0); // windowns padding

    let draw = app.draw();
    draw.background().color(background);

    // Linha central com o eixo X = 0
    draw.line()
        .start(Vec2::new(win_rect.left(), 0.0))
        .end(Vec2::new(win_rect.right(), 0.0))
        .color(rgb(0.255, 0.020, 0.275));

    // Noise line
    draw.polyline()
        .x(0 as f32 - model.points.len() as f32)
        .weight(1.0)
        .points(
            model
                .points
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, mut point)| {
                    point.x = index as f32;
                    point
                })
                .collect::<Vec<Vec2>>(),
        )
        .color(foreground);

    // Ponto que indica a posição atual do ruido
    draw.ellipse()
        .x(0.0)
        .y(model.points.iter().last().unwrap().y)
        .w_h(10.0, 10.0)
        .color(foreground);

    // Desenha a linha horizontal
    for i in 0..(app.elapsed_frames() / 500) {
        draw.line()
            .start(Vec2::new(
                (i * 500 - model.points.len() as u64) as f32,
                win_rect.top(),
            ))
            .end(Vec2::new(
                (i * 500 - model.points.len() as u64) as f32,
                win_rect.bottom(),
            ));
    }

    // Mostrar a seed atual
    let seed = model.noise.seed();
    let seed_bytes = seed.to_be_bytes();
    let seed_display = std::str::from_utf8(&seed_bytes).unwrap();

    draw.text(&format!("{}", seed_display))
        .font_size(40)
        .wh(win_p.wh())
        .right_justify()
        .align_text_bottom()
        .color(foreground);

    // Desenhe o frame
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    let now = chrono::offset::Local::now();

    if key == Key::S {
        app.main_window()
            .capture_frame(format!("{}{}{}", app.exe_name().unwrap(), now, ".png"));
    }
}
fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    // Gera uma nova seed utilizando o AscII characters
    let mut generator = Generator::default();
    let seed = generator.next().unwrap();
    let mut bytes: [u8; 4] = [0; 4];

    for (i, byte) in seed.as_bytes().iter().take(4).enumerate() {
        bytes[i] = *byte;

    }

    let seed_number:u32 =  u32::from_be_bytes(bytes);
   
    // inicia um novo perlin noise
    let noise = BasicMulti::new();
    let seeded_noise = noise.set_seed(seed_number);

    // Defini um novo valor para o modelo
    model.noise = seeded_noise;
    
    // Limpa os dados antigos
    model.frame_start = app.elapsed_frames();

    // limpa o vector para evitar que antigos valores sejam utilizados na nova seed
    model.points = vec![];
}


fn main() {
    nannou::app(model).update(update).run();
    
}
