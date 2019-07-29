-- wengwengweng

local app = require("app")
local window = app.make()

window:run(function(ctx)
	print(ctx:time())
end)

