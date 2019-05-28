-- wengwengweng

local window = require("window")
local win = window.make({})

win:run(function(ctx)

	if ctx:key_pressed("f") then
		ctx:toggle_fullscreen()
	end

	if ctx:key_pressed("esc") then
		ctx:close()
	end

end)

