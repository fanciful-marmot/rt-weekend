-- Total time: 280.452 seconds (4.674213 minutes)
-- width = 1200;
-- height = 600;
-- samples = 200;

width = 1200;
height = 600;
samples = 200;

look_from = vec3(13, 2, 3);
look_at = vec3(0, 0, 0);
v_up = vec3(0, 1, 0);
v_fov = 20;
cam = camera(look_from, look_at, v_up, v_fov, width / height);

hitlist = {}

ground_mat = {
    type = "lambertian",
    albedo = vec3(0.5, 0.5, 0.5)
}

table.insert(hitlist, {
    type = "sphere",
    position = vec3(0, -1000, 0),
    radius = 1000,
    material = ground_mat
})

math.randomseed(os.time())

function rfloat(a, b)
    return a + math.random() * (b - a)
end

for a= -11, 10 do
    for b = -11, 10 do
        choose_mat = math.random()
        x = a + 0.9 * math.random()
        y= 0.2
        z = b + 0.9 * math.random()

        x_sqr = (x - 4) * (x - 4)
        y_sqr = 0
        z_sqr = z * z

        if (x_sqr + z_sqr) > 0.9 * 0.9 then
            if choose_mat < 0.8 then
                -- diffuse
                table.insert(hitlist, {
                    type = "sphere",
                    position = vec3(x, y, z),
                    radius = 0.2,
                    material = {
                        type = "lambertian",
                        albedo = vec3(math.random() * math.random(), math.random() * math.random(), math.random() * math.random())
                    }
                })
            elseif choose_mat < 0.95 then
                -- metal
                table.insert(hitlist, {
                    type = "sphere",
                    position = vec3(x, y, z),
                    radius = 0.2,
                    material = {
                        type = "metal",
                        albedo = vec3(rfloat(0.5, 1), rfloat(0.5, 1), rfloat(0.5, 1)),
                        roughness = rfloat(0, 0.5)
                    }
                })
            else
                -- glass
                table.insert(hitlist, {
                    type = "sphere",
                    position = vec3(x, y, z),
                    radius = 0.2,
                    material = {
                        type = "dielectric",
                        refractive_index = 1.5
                    }
                })
            end
        end
    end
end

table.insert(hitlist, {
    type = "sphere",
    position = vec3(0, 1, 0),
    radius = 1,
    material = {
        type = "dielectric",
        refractive_index = 1.5
    }
})

table.insert(hitlist, {
    type = "sphere",
    position = vec3(-4, 1, 0),
    radius = 1,
    material = {
        type = "lambertian",
        albedo = vec3(0.7, 0.2, 0.1)
    }
})

table.insert(hitlist, {
    type = "sphere",
    position = vec3(4, 1, 0),
    radius = 1,
    material = {
        type = "metal",
        albedo = vec3(0.7, 0.6, 0.5),
        roughness = 0
    }
})

render(width, height, samples, cam, hitlist, "image_ch_1.png")
