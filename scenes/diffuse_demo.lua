-- Total time: 26.445 seconds
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
hitlist = {
    {
        type = "sphere",
        position = vec3(0, 0, -1),
        radius = 0.5,
        material = white_mat
    },
    {
        type = "sphere",
        position = vec3(0, -100.5, -1),
        radius = 100,
        material = white_mat
    }
}
render(width, height, samples, cam, hitlist, "diffuse_demo.png")
