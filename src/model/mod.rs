pub trait Keyed {
    fn pk(&self) -> String;
    fn sk(&self) -> String;
}
