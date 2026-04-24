pub const VERT_SRC: &str = r#"
#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aT;
out vec2 aTex;
layout (location = 5)uniform mat4 pv;
layout (location = 6)uniform mat4 model;
void main() {
    gl_Position = pv*model*vec4(aPos, 1.0);
    aTex=aT;
}
"#;

pub const FRAG_SRC: &str = r#"
#version 460 core
out vec4 FragColor;
in vec2 aTex;
uniform sampler2D tex;
void main() {
    FragColor = texture(tex, aTex);
}
"#;
//*vec4(0.3,0.5,0.3,1.0);
