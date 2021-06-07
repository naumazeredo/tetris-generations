// ImDraw trait

// [ ] change from &str to generic string type

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, BTreeMap};
use core::fmt::Display;

use imgui::*;
pub use imdraw_derive::ImDraw;

// @Refactor use a template to any string type
pub trait ImDraw {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui);
}

// ------
// Macros
// ------

// @TODO remove this!!!
#[macro_export]
macro_rules! im_str2 {
    ($e:tt) => ({
        &$crate::imgui::ImString::new($e)
    });
}

#[macro_export]
macro_rules! impl_imdraw_todo {
    ($type:path) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                ui.text(format!("{}: (todo)", label));
            }
        }
    };
}

// @Refactor use optionals to be able to join with and using
macro_rules! impl_drag {
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

// ------

impl_drag!(u8);
impl_drag!(u16);
impl_drag!(u32);
impl_drag!(u64);

impl_drag!(i8);
impl_drag!(i16);
impl_drag!(i32);
impl_drag!(i64);

impl_drag!(f32 with speed(0.1));
impl_drag!(f64 with speed(0.1));

impl_drag!(usize using u64);
impl_drag!(isize using i64);

impl ImDraw for bool {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        ui.checkbox(im_str2!(label), self);
    }
}

impl ImDraw for String {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        let mut im_str = self.clone().into();
        imgui::InputText::new(ui, im_str2!(label), &mut im_str).build();
        *self = im_str.to_string();
    }
}

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

// option

impl<T> ImDraw for Option<T>
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        if self.is_none() {
            ui.text(format!("{}: (None)", label));
        } else {
            let inner = self.as_mut().unwrap();
            inner.imdraw(&format!("Some {}", label).to_owned(), ui);
        }
    }
}

// std pointers

/*
impl<T> ImDraw for Rc<T>
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        //ui.text(format!("{}: (testing)", label));
        T::imdraw(self, &format!("Rc {}", label).to_owned(), ui);
    }
}
*/

impl<T> ImDraw for Rc<RefCell<T>>
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        self.borrow_mut().imdraw(&format!("Rc RefCell {}", label).to_owned(), ui);
    }
}

// std containers

impl<T> ImDraw for Vec<T>
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);
            for (i, value) in self.iter_mut().enumerate() {
                value.imdraw(&format!("[{}]", i).to_owned(), ui);
            }
            id.pop(ui);
        });
    }
}

impl<K, V> ImDraw for HashMap<K, V>
where
    K: Display,
    V: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);

            for (key, value) in self.iter_mut() {
                value.imdraw(&format!("{}", key).to_owned(), ui);
            }

            id.pop(ui);
        });
    }
}

impl<K, V> ImDraw for BTreeMap<K, V>
where
    K: Display,
    V: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);

            for (key, value) in self.iter_mut() {
                value.imdraw(&format!("{}", key).to_owned(), ui);
            }

            id.pop(ui);
        });
    }
}

// std cells

impl<T> ImDraw for RefCell<T>
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        self.borrow_mut().imdraw(&format!("RefCell {}", label).to_owned(), ui);
    }
}

// arrays
// @TODO macro this to do multiple sizes
impl ImDraw for [u8; 7] {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            for i in 0..7 {
                self[i].imdraw(&format!("[{}]", i), ui);
            }
        });
    }
}
