// wengwengweng

export default {

	oninit() {
		document.title = "DIRTY";
	},

	view() {
		return [
			m("img#logo", {
				src: "/img/logo.png",
			}),
			m("p#intro", [m("b", "DIRTY"), " is a cross-platform toolkit for making games / interactive experiences."]),
			m("a", {
				href: "https://github.com/slmjkdbtl/DIRTY",
			}, "code"),
			m("br"),
			m("a", {
				href: "/examples",
			}, "examples"),
			m("br"),
			m("a", {
				href: "/doc/dirty/index.html",
			}, "doc"),
		];
	},

};

