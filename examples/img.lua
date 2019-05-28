-- wengwengweng

local img = require("img")
local fs = require("fs")

local icon = img.load(fs.read("icon.png"))

print(icon:width())
print(icon:height())

