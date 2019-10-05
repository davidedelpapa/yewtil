use std::rc::Rc;
use yew::{Callback, Children, Component, ComponentLink, Html, Properties, Renderable};

pub struct PersistFetching<T: 'static, M: 'static> {
    props: PersistFetchingProps<T, M>,
}

pub struct Render<T: 'static, M: 'static> {
    render: Option<Box<dyn Fn(&T) -> Html<PersistFetching<T, M>>>>,
}

impl<T, M: 'static> Render<T, M> {
    fn new<F: Fn(&T) -> Html<PersistFetching<T, M>> + 'static>(f: F) -> Self {
        Render {
            render: Some(Box::new(f)),
        }
    }
}

impl<T, M> Default for Render<T, M> {
    fn default() -> Self {
        Render { render: None }
    }
}

impl<T: 'static, M: 'static> PersistFetching<T, M> {
    pub fn render<F: Fn(&T) -> Html<PersistFetching<T, M>> + 'static>(f: F) -> Render<T, M> {
        Render::new(f)
    }
}

#[derive(Properties)]
pub struct PersistFetchingProps<T: 'static, M: 'static> {
    children: Children<PersistFetching<T, M>>,
    pub render: Render<T, M>,
    /// The user should not set this, but it will always be set by the Fetch component.
    pub(crate) data: Option<Rc<T>>, // TODO maybe use a wrapper around MaybeUninit<RC<T>> with Default implemented for it here. That way, we can eliminate the discriminant.

    pub(crate) callback: Option<Callback<M>>,
}

impl<T: 'static, M: 'static> Component for PersistFetching<T, M> {
    type Message = M;
    type Properties = PersistFetchingProps<T, M>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PersistFetching { props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        if let Some(callback) = &self.props.callback {
            callback.emit(msg)
        }
        false
    }
}

impl<T: 'static, M: 'static> Renderable<PersistFetching<T, M>> for PersistFetching<T, M> {
    fn view(&self) -> Html<Self> {
        if let Some(render) = &self.props.render.render {
            (render)(&self.props.data.as_ref().unwrap())
        } else {
            self.props.children.iter().collect()
        }
    }
}