-- wengwengweng

local http = require("http")
local server = http.server("127.0.0.1", 7878)

server:get("/", function()
	return "123"
end)

server:serve()

