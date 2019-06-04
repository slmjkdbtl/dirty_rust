-- wengwengweng

local window = require("window")
local gfx = require("gfx")
local fs = require("fs")
local win = window.make({})

local a = gfx.texture(fs.read("examples/res/car.png"))

win:run(function(ctx)

	if ctx:key_pressed("f") then
		ctx:toggle_fullscreen()
	end

	if ctx:key_pressed("esc") then
		ctx:close()
	end

end)

