-- wengwengweng

local fs = require("fs")
local http = require("http")

http.serve("localhost", 7878, function(req)
	if req:path() == "/" then
		return http.response(fs.read_str("examples/res/index.html"));
	else
		return http.response("no")
	end
end)


