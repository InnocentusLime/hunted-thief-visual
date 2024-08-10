const TILE_SIZE: f32 = 0.1;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_height: 900,
        window_width: 1600,
        fullscreen: true,
        high_dpi: true,
        sample_count: 2,
        ..Default::default()
    }
}

fn draw_floor_crease(
    crease: &Texture2D,
) {
    let lwmat = Mat4::from_translation(Vec3 { x: -TILE_SIZE - 1.0, y: 0.7, z: 0.5 - TILE_SIZE + 1.0 })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .mul_mat4(&Mat4::from_rotation_z(-std::f32::consts::FRAC_PI_2));
    let rwmat = Mat4::from_translation(Vec3 { x: -TILE_SIZE + 1.0, y: 0.7, z: 0.5 - TILE_SIZE + 1.0 })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .mul_mat4(&Mat4::from_rotation_z(-std::f32::consts::FRAC_PI_2));
    let bwmat = Mat4::from_translation(Vec3 { x: -TILE_SIZE, y: 0.7, z: 0.5 - TILE_SIZE })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2));
    let wall_size = Vec2 { x: 1.0, y: 0.7 };

    /* left crease */
    let p1 = lwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: wall_size.y });
    let p2 = lwmat.transform_point3(Vec3 { x:  wall_size.x, y: 0.0, z: wall_size.y });
    draw_line_3d(p1, p2, BLACK);

    /* back crease */
    let p1 = bwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: wall_size.y });
    let p2 = bwmat.transform_point3(Vec3 { x:  wall_size.x, y: 0.0, z:  wall_size.y });
    draw_line_3d(p1, p2, BLACK);

    /* right crease */
    let p1 = rwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z:  wall_size.y });
    let p2 = rwmat.transform_point3(Vec3 { x:  wall_size.x, y: 0.0, z:  wall_size.y });
    draw_line_3d(p1, p2, BLACK);

    /* rb crease */
    let p1 = bwmat.transform_point3(Vec3 { x: wall_size.x, y: 0.0, z: -wall_size.y });
    let p2 = bwmat.transform_point3(Vec3 { x: wall_size.x, y: 0.0, z:  wall_size.y });
    draw_line_3d(p1, p2, BLACK);

    /* lb crease */
    let p1 = bwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: -wall_size.y });
    let p2 = bwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z:  wall_size.y });
    draw_line_3d(p1, p2, BLACK);

    // /* right corner */
    // let p1 = mat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: -wall_size.y });
    // let p2 = mat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z:  wall_size.y });
    // draw_line_3d(p1, p2, BLACK);
}

fn draw_map(stuff: &[Texture2D], wall: &Texture2D, crease: &Texture2D) {
    let idx = |x: u32, y: u32| ((x << 16) ^ y) % (stuff.len() as u32);

    for y in 0..10 {
        for x in 0..10 {
            let idx = idx(x, y);
            let texture = stuff.get(idx as usize).unwrap();
            draw_plane(
                Vec3 {
                    x: 2.0 * TILE_SIZE * (x as f32) - 1.0,
                    y: 0.0,
                    z: 2.0 * TILE_SIZE * (y as f32) + 0.5,
                },
                Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                },
                Some(texture),
                if x == 0 && y == 0 { RED } else { WHITE }
            );
        }
    }

    let gl = unsafe { get_internal_gl().quad_gl };

    /* Back wall */
    gl.push_model_matrix(
        Mat4::from_translation(Vec3 { x: -TILE_SIZE, y: 0.7, z: 0.5 - TILE_SIZE })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))
    );
    draw_plane(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec2 { x: 1.0, y: 0.7 },
        Some(wall),
        WHITE
    );
    gl.pop_model_matrix();

    /* Left wall */
    gl.push_model_matrix(
        Mat4::from_translation(Vec3 { x: -TILE_SIZE - 1.0, y: 0.7, z: 0.5 - TILE_SIZE + 1.0 })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .mul_mat4(&Mat4::from_rotation_z(-std::f32::consts::FRAC_PI_2))
    );
    draw_plane(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec2 { x: 1.0, y: 0.7 },
        Some(wall),
        WHITE
    );
    gl.pop_model_matrix();

    /* Right wall */
    gl.push_model_matrix(
        Mat4::from_translation(Vec3 { x: -TILE_SIZE + 1.0, y: 0.7, z: 0.5 - TILE_SIZE + 1.0 })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .mul_mat4(&Mat4::from_rotation_z(-std::f32::consts::FRAC_PI_2))
    );
    draw_plane(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec2 { x: 1.0, y: 0.7 },
        Some(wall),
        WHITE
    );
    gl.pop_model_matrix();

    // set_default_camera();
    // draw_texture_ex(
    //     crease,
    //     0.0,
    //     680.0,
    //     WHITE,
    //     DrawTextureParams {
    //         rotation: 40.0 * std::f32::consts::PI / 180.0,
    //         ..Default::default()
    //     }
    // );

    // gl.push_model_matrix(
    //     Mat4::from_translation(Vec3 { x: -1.0*TILE_SIZE - 1.0, y: 0.0, z: 0.5 - TILE_SIZE + 1.0 })
    //         .mul_mat4(&Mat4::from_rotation_y(0.95 * -std::f32::consts::FRAC_PI_2))
    // );
    // draw_plane(
    //     Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    //     Vec2 { x: 1.0, y: 0.04 },
    //     Some(crease),
    //     WHITE
    // );
    // gl.pop_model_matrix();
    draw_floor_crease(crease);
}

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(1600, 900);
    render_target.texture.set_filter(FilterMode::Nearest);

    let floor = [
        load_texture("assets/grass1.png").await.unwrap(),
        load_texture("assets/grass2.png").await.unwrap(),
        load_texture("assets/grass3.png").await.unwrap(),
        load_texture("assets/grass4.png").await.unwrap(),
        load_texture("assets/grass5.png").await.unwrap(),
        load_texture("assets/grass6.png").await.unwrap(),
    ];
    let wall = load_texture("assets/wall.png").await.unwrap();
    let wall_crease = load_texture("assets/wall_crease.png").await.unwrap();

    let material = load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX_SHADER,
            fragment: CRT_FRAGMENT_SHADER,
        },
        MaterialParams::default(),
    )
    .unwrap();

    let mut y = 1.5;
    let mut z = 3.0;
    let mut ty = 0.0;

    loop {
        clear_background(BLACK);


        if is_key_pressed(KeyCode::Q) {
            y += 0.1;
        }
        if is_key_pressed(KeyCode::A) {
            y -= 0.1;
        }
        if is_key_pressed(KeyCode::W) {
            z += 0.1;
        }
        if is_key_pressed(KeyCode::S) {
            z -= 0.1;
        }
        if is_key_pressed(KeyCode::E) {
            ty += 0.1;
        }
        if is_key_pressed(KeyCode::D) {
            ty -= 0.1;
        }

        let mut cam: Camera3D = Default::default();
        cam.aspect = Some(1600.0 / 900.0);
        cam.position = Vec3 { x: 0.0, y, z };
        // cam.position = Vec3 { x: 0.0, y: 1.5, z: 1.5 };
        cam.fovy = 30.0;
        cam.target = Vec3 { x: 0.0, y: ty, z: 0.0 };
        cam.projection = Projection::Perspective;

        let right = Vec3 { x: 0.0, y: 1.0, z: 0.0 }
            .cross(cam.target - cam.position).normalize();
        let up = right.cross(cam.target - cam.position).normalize();

        cam.up = up;

        set_camera(&cam);

        let gl = unsafe { get_internal_gl().quad_gl };
        gl.depth_test(false);

        // set_camera(&Camera3D {
        //     position: vec3(-20., 15., 0.),
        //     up: vec3(0., 1., 0.),
        //     target: vec3(0., 0., 0.),
        //     ..Default::default()
        // });
        // draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), None, WHITE);

        draw_map(&floor, &wall, &wall_crease);
        // drawing to the texture

        // 0..100, 0..100 camera
        // set_camera(&Camera2D {
        //     zoom: vec2(0.01, 0.01),
        //     target: vec2(0.0, 0.0),
        //     render_target: Some(render_target.clone()),
        //     ..Default::default()
        // });

        // clear_background(LIGHTGRAY);
        // draw_line(-30.0, 45.0, 30.0, 45.0, 3.0, BLUE);
        // draw_circle(-45.0, -35.0, 20.0, YELLOW);
        // draw_circle(45.0, -35.0, 20.0, GREEN);

        // // drawing to the screen

        // set_default_camera();

        // clear_background(WHITE);
        // gl_use_material(&material);
        // draw_texture_ex(
        //     &render_target.texture,
        //     0.,
        //     0.,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(screen_width(), screen_height())),
        //         ..Default::default()
        //     },
        // );
        // gl_use_default_material();

        set_default_camera();

        draw_text(
            &format!("y : {y:}"),
            20.0,
            20.0,
            20.0,
            BLUE
        );
        draw_text(
            &format!("z : {z:}"),
            20.0,
            50.0,
            20.0,
            BLUE
        );
        draw_text(
            &format!("ty : {ty:}"),
            20.0,
            80.0,
            20.0,
            BLUE
        );

        next_frame().await;
    }
}

const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

void main() {
    vec2 crtUV = CRTCurveUV(uv);
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    }
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";