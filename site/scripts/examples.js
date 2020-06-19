// wengwengweng

const list = [
	"tri",
	"sprite",
	"shader",
	"model",
	"3d",
	"canvas",
	"input",
	"mask",
	"spline",
	"imgui",
	"audio",
	"data",
];

export default {

	oninit() {
		document.title = "Examples";
	},

	view() {

		return [
			m("img#logo", {
				src: "/img/logo.png",
			}),
			m("p#intro", [m("b", "Examples")]),
			list.map((name) => {
				return [
					m("a", {
						href: `/examples/${name}`
					}, name),
					m("br"),
				];
			}),
		];

	},

};

