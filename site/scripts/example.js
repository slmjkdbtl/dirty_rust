// wengwengweng

// TODO: kinda messy

let state = 1;

export default {

	oninit(vnode) {
		import(`/examples/${vnode.attrs.name}.js`).then((mod) => {
			state = 2;
			mod.default();
			m.redraw();
		}).catch((e) => {
			state = 0;
			m.redraw();
		});
	},

	view(vnode) {

		if (state == 2) {
			return [
				m("a", {
					href: `https://git.sr.ht/~slmjkdbtl/DIRTY/tree/master/examples/${vnode.attrs.name}.rs`,
				}, "source"),
				m("br"),
				m("br"),
			];
		} else if (state == 0) {
			return m("p", "no such example");
		} else {
			return m("p", "loading...");
		}

	},

};

