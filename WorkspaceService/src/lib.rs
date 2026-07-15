#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkspaceCapabilities {
    pub rag: bool,
    pub memory_index: bool,
    pub task_queue: bool,
    pub enterprise_gateway: bool,
}

impl WorkspaceCapabilities {
    pub fn is_operational(self) -> bool {
        self.rag || self.memory_index || self.task_queue || self.enterprise_gateway
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_is_explicitly_unconfigured() {
        let capabilities = WorkspaceCapabilities::default();

        assert!(!capabilities.is_operational());
        assert!(!capabilities.rag);
        assert!(!capabilities.memory_index);
        assert!(!capabilities.task_queue);
        assert!(!capabilities.enterprise_gateway);
    }
}
