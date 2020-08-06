// wengwengweng

use super::*;

macro_rules! make_handle {

	($t:ident, $lt:ident) => {

		paste::paste! {

			pub(super) type [<$t ID>] = <glow::Context as HasContext>::$t;

			pub(super) struct [<$t Handle>] {
				ctx: Rc<glow::Context>,
				id: [<$t ID>],
			}

			impl [<$t Handle>] {
				pub fn new(ctx: Rc<glow::Context>) -> Result<Self> {
					unsafe {
						return Ok(Self {
							id: ctx.[<create_ $lt>]()?,
							ctx: ctx.clone(),
						});
					}
				}
				pub fn id(&self) -> [<$t ID>] {
					return self.id;
				}
				pub fn ctx(&self) -> &glow::Context {
					return &self.ctx;
				}
			}

			impl Drop for [<$t Handle>] {
				fn drop(&mut self) {
					unsafe {
						self.ctx.[<delete_ $lt>](self.id);
					}
				}
			}

			impl PartialEq for [<$t Handle>] {
				fn eq(&self, other: &Self) -> bool {
					return self.id == other.id;
				}
			}

		}

	}

}

make_handle!(Buffer, buffer);
make_handle!(Texture, texture);
make_handle!(Program, program);
make_handle!(Framebuffer, framebuffer);
make_handle!(Renderbuffer, renderbuffer);

