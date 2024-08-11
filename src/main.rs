const TILE_SIZE: f32 = 0.1;

use macroquad::{material, prelude::*};
use miniquad::{BlendFactor, BlendState, BlendValue, Equation, UniformDesc};

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

/*
    p1 -- near point
    p2 -- far point
*/
fn draw_crease(
    p1: Vec3,
    p2: Vec3,
    texture: &Texture2D,
    ort: Vec3,
    padding: f32,
    material: &Material,
) {
    let gl = unsafe { get_internal_gl().quad_gl };
    let color = WHITE;
    // let color = RED;

    let v1 = p1 - padding * ort;
    let v2 = p2 - padding * ort;
    let v3 = p2 + padding * ort;
    let v4 = p1 + padding * ort;
    // let p1 = Vec3 { x: p1.x - 0.03, ..p1 };
    // let p2 = Vec3 { x: p2.x - 0.03, ..p2 };

    #[rustfmt::skip]
    let vertices = [
        Vertex::new(v1.x, v1.y, v1.z,  1.0,  1.0, color),
        Vertex::new(v2.x, v2.y, v2.z, 0.0, 1.0, color),
        Vertex::new(v3.x, v3.y, v3.z,  0.0, 0.0, color),
        Vertex::new(v4.x, v4.y, v4.z, 1.0,  0.0, color),
    ];
    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    material.set_uniform("Normal", ort.cross(p2 - p1).normalize_or_zero());

    gl.texture(Some(texture));
    // gl.texture(None);
    gl.draw_mode(DrawMode::Triangles);
    gl.geometry(&vertices, &indices);
}

fn draw_floor_crease(
    crease: &Texture2D,
    material: &Material,
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
    // draw_line_3d(p1, p2, BLACK);
    draw_crease(p1, p2, crease, Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 0.03, material);

    /* back crease */
    let p1 = bwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: wall_size.y });
    let p2 = bwmat.transform_point3(Vec3 { x:  wall_size.x, y: 0.0, z:  wall_size.y });
    // draw_line_3d(p1, p2, RED);
    // let p1 = Vec3
    draw_crease(p1, p2, crease, Vec3 { x: 0.0, y: 0.0, z: 1.0 }, 0.06, material);

    /* right crease */
    let p1 = rwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z:  wall_size.y });
    let p2 = rwmat.transform_point3(Vec3 { x:  wall_size.x, y: 0.0, z:  wall_size.y });
    // draw_line_3d(p1, p2, BLACK);
    draw_crease(p1, p2, crease, Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 0.03, material);

    /* rb crease */
    let p1 = bwmat.transform_point3(Vec3 { x: wall_size.x, y: 0.0, z: -wall_size.y });
    let p2 = bwmat.transform_point3(Vec3 { x: wall_size.x, y: 0.0, z:  wall_size.y });
    // draw_line_3d(p1, p2, BLACK);
    draw_crease(p2, p1, crease, Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 0.03, material);

    /* lb crease */
    let p1 = bwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: -wall_size.y });
    let p2 = bwmat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z:  wall_size.y });
    // draw_line_3d(p1, p2, BLACK);
    draw_crease(p2, p1, crease, Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 0.03, material);

    // /* right corner */
    // let p1 = mat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z: -wall_size.y });
    // let p2 = mat.transform_point3(Vec3 { x: -wall_size.x, y: 0.0, z:  wall_size.y });
    // draw_line_3d(p1, p2, BLACK);
}

