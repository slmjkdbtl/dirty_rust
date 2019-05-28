-- wengwengweng

local audio = require("audio")
local yo_load = audio.async_load_file("examples/res/yo.ogg")

while true do

	local yo = yo_load:poll()

	if yo then
		yo:play()
	end

end

