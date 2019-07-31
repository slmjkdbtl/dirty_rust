-- wengwengweng

local app = require("app")
local window = app.make()

window:init(function(ctx)
end)

window:run(function(ctx)

	if ctx:key_down("f") then
		ctx:fullscreen()
	end

	print(ctx:mouse_pos())

	if ctx:key_down("esc") then
		ctx:quit()
	end

end)

