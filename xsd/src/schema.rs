
pub enum Form {
    Qualified,
    Unqualified
}

pub struct Schema {
    pub id: Option<String>,
    pub attribute_form_default: Form,
    pub element_form_default: Form
}
