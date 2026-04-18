// Placeholder trait – not yet implemented.
#[allow(dead_code)]
pub trait BindablePrimitive<P, S, C>
where
    P: stratum_core::Props,
    S: stratum_core::State + Clone,
    C: stratum_core::Component<Props = P, State = S>,
{
    // Future binding API will go here.
}
