#[derive(Clone, Debug)]
pub enum Value {
    False,
    True,
    Number(String),
    String(String),
}
