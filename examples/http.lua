-- wengwengweng

local fs = require("fs")
local http = require("http")
local res = http.res

http.serve("localhost", 7878, function(req)

	if req:path() == "/icon.png" then
		return res.png(fs.read("icon.png"))
	end

	if req:path() == "/" then
		return res.html(fs.read_str("examples/res/index.html"));
	else
		return res.text("no")
	end

end)

