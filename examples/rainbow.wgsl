@group(0) @binding(0) var<uniform> update: u32;

struct Fragment {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@vertex
fn vs(@location(0) position: vec3f) -> Fragment {
    var frag: Fragment;
    frag.position = vec4f(position, 1);
    return frag;
}

@fragment
fn fs(frag: Fragment) -> @location(0) vec4f {
    let frequency = 3.14159 / 40.0;
    let r = sin((frequency * f32(update)) + 0.0) * 0.5 + 0.5;
    let g = sin((frequency * f32(update)) + 2.0) * 0.5 + 0.5;
    let b = sin((frequency * f32(update)) + 4.0) * 0.5 + 0.5;

    return vec4f(r, g, b, 1);
}
