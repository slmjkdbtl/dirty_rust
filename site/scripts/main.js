// wengwengweng

import home from "./home.js";
import examples from "./examples.js";
import example from "./example.js";
import err from "./err.js";

m.route.prefix = "";

m.route(document.body, "/", {
	"/": home,
	"/examples": examples,
	"/examples/:name": example,
	"/:404...": err,
});

