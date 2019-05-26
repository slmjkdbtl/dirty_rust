-- wengwengweng

local window = require("window")
local audio = require("audio")
local ansi = require("ansi")

local win = window.create({})
local sound = audio.async_load_file("examples/res/yo.ogg")

win:run(function(ctx)

	if not sound:done() then

		sound:poll()

		if sound:done() then
			sound:data():play()
		end

	end

end)

