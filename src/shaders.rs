pub static VS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;

void main()
{
    gl_Position = pos;
}
";

pub static VS_COLORED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

in vec4 pos;

void main()
{
    gl_Position = pos;
}
";

pub static FS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

void main()
{
    gl_FragColor = color;
}
";

pub static FS_COLORED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

out vec4 out_color;

void main()
{
    out_color = color;
}
";

pub static VS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;
attribute vec2 uv;

uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    v_uv = uv;
    gl_Position = pos;
}
";

pub static VS_TEXTURED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;

in vec4 pos;
in vec2 uv;

uniform sampler2D s_texture;

out vec2 v_uv;

void main()
{
    v_uv = uv;
    gl_Position = pos;
}
";

pub static FS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;
uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    gl_FragColor = texture2D(s_texture, v_uv) * color;
}
";

pub static FS_TEXTURED_150_CORE: &'static str = "
#version 150 core
uniform vec4 color;
uniform sampler2D s_texture;

out vec4 out_color;

in vec2 v_uv;

void main()
{
    out_color = texture(s_texture, v_uv) * color;
}
";
