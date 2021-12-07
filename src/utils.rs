
macro_rules! some_if{
    ($condition:expr,$some:expr) => {{
    match $condition{
    true => Some($some),
    _ => None,
    }
    }}
}

pub(crate) use some_if;