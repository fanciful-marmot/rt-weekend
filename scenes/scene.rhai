// Total time: 103.49 seconds (Macbook Air M1, 2020)
// width = 1200;
// height = 600;
// samples = 200;
let width = 1920.0;
let height = 1080.0;
let samples = 600;

// Setup camera
let look_from = vec3(15.0, 2.0, 3.0);
let look_at = vec3(0.0, 0.0, 0.0);
let v_up = vec3(0.0, 1.0, 0.0);
let v_fov = 20.0;
let cam = camera(look_from, look_at, v_up, v_fov, width / height);

// Materials
let ground_mat = lambertian(vec3(0.5, 0.5, 0.5));
let red_mat = lambertian(vec3(0.5, 0.2, 0.2));
let shiny_metal = metal(vec3(0.7, 0.6, 0.5), 0.05);
let rough_metal = metal(vec3(0.8, 0.6, 0.2), 0.5);

// Scene
let scene = [
    sphere(vec3(0.0, -1000.0, -1.0), 1000.0, ground_mat),
];

fn rand2() {
    rand_float() * rand_float()
}

for a in range(-11, 10) {
    for b in range(-11, 10) {
        let choose_mat = rand_float();
        let x = a + 0.9 * rand_float();
        let y = 0.2;
        let z = b + 0.9 * rand_float();

        let x_sqr = (x - 4.0) * (x - 4.0);
        let y_sqr = 0.0;
        let z_sqr = z * z;
        let r = 0.2;

        if (x_sqr + z_sqr) > 0.9 * 0.9 {
            let mat = if choose_mat < 0.65 {
                // diffuse
                lambertian(vec3(rand2(), rand2(), rand2()))
            } else if choose_mat < 0.85 {
                // metal
                metal(
                    vec3(rand_float(0.5, 1.0), rand_float(0.5, 1.0), rand_float(0.5, 1.0)),
                    rand_float(0.0, 0.5),
                )
            } else if choose_mat < 0.95 {
                r = 0.25;
                // emissive
                emissive(vec3(
                    rand_float(2.2, 8.0),
                    rand_float(2.2, 8.0),
                    rand_float(2.2, 8.0),
                ))
            } else {
                // glass
                dielectric(1.5)
            };

            scene.push(
                sphere(vec3(x, r, z), r, mat)
            );
        }
    }
}

scene.push(
    sphere(vec3(0.0, 1.0, 0.0), 1.0, dielectric(1.5))
);
scene.push(
    sphere(vec3(-4.0, 1.0, 0.0), 1.0, lambertian(vec3(0.7, 0.2, 0.1)))
);
scene.push(
    sphere(vec3(4.0, 1.0, 0.0), 1.0, metal(vec3(0.7, 0.6, 0.5), 0.0))
);

// Render
render(width.to_int(), height.to_int(), samples, cam, scene, "image_ch_1.png");
