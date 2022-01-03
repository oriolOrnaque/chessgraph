
pub trait GvizNode {
    fn id(&self) -> String;
    fn id_with_attributes(&self) -> String;
}
