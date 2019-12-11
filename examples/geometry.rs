use blinds::traits::*;
use blinds::*;
use golem::{Context, GolemError};
use golem::objects::GeometryType;
use golem::shader::{Attribute, AttributeType, Dimension::{D2, D4}, ShaderDescription};

async fn app(window: Window, ctx: glow::Context, mut events: EventStream) -> Result<(), GolemError> {
    let ctx = Context::from_glow(ctx)?;

    let vertices = [
        // Position         Color
        -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
        0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
        0.5, 0.5,           0.0, 0.0, 1.0, 1.0,
        -0.5, 0.5,          1.0, 1.0, 1.0, 1.0,
    ];
    let indices = [ 0, 1, 1, 2, 2, 3, 3, 0];

    let mut shader = ctx.new_shader(ShaderDescription {
        vertex_input: &[
            Attribute::new("vert_position", AttributeType::Vector(D2)),
            Attribute::new("vert_color", AttributeType::Vector(D4)),
        ],
        fragment_input: &[
            Attribute::new("frag_color", AttributeType::Vector(D4)),
        ],
        uniforms: &[],
        vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
        fragment_shader:
        r#" void main() {
            gl_FragColor = frag_color;
        }"#
    })?;

    let mut vb = ctx.new_vertex_buffer()?;
    let mut eb = ctx.new_element_buffer()?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind(&vb);

    ctx.clear();
    ctx.draw_with_type(&eb, 0..indices.len(), GeometryType::Lines)?;
    window.present();

    while let Some(_) = events.next().await {
    }

    Ok(())
}

fn main() {
    blinds::run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
