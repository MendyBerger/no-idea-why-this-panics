use futures_signals::signal_vec::MutableVec;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use dominator::{Dom, html, clone, events};


#[derive(Debug)]
pub struct Component {
    pub reverse: Mutable<bool>,
    pub list: MutableVec<String>,
}

impl Component {
    pub fn new() -> Rc<Self> {
        Rc::new(Component {
            reverse: Mutable::new(true),
            list: MutableVec::new_with_values(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ]),
        })
    }

    pub fn render(state: Rc<Self>) -> Dom {
        html!("ul", {
            .child(html!("button", {
                .text("Reverse")
                .event(clone!(state => move |_event: events::Click| {
                    let current: bool = *state.reverse.lock_ref();
                    *state.reverse.lock_mut() = !current;
                }))
            }))
            .children_signal_vec(state.list.signal_vec_cloned()
                .map_signal(clone!(state => move |item| {
                    state.reverse.signal_cloned().map(clone!(item => move |reverse| {
                        (item.clone(), reverse)
                    }))
                }))
                .sort_by_cloned(|(a, reverse), (b, _)| {
                    let mut ord = a.cmp(&b);

                    if *reverse {
                        ord = ord.reverse();
                    };

                    ord
                })
                .map(|(item, _)| item)
                .map(clone!(state => move |item| {
                    html!("li", {
                        .text(&item)
                    })
                }))
            )
        })
    }
}