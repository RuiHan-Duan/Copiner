use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPermission {
    FileRead,
    FileWrite,
    Network,
    EnterpriseData,
    ExternalAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectLevel {
    ReadOnly,
    LocalMutation,
    ExternalMutation,
}

impl EffectLevel {
    pub fn requires_approval(self) -> bool {
        self != Self::ReadOnly
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionApproval {
    NotRequired,
    Pending,
    Approved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub name: String,
    pub permissions: Vec<ToolPermission>,
    pub effect: EffectLevel,
}

impl ToolDescriptor {
    pub fn new(name: impl Into<String>, effect: EffectLevel) -> Self {
        Self {
            name: name.into(),
            permissions: Vec::new(),
            effect,
        }
    }

    pub fn can_execute(&self, approval: ActionApproval) -> bool {
        if self.effect.requires_approval() {
            approval == ActionApproval::Approved
        } else {
            matches!(
                approval,
                ActionApproval::NotRequired | ActionApproval::Approved
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRegistryError {
    EmptyName,
    DuplicateName(String),
}

#[derive(Debug, Default)]
pub struct ToolRegistry {
    tools: BTreeMap<String, ToolDescriptor>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: ToolDescriptor) -> Result<(), ToolRegistryError> {
        if tool.name.trim().is_empty() {
            return Err(ToolRegistryError::EmptyName);
        }
        if self.tools.contains_key(&tool.name) {
            return Err(ToolRegistryError::DuplicateName(tool.name));
        }
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ToolDescriptor> {
        self.tools.get(name)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_action_cannot_execute_while_pending() {
        let tool = ToolDescriptor::new("send-mail", EffectLevel::ExternalMutation);

        assert!(!tool.can_execute(ActionApproval::Pending));
        assert!(tool.can_execute(ActionApproval::Approved));
    }

    #[test]
    fn duplicate_tool_name_is_rejected() {
        let mut registry = ToolRegistry::default();
        registry
            .register(ToolDescriptor::new("search", EffectLevel::ReadOnly))
            .expect("first registration");

        assert_eq!(
            registry.register(ToolDescriptor::new("search", EffectLevel::ReadOnly)),
            Err(ToolRegistryError::DuplicateName("search".to_owned()))
        );
    }
}
