-- Total time: 24.715 seconds (Macbook Air M1, 2020)
-- width = 1200;
-- height = 600;
-- samples = 200;

width = 1200;
height = 600;
samples = 200;

--- Built-ins ---
-- TODO: Should be automatically provided by engine
function vec3(x, y, z)
    return {
        x = x,
        y = y,
        z = z,
    }
end

function camera(look_from, look_at, v_up, v_fov, aspect)
    return {
        look_from = look_from,
        look_at = look_at,
        v_up = v_up,
        v_fov = v_fov,
        aspect = aspect,
    }
end
-- Built-ins ---

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
