// Total time: 0.315 seconds (Macbook Air M1, 2020)
// width = 400;
// height = 200;
// samples = 20;
let width = 400.0;
let height = 200.0;
let samples = 20;

// Setup camera
let look_from = vec3(0.0, 0.0, 0.0);
let look_at = vec3(0.0, 0.0, -1.0);
let v_up = vec3(0.0, 1.0, 0.0);
let v_fov = 90.0;
let cam = camera(look_from, look_at, v_up, v_fov, width / height);

// Materials
let white_mat = lambertian(vec3(0.5, 0.5, 0.5));
let red_mat =  lambertian(vec3(0.5, 0.2, 0.2));
let glass_mat =  dielectric(1.5);
let rough_metal = metal(vec3(0.8, 0.6, 0.2), 0.5);

// Scene
let scene = [
  sphere(vec3(0.0, -100.5, -1.0), 100.0, white_mat),
  sphere(vec3(-1.0, 0.0, -1.0), 0.5, glass_mat),
  sphere(vec3(0.0, 0.0, -1.0), 0.5, red_mat),
  sphere(vec3(1.0, 0.0, -1.0), 0.5, rough_metal),
];

// Render
let sky_brightness = 1.0;
render(width.to_int(), height.to_int(), samples, cam, scene, sky_brightness, "quick_demo");
