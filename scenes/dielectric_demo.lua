-- Total time: 66.816 seconds
-- width = 1200;
-- height = 600;
-- samples = 200;

width = 1200;
height = 600;
samples = 200;

look_from = vec3(0, 0, 0);
look_at = vec3(0, 0, -1);
v_up = vec3(0, 1, 0);
v_fov = 90;
cam = camera(look_from, look_at, v_up, v_fov, width / height);

white_mat = {
    type = "lambertian",
    albedo = vec3(0.5, 0.5, 0.5)
}
blue_mat = {
    type = "lambertian",
    albedo = vec3(0.1, 0.2, 0.5)
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
        material = blue_mat
    },
    {
        type = "sphere",
        position = vec3(1, 0, -1),
        radius = 0.5,
        material = white_mat
    },
}
render(width, height, samples, cam, hitlist, "dielectric_demo.png")
