//! Shortcut for terse component definitions.
use crate::NeqAssign;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender};
use yew::virtual_dom::VNode;


/// Allows immutable components to be declared using a single struct and two methods.
///
/// This trait defers to its own message handling function versus using the `Emissive` trait like `PureComponent` does.
///
/// If you pass more than one `Callback` as props, then the `Emissive` derive macro will only handle the first.
/// While you could just implement Emissive manually, this trait facilitates keeping all your pure component
/// functions under a single trait definition.
///
/// This trait is blanket implemented for any `T: PureComponent + Emissive`, and is used with the `Pure`
/// wrapper component.
pub trait PureEmissiveComponent: Properties + PartialEq + Sized + 'static {
    /// The message to handled.
    type Message;
    /// Renders self to `Html`.
    fn render(&self) -> Html<Pure<Self>>;
    /// Sends a message.
    fn send_message(&self, _msg: Self::Message) {}
}

impl <T> PureEmissiveComponent for T where T: PureComponent + Emissive {
    type Message = <T as Emissive>::Message;

    fn render(&self) -> VNode<Pure<Self>> {
        <Self as PureComponent>::render(self)
    }

    fn send_message(&self, msg: Self::Message) {
        <Self as Emissive>::emit(self, msg)
    }
}

/// Allows immutable components to be declared using a single struct and a single method.
pub trait PureComponent: Properties + Emissive + PartialEq + Sized + 'static {
    /// Renders self to `Html`.
    fn render(&self) -> Html<Pure<Self>>;
}

/// Derivable trait used to simplify calling callbacks in pure components.
///
/// This trait is responsible for automating calling emit on a callback passed via props within types
/// that implement `PureComponent`.
///
/// # Note
/// When deriving, the derive macro will attempt to locate a field with a `Callback<_>`.
/// type and use the inner type of the callback to specify the `Message` type of `Emissive`.
/// The derived `emit` function will call `self.<name of the callback struct>.emit(msg)`.
///
/// If it cannot find a callback struct, the `Message` type will be set to `()` and `emit` will do nothing.
pub trait Emissive {
    type Message;
    fn emit(&self, msg: Self::Message);
}

/// Wrapper component for pure components.
///
/// Due to constraints in Rust's coherence rules, `Component` can't be implemented for any `T` that implements
/// `PureComponent`, so instead this struct wraps a `T: PureComponent` and `Component` is implemented
/// for this instead.
///
/// # Example
/// It is reasonable practice to use `Pure` as a prefix or `Impl` as a suffix to your pure component model
/// and use an alias to provide a terser name to be used by other components:
///
/// ```
/// use yew::Properties;
/// use yew::Html;
/// use yewtil::{PureComponent, Pure, Emissive};
///
/// #[derive(Properties, PartialEq, Emissive)]
/// pub struct PureMyComponent {
///     pub data: String
/// }
///
/// impl PureComponent for PureMyComponent {
/// fn render(&self) -> Html<Pure<Self>> {
///#        unimplemented!()
///        // ...
///     }
/// }
///
/// /// Use this from within `html!` macros.
/// pub type MyComponent = Pure<PureMyComponent>;
/// ```
#[derive(Debug)]
pub struct Pure<T>(T);

impl<T: PureEmissiveComponent + 'static> Component for Pure<T> {
    type Message = <T as PureEmissiveComponent>::Message;
    type Properties = T;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Pure(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.0.send_message(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn view(&self) -> Html<Self> {
        self.0.render()
    }
}
