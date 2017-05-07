#[derive(Debug)]
pub enum WsdlEvent {
    StartDefinition {
        name: Option<String>,
        target_namespace: Option<String>
    },
    EndDefinition
}
