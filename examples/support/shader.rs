pub const VERTEX: &'static [u8] = br###"
#version 150

in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}
"###;

pub const FRAGMENT: &'static [u8] = br###"
#version 150

out vec4 outColor;

void main()
{
    outColor = vec4(0.0, 0.0, 0.0, 1.0);
}
"###;
