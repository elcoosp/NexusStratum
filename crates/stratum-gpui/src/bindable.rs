use gpui::*;

/// A trait for headless primitives that can be bound to a GPUI model field.
pub trait BindablePrimitive<P, S, C>
where
    P: stratum_core::Props,
    S: stratum_core::State + Clone,
    C: stratum_core::Component<Props = P, State = S>,
{
    /// Create a binding from a getter and setter on the model.
    fn bind<M, FGet, FSet>(
        get: FGet,
        set: FSet,
    ) -> impl FnOnce(&mut ViewContext<M>) -> Stateful<Div>
    where
        M: 'static,
        FGet: Fn(&M) -> S + 'static,
        FSet: Fn(&mut M, S) + 'static;
}
