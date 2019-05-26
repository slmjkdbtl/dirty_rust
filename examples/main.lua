-- wengwengweng

local window = require("window")
local audio = require("audio")
local term = require("term")

local win = window.make({})
local sound = audio.async_load_file("examples/res/yo.ogg")

win:run(function(ctx)

	if not sound:done() then

		sound:poll()

		if sound:done() then
			sound:data():play()
		end

	end

end)

