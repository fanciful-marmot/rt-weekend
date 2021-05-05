-- Total time: 0.6525 seconds
-- width = 400;
-- height = 200;
-- samples = 20;

width = 400;
height = 200;
samples = 20;

look_from = vec3(0, 0, 0);
look_at = vec3(0, 0, -1);
v_up = vec3(0, 1, 0);
v_fov = 90;
cam = camera(look_from, look_at, v_up, v_fov, width / height);

white_mat = {
    type = "lambertian",
    albedo = vec3(0.5, 0.5, 0.5)
}
red_mat = {
    type = "lambertian",
    albedo = vec3(0.5, 0.2, 0.2)
}
glass_mat = {
    type = "dielectric",
    refractive_index = 1.5
}
rough_metal = {
    type = "metal",
    albedo = vec3(0.8, 0.6, 0.2),
    roughness = 0.5,
}
hitlist = {
    {
        type = "sphere",
        position = vec3(0, -100.5, -1),
        radius = 100,
        material = white_mat
    },
    {
        type = "sphere",
        position = vec3(-1, 0, -1),
        radius = 0.5,
        material = glass_mat
    },
    {
        type = "sphere",
        position = vec3(0, 0, -1),
        radius = 0.5,
        material = red_mat
    },
    {
        type = "sphere",
        position = vec3(1, 0, -1),
        radius = 0.5,
        material = rough_metal
    },
}
render(width, height, samples, cam, hitlist, "quick_demo.png")