fn draw_map(
    stuff: &[Texture2D],
    wall: &Texture2D,
    crease: &Texture2D,
    material: &Material,
) {
    let idx = |x: u32, y: u32| ((x << 16) ^ y) % (stuff.len() as u32);

    material.set_uniform("Normal", vec3(0.0, 1.0, 0.0));
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
    material.set_uniform("Normal", vec3(0.0, 1.0, 0.0));
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
    material.set_uniform("Normal", vec3(0.0, 1.0, 0.0));
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
    material.set_uniform("Normal", vec3(0.0, 1.0, 0.0));
    gl.push_model_matrix(
        Mat4::from_translation(Vec3 { x: -TILE_SIZE + 1.0, y: 0.7, z: 0.5 - TILE_SIZE + 1.0 })
            .mul_mat4(&Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .mul_mat4(&Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2))
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
    draw_floor_crease(crease, material);
}

#[macroquad::main(window_conf)]
async fn main() {
    let pos_buffer = render_target(1600, 900);
    pos_buffer.texture.set_filter(FilterMode::Nearest);

    let light = load_texture("assets/light5.png").await.unwrap();
    let floor = [
        load_texture("assets/grass1.png").await.unwrap(),
        load_texture("assets/grass2.png").await.unwrap(),
        load_texture("assets/grass3.png").await.unwrap(),
        load_texture("assets/grass4.png").await.unwrap(),
        load_texture("assets/grass5.png").await.unwrap(),
        load_texture("assets/grass6.png").await.unwrap(),
    ];
    let wall = load_texture("assets/wall2.png").await.unwrap();
    let wall_crease = load_texture("assets/wall_crease.png").await.unwrap();
    // let wall_crease = load_texture("assets/boxy.png").await.unwrap();

    let material = load_material(
        ShaderSource::Glsl {
            vertex: HT_VERT,
            fragment: HT_FRAG,
        },
        MaterialParams {
            textures: vec![
                "lightTex".to_string()
            ],
            uniforms: vec![
                ("lightPos".to_string(), UniformType::Float3),
                ("Normal".to_string(), UniformType::Float3),
            ],
            pipeline_params: PipelineParams {
                depth_test: Comparison::Never,
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
                ),
                ..Default::default()
            },
            ..Default::default()
        }
    )
    .unwrap();

    let mut y = 1.5;
    let mut z = 3.0;
    let mut ty = 0.0;

    let mut lx = 0.0;
    let mut lz = 1.5;

    loop {
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
        if is_key_down(KeyCode::Left) {
            lx -= 0.1;
        }
        if is_key_down(KeyCode::Right) {
            lx += 0.1;
        }
        if is_key_down(KeyCode::Up) {
            lz -= 0.1;
        }
        if is_key_down(KeyCode::Down) {
            lz += 0.1;
        }

        clear_background(BLACK);
        // clear_background(
        //     Color::new(
        //         72.0 / 255.0 * 0.08,
        //         133.0 / 255.0 * 0.08,
        //         218.0 / 255.0 * 0.08,
        //         1.0
        //     )
        // );

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

        gl_use_material(&material);
        material.set_texture("lightTex", light.clone());
        material.set_uniform("lightPos", vec3(lx, 0.13, lz));

        // set_camera(&Camera3D {
        //     position: vec3(-20., 15., 0.),
        //     up: vec3(0., 1., 0.),
        //     target: vec3(0., 0., 0.),
        //     ..Default::default()
        // });
        // draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), None, WHITE);

        draw_map(&floor, &wall, &wall_crease, &material);
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

        gl_use_default_material();

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
        draw_text(
            &format!("lx : {lx:}"),
            20.0,
            110.0,
            20.0,
            BLUE
        );
        draw_text(
            &format!("lz : {lz:}"),
            20.0,
            140.0,
            20.0,
            BLUE
        );

        next_frame().await;
    }
}

const HT_FRAG: &'static str = r#"#version 100
precision lowp float;
varying lowp vec4 color;
varying lowp vec2 uv;
varying lowp vec3 fragPos;
varying lowp vec3 fragNormal;
varying lowp vec3 fragUp;
varying lowp vec3 fragRight;

uniform lowp vec3 lightPos;
uniform sampler2D Texture;
uniform sampler2D lightTex;

vec4 bigLight() {
    // Params
    float intensityCap = 1.2;

    // Light Infi
    vec3 lightOff = (lightPos - fragPos);
    vec3 lightDir = normalize(lightOff);
    float lightDist = length(lightOff);
    vec3 NfragNormal = normalize(fragNormal);

    // How did we hit the fragment
    float Diff = clamp(dot(NfragNormal, lightDir), 0.0, 1.0);
    vec3 lightOff2N = dot(NfragNormal, lightOff) * NfragNormal;

    // Light Texture
    vec2 lightP = vec2(
        dot(normalize(fragUp) / 3.0, lightOff),
        dot(normalize(fragRight) / 3.0, lightOff)
    ) + vec2(0.5, 0.5);
    lightP.x = clamp(lightP.x, 0.0, 1.0);
    lightP.y = clamp(lightP.y, 0.0, 1.0);
    float lightPix = length(texture2D(lightTex, lightP).xyz);

    // Light Intensity
    float intense = 0.0;
    if (lightDist > 0.0) {
        intense = 1.0 / pow(lightDist, 2.0);
    }
    intense = min(intense, intensityCap);

    // Colors
    vec3 shadow = vec3(72.0 / 255.0, 133.0 / 255.0, 218.0 / 255.0) * 0.3;
    vec4 tex = texture2D(Texture, uv);

    // Final mixing
    float lightLevel = lightPix * intense * Diff;
    vec3 lightTex = tex.xyz * (lightLevel * shadow);
    return vec4(lightTex, tex.w);
}

