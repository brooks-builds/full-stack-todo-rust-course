use bounce::Atom;

#[derive(Default, PartialEq, Atom)]
pub struct TextInput {
    pub inner: String,
}
