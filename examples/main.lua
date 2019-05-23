-- wengwengweng

local win = window.create()
local sound = fs.aread_bytes("can.ogg")
local s = nil
local i = 1

win:run(function(ctx)

	if not sound:done() then

		i = i + 1
		print(i)
		sound:poll()

		if sound:done() then
			print("yoyoyo")
-- 			s = audio.sound(sound:data())
-- 			s:play()
		end

	end

end)

