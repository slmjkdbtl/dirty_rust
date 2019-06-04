-- wengwengweng

local http = require("http")

http.serve("localhost", 7878, function(req)

	local path = req:path()

	if path == "/" then
		print("hi")
	else
		print("no")
	end

end)


