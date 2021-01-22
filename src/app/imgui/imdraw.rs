//use std::ffi::CString;
use crate::imgui::*;
pub use imdraw_derive::ImDraw;

pub trait ImDraw {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui);
}

#[macro_export]
macro_rules! im_str2 {
    ($e:tt) => ({
        &$crate::imgui::ImString::new($e)
    });
}

// @Refactor use optionals to be able to join with and using
macro_rules! impl_imdraw {
    ($type:ident using $cast:ident) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                let mut data = *self as $cast;
                Drag::new(im_str2!(label)).build(ui, &mut data);
                *self = data as _;
            }
        }
    };

    ($type:ident with $($extra:ident ( $extra_val:expr )),*) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                Drag::new(im_str2!(label))
                    $( .$extra($extra_val) )*
                    .build(ui, self);
            }
        }
    };

    ($type:ident) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                Drag::new(im_str2!(label)).build(ui, self);
            }
        }
    };
}

impl_imdraw!(u8);
impl_imdraw!(u16);
impl_imdraw!(u32);
impl_imdraw!(u64);

impl_imdraw!(i8);
impl_imdraw!(i16);
impl_imdraw!(i32);
impl_imdraw!(i64);

impl_imdraw!(f32 with speed(0.1));
impl_imdraw!(f64 with speed(0.1));

impl_imdraw!(usize using u64);
impl_imdraw!(isize using i64);

// Tuples
// we shouldn't need more than length 4

impl<A, B> ImDraw for (A, B)
where
    A: ImDraw,
    B: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);
            self.0.imdraw("(0)", ui);
            self.1.imdraw("(1)", ui);
            id.pop(ui);
        });
    }
}

impl<A, B, C> ImDraw for (A, B, C)
where
    A: ImDraw,
    B: ImDraw,
    C: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);
            self.0.imdraw("(0)", ui);
            self.1.imdraw("(1)", ui);
            self.2.imdraw("(2)", ui);
            id.pop(ui);
        });
    }
}

impl<A, B, C, D> ImDraw for (A, B, C, D)
where
    A: ImDraw,
    B: ImDraw,
    C: ImDraw,
    D: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);
            self.0.imdraw("(0)", ui);
            self.1.imdraw("(1)", ui);
            self.2.imdraw("(2)", ui);
            self.3.imdraw("(3)", ui);
            id.pop(ui);
        });
    }
}

// Would be cool to be able to use a declarative macro for this, but we can't simply iterate over a
// tuple in declarative macros, and procedural macros are more annoying to make.
/*
macro_rules! tuple_impl_imdraw {
    ($head:ident, $( $tail:ident ),* ) => {
        impl<$head, $($tail),*> ImDraw for ($head, $($tail),*)
        where
            $head: ImDraw,
            $( $tail: ImDraw ),*
        {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                imgui::TreeNode::new(im_str2!(label)).build(ui, || {
                    let mut id = ui.push_id(label);

                    let index = 0;
                    $head.imdraw(format!("({})", index), ui);

                    $(
                        let index = index + 1;
                        $tail.imdraw(format!("({})", index), ui);
                    )*

                    id.pop(ui);
                });
            }
        }

        //tuple_impl_imdraw!($( $tail ),*);
    };

    () => {};
}

tuple_impl_imdraw!(A, B, C, D);
*/

