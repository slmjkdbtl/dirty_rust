-- wengwengweng

local app = require("app")
local window = app.make()

window:init(function(ctx)
end)

window:run(function(ctx)

	ctx:title(tostring(ctx:mouse_pos()))
	ctx:text("yo")

	if ctx:key_pressed("f") then
		ctx:fullscreen(not ctx:fullscreen())
	end

	if ctx:key_pressed("esc") then
		ctx:quit()
	end

end)

