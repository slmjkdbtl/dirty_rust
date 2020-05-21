// wengwengweng

export default {

	view() {

		return [
			m("img#logo", {
				src: "/img/icon.png",
			}),
			m("p#intro", [m("b", "DIRTY"), " is a cross-platform toolkit for making games / interactive experiences."]),
			m("a", {
				href: "https://git.sr.ht/~slmjkdbtl/DIRTY",
			}, "code"),
			m("br"),
			m("a", {
				href: "/examples",
			}, "examples"),
			m("br"),
			m("a", {
				href: "/doc",
			}, "doc"),
		];

	},

};

