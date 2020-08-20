// wengwengweng

let state = "loading";

export default {

	async oninit(vnode) {

		try {
			const mod = await import(`/examples/${vnode.attrs.name}.js`);
			state = "ready";
			mod.default();
			document.title = vnode.attrs.name;
			m.redraw();
		} catch (e) {
			state = "error";
			document.title = "error";
			console.error(e);
			m.redraw();
		}

	},

	view(vnode) {

		switch (state) {
			case "ready":
				return [
					m("a", {
						href: `https://git.sr.ht/~slmjkdbtl/DIRTY/tree/master/examples/${vnode.attrs.name}.rs`,
					}, "source"),
					m("br"),
					m("br"),
				];
				break;
			case "error":
			return m("p", `error loading example ${vnode.attrs.name}`);
				break;
			default:
				return m("p", "loading...");
		}

	},

};

