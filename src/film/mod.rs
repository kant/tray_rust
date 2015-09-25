//! The film module provides color types and a render target that the image
//! is written too.

pub use self::color::Colorf;
pub use self::render_target::RenderTarget;
pub use self::camera::Camera;
pub use self::render_target::ImageSample;

pub mod color;
pub mod render_target;
pub mod camera;
pub mod filter;

