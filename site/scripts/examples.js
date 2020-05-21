// wengwengweng

const list = [
	"raw",
	"sprite",
	"3d",
	"input",
	"audio",
];

export default {

	view() {

		return [
			m("img#logo", {
				src: "/img/icon.png",
			}),
			m("p#intro", [m("b", "Examples")]),
			list.map((name) => {
				return [
					m("a", {
						href: `/example/${name}`
					}, name),
					m("br"),
				];
			}),
		];

	},

};

