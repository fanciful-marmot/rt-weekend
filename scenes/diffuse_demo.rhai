// Total time: 13.05 seconds (Macbook Air M1, 2020)
// width = 1200;
// height = 600;
// samples = 200;
let width = 1200.0;
let height = 600.0;
let samples = 200;

// Setup camera
let look_from = vec3(0.0, 0.0, 0.0);
let look_at = vec3(0.0, 0.0, -1.0);
let v_up = vec3(0.0, 1.0, 0.0);
let v_fov = 90.0;
let cam = camera(look_from, look_at, v_up, v_fov, width / height);

// Materials
let white_mat = lambertian(vec3(0.5, 0.5, 0.5));

// Scene
let scene = [
  sphere(vec3(0.0, 0.0, -1.0), 0.5, white_mat),
  sphere(vec3(0.0, -100.5, -1.0), 100.0, white_mat),
];

// Render
let sky_brightness = 1.0;
render(width.to_int(), height.to_int(), samples, cam, scene, sky_brightness, "diffuse_demo");
