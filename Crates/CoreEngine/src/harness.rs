use std::collections::BTreeMap;

use crate::policy::DataSensitivity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillDescriptor {
    pub id: String,
    pub manual_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookDescriptor {
    pub id: String,
    pub sensitivity: DataSensitivity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryPolicy {
    pub portable_text: bool,
    pub human_editable: bool,
    pub sensitive_by_default: bool,
}

impl Default for MemoryPolicy {
    fn default() -> Self {
        Self {
            portable_text: true,
            human_editable: true,
            sensitive_by_default: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessCatalogError {
    EmptySkillId,
    DuplicateSkillId(String),
}

#[derive(Debug, Default)]
pub struct HarnessCatalog {
    skills: BTreeMap<String, SkillDescriptor>,
}

impl HarnessCatalog {
    pub fn register_skill(&mut self, skill: SkillDescriptor) -> Result<(), HarnessCatalogError> {
        if skill.id.trim().is_empty() {
            return Err(HarnessCatalogError::EmptySkillId);
        }
        if self.skills.contains_key(&skill.id) {
            return Err(HarnessCatalogError::DuplicateSkillId(skill.id));
        }
        self.skills.insert(skill.id.clone(), skill);
        Ok(())
    }

    pub fn skill(&self, id: &str) -> Option<&SkillDescriptor> {
        self.skills.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_defaults_to_sensitive_portable_text() {
        let policy = MemoryPolicy::default();

        assert!(policy.portable_text);
        assert!(policy.human_editable);
        assert!(policy.sensitive_by_default);
    }
}
