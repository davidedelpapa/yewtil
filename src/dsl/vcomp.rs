use yew::Component;
use yew::virtual_dom::vcomp::ScopeHolder;
use yew::virtual_dom::VComp;
use crate::dsl::BoxedVNodeProducer;

pub struct VCompProducer<COMP: Component>(Box<dyn FnOnce(ScopeHolder<COMP>) -> VComp<COMP>>);

impl <COMP: Component> VCompProducer<COMP> {
    pub fn new<CHILD: Component>(props: CHILD::Properties) -> Self {
        VCompProducer(Box::new(move |scope| {
            VComp::new::<CHILD>(props, scope)
        }))
    }
}

impl <COMP: Component> From<VCompProducer<COMP>> for BoxedVNodeProducer<COMP> {
    fn from(vcomp_prod: VCompProducer<COMP>) -> Self {
        BoxedVNodeProducer::wrap(move |scope| {
            (vcomp_prod.0)(scope).into()
        })
    }
}