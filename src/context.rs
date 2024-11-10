use crate::engine::Engine;

pub struct ExecutionContext<E>
where
    E: Engine,
{
    engine: E,
}

pub trait Context<E>
where
    E: Engine,
{
}
