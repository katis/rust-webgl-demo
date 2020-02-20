attribute vec2 coordinates;
attribute vec2 texcoord;

varying highp vec2 uv;

void main(void) {
    gl_Position = vec4(coordinates, 0.0, 1.0);
    uv = texcoord;
}
