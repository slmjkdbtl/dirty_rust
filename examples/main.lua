-- wengwengweng

local win = window.create()
local s = audio.sound(fs.read_bytes("examples/res/yo.ogg"))

s:play()

win:run(function(ctx)
-- 	print(ctx:time())
end)

