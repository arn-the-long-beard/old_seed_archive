use seed::prelude::Node;

pub trait ToView<Routes, State, Msg> {
    fn view(&self, scoped_state: &State) -> Node<Msg>;
}

pub trait Guarded<Routes, State, Msg> {
    fn check_before_load(&self, scoped_state: &State) -> Option<bool>;
}
