// Total time: 29.82 seconds (Macbook Air M1, 2020)
// width = 1200;
// height = 600;
// samples = 200;
let width = 600.0;
let height = 300.0;
let samples = 200;

// Setup camera
let look_from = vec3(0.0, 0.5, 0.5);
let look_at = vec3(0.0, 0.0, -1.0);
let v_up = vec3(0.0, 1.0, 0.0);
let v_fov = 90.0;
let cam = camera(look_from, look_at, v_up, v_fov, width / height);

// Materials
let white_mat = lambertian(vec3(0.5, 0.5, 0.5));
let emissive_mat = emissive(vec3(6.0, 12.0, 12.0));
let shiny_metal = metal(vec3(0.7, 0.6, 0.5), 0.05);
let rough_metal = metal(vec3(0.8, 0.6, 0.2), 0.5);
let glass_mat =  dielectric(1.5);

// Scene
let scene = [
    sphere(vec3(0.0, -100.5, -1.0), 100.0, white_mat),
    sphere(vec3(-1.0, 0.0, -1.0), 0.5, shiny_metal),
    sphere(vec3(0.0, 2.0, -1.0), 0.25, emissive_mat),
    sphere(vec3(0.0, 0.25, -1.0), 0.25, glass_mat),
    sphere(vec3(1.0, 0.0, -1.0), 0.5, rough_metal),
];

// Render
let sky_brightness = 0.1;
render(width.to_int(), height.to_int(), samples, cam, scene, sky_brightness, "caustics_demo");
