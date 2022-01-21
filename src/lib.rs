pub mod app;
pub mod cpp;
mod lib_impl;
pub mod lua2cpp;
pub mod phx;

pub use lib_impl::lib;

#[macro_export]
macro_rules! size_of {
    ($ty:tt) => {
        std::mem::size_of::<$ty>()
    }
}

#[cfg(feature = "type_assert")]
#[macro_use]
extern crate memoffset;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct lua_State(u64);

#[cfg(feature = "type_assert")]
pub fn validate() {
    app::LinkEvent::assert();
    app::LinkEventCapture::assert();
    app::LinkEventCaptureItem::assert();
    app::LinkEventCaptureDriver::assert();
    app::LinkEventCaptureMimikkyu::assert();
    app::LinkEventCapturePulled::assert();
    app::LinkEventFinal::assert();
    app::LinkEventMask::assert();
    app::LinkEventPos::assert();
    app::LinkEventStarShot::assert();
    app::LinkEventThrow::assert();
    app::LinkEventTouchItem::assert();
    app::LinkEventYoshiTamagoDamageEffect::assert();
}