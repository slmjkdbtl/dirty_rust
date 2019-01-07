-- wengwengweng

app_init()
window_init("yo", 640, 480)
gfx_init()
res_init()

res_load_sprites("examples/", {"car"})

app_run(function()
	gfx_clear()
end)

