// ImDraw trait

// [ ] change from &str to generic string type

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, BTreeMap, BinaryHeap};
use core::fmt::Display;

use imgui::*;
pub use imdraw_derive::ImDraw;

pub trait ImDraw {
    // @Maybe use AsRef<Str>
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui);
}

// ------
// Macros
// ------

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

#[macro_export]
macro_rules! impl_imdraw_blank {
    ($type:path) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, _label: &str, _ui: &imgui::Ui) {
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
                Drag::new(label).build(ui, &mut data);
                *self = data as _;
            }
        }
    };

    ($type:ident with $($extra:ident ( $extra_val:expr )),*) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                Drag::new(label)
                    $( .$extra($extra_val) )*
                    .build(ui, self);
            }
        }
    };

    ($type:ident) => {
        impl ImDraw for $type {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                Drag::new(label).build(ui, self);
            }
        }
    };
}

// ------

impl_drag!(u8);
impl_drag!(u16);
impl_drag!(u32);
impl_drag!(u64);
//impl_drag!(u128);

impl_drag!(i8);
impl_drag!(i16);
impl_drag!(i32);
impl_drag!(i64);
//impl_drag!(i128);

impl_drag!(f32 with speed(0.1));
impl_drag!(f64 with speed(0.1));

impl_drag!(usize using u64);
impl_drag!(isize using i64);

impl ImDraw for bool {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        ui.checkbox(label, self);
    }
}

impl ImDraw for &str {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        ui.text(format!("{}: {}", label, self));
    }
}

impl ImDraw for String {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::InputText::new(ui, label, self).build();
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
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);
            self.0.imdraw("(0)", ui);
            self.1.imdraw("(1)", ui);
            id.pop();
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
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);
            self.0.imdraw("(0)", ui);
            self.1.imdraw("(1)", ui);
            self.2.imdraw("(2)", ui);
            id.pop();
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
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);
            self.0.imdraw("(0)", ui);
            self.1.imdraw("(1)", ui);
            self.2.imdraw("(2)", ui);
            self.3.imdraw("(3)", ui);
            id.pop();
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
                imgui::TreeNode::new(label).build(ui, || {
                    let mut id = ui.push_id(label);

                    let index = 0;
                    $head.imdraw(format!("({})", index), ui);

                    $(
                        let index = index + 1;
                        $tail.imdraw(format!("({})", index), ui);
                    )*

                    id.pop();
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
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);
            for (i, value) in self.iter_mut().enumerate() {
                value.imdraw(&format!("[{}]", i).to_owned(), ui);
            }
            id.pop();
        });
    }
}

impl<K, V> ImDraw for HashMap<K, V>
where
    K: Display,
    V: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);

            for (key, value) in self.iter_mut() {
                value.imdraw(&format!("{}", key).to_owned(), ui);
            }

            id.pop();
        });
    }
}

impl<V> ImDraw for HashSet<V>
where
    V: ImDraw + Copy,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);

            /*
            // @TODO should we allow modifying this?
            while let Some(&value) = self.iter().next() {
                let old_value = value;
                value.imdraw("", ui);
                if value != old_value { ... }
            }
            */

            //for value in self.iter() {
            while let Some(&(mut value)) = self.iter().next() {
                value.imdraw("", ui);
            }

            id.pop();
        });
    }
}

/*
// @Fix Display can't be implemented to tuples, so this breaks quite easily
impl<K, V> ImDraw for BTreeMap<K, V>
where
    K: Display,
    V: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);

            for (key, value) in self.iter_mut() {
                value.imdraw(&format!("{}", key).to_owned(), ui);
            }

            id.pop();
        });
    }
}
*/

impl<K, V> ImDraw for BTreeMap<K, V>
where
    K: Display,
    V: ImDraw,
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);

            for (key, value) in self.iter_mut() {
                value.imdraw(&format!("{key}").to_owned(), ui);
            }

            id.pop();
        });
    }
}


// @TODO imdraw BinaryHeap
impl<V> ImDraw for BinaryHeap<V>
{
    fn imdraw(&mut self, _label: &str, _ui: &imgui::Ui) {}
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
impl<T, const LENGTH: usize> ImDraw for [T; LENGTH]
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            for i in 0..LENGTH {
                self[i].imdraw(&format!("[{}]", i), ui);
            }
        });
    }
}

// slices
/*
impl<T> ImDraw for &[T]
where
    T: ImDraw
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            for i in 0..self.len() {
                self[i].imdraw(&format!("[{}]", i), ui);
            }

            /*
            self.iter()
                .enumerate()
                .for_each(|(i, elem)| elem.imdraw(&format!("[{}]", i), ui));
            */
        });
    }
}
*/

impl<T> ImDraw for &[T]
{
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        ui.text(format!("{}: (todo)", label));
    }
}
