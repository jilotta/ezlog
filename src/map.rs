#[derive(PartialEq, Debug)]
pub struct Map<'a>(pub &'a [(String, String)]);

impl std::fmt::Display for Map<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let arr: &[(String, String)] = self.0;
        write!(f, "{}: {}", arr[0].0, arr[0].1)?;
        for el in &arr[1..] {
            write!(f, "\n     {}: {}", el.0, el.1)?
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! map {
    ($($key:ident $(: $value:expr)?),*) => {
        $crate::Map(&[
            $(map!(@map_one $key $(: $value)?)),*
        ][..])
    };

    (@map_one $key:ident) => {
        (stringify!($key).to_string(), $key.to_string())
    };
    (@map_one $key:ident : $value:expr) => {
        (stringify!($key).to_string(), $value.to_string())
    };
}
