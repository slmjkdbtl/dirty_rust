-- wengwengweng

local http = require("http")
local res = http.get("https://db.ygoprodeck.com/api/v4/cardinfo.php")

print(res:text())