void main() {
    // float l = length(texture2D(lightTex, lightP).xyz);
    // vec2 ll = vec2(l, length(lightOff.y));
    // float lightDist = length(ll);
    // float dist = 1.0 - clamp(lightDist, 0.0, 1.0);
    gl_FragColor = bigLight();
}"#;

const HT_VERT: &'static str = r#"#version 100
precision lowp float;
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec3 fragPos;
varying lowp vec2 uv;
varying lowp vec4 color;
varying lowp vec3 fragNormal;
varying lowp vec3 fragUp;
varying lowp vec3 fragRight;

uniform vec3 Normal;
uniform mat4 Model;
uniform mat4 Projection;

mat4 inverse(mat4 m) {
  float
      a00 = m[0][0], a01 = m[0][1], a02 = m[0][2], a03 = m[0][3],
      a10 = m[1][0], a11 = m[1][1], a12 = m[1][2], a13 = m[1][3],
      a20 = m[2][0], a21 = m[2][1], a22 = m[2][2], a23 = m[2][3],
      a30 = m[3][0], a31 = m[3][1], a32 = m[3][2], a33 = m[3][3],

      b00 = a00 * a11 - a01 * a10,
      b01 = a00 * a12 - a02 * a10,
      b02 = a00 * a13 - a03 * a10,
      b03 = a01 * a12 - a02 * a11,
      b04 = a01 * a13 - a03 * a11,
      b05 = a02 * a13 - a03 * a12,
      b06 = a20 * a31 - a21 * a30,
      b07 = a20 * a32 - a22 * a30,
      b08 = a20 * a33 - a23 * a30,
      b09 = a21 * a32 - a22 * a31,
      b10 = a21 * a33 - a23 * a31,
      b11 = a22 * a33 - a23 * a32,

      det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

  return mat4(
      a11 * b11 - a12 * b10 + a13 * b09,
      a02 * b10 - a01 * b11 - a03 * b09,
      a31 * b05 - a32 * b04 + a33 * b03,
      a22 * b04 - a21 * b05 - a23 * b03,
      a12 * b08 - a10 * b11 - a13 * b07,
      a00 * b11 - a02 * b08 + a03 * b07,
      a32 * b02 - a30 * b05 - a33 * b01,
      a20 * b05 - a22 * b02 + a23 * b01,
      a10 * b10 - a11 * b08 + a13 * b06,
      a01 * b08 - a00 * b10 - a03 * b06,
      a30 * b04 - a31 * b02 + a33 * b00,
      a21 * b02 - a20 * b04 - a23 * b00,
      a11 * b07 - a10 * b09 - a12 * b06,
      a00 * b09 - a01 * b07 + a02 * b06,
      a31 * b01 - a30 * b03 - a32 * b00,
      a20 * b03 - a21 * b01 + a22 * b00) / det;
}

mat4 transpose(mat4 inMatrix) {
    vec4 i0 = inMatrix[0];
    vec4 i1 = inMatrix[1];
    vec4 i2 = inMatrix[2];
    vec4 i3 = inMatrix[3];

    mat4 outMatrix = mat4(
                 vec4(i0.x, i1.x, i2.x, i3.x),
                 vec4(i0.y, i1.y, i2.y, i3.y),
                 vec4(i0.z, i1.z, i2.z, i3.z),
                 vec4(i0.w, i1.w, i2.w, i3.w)
                 );

    return outMatrix;
}

void main() {
    vec4 worldPos = Model * vec4(position, 1);
    gl_Position = Projection * worldPos;
    fragPos = worldPos.xyz;
    color = color0 / 255.0;
    uv = texcoord;

    mat3 fragTf = mat3(transpose(inverse(Model)));
    fragNormal = fragTf * Normal;
    fragUp = fragTf * vec3(0.0, 0.0, -1.0);
    fragRight = fragTf * vec3(1.0, 0.0, 0.0);
}
"#;

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