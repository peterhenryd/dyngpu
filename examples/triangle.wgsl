@vertex
fn vs(@location(0) position: vec3f) -> @builtin(position) vec4f {
  return vec4f(position, 1);
}

@fragment
fn fs() -> @location(0) vec4f {
  return vec4f(1, 0, 1, 1);
}
