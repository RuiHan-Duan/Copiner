#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowState {
    Clarify,
    Todos,
    Plan,
    AwaitingApproval,
    Execute,
    Feynman,
    Archive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowEvent {
    RequirementsClear,
    TodosReady,
    PlanReady,
    Approve,
    ExecutionComplete,
    ExplanationAccepted,
    ArchiveComplete,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkflowError {
    InvalidTransition {
        from: WorkflowState,
        event: WorkflowEvent,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Workflow {
    state: WorkflowState,
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            state: WorkflowState::Clarify,
        }
    }
}

impl Workflow {
    pub fn state(&self) -> WorkflowState {
        self.state
    }

    pub fn apply(&mut self, event: WorkflowEvent) -> Result<WorkflowState, WorkflowError> {
        self.state = match (self.state, event) {
            (WorkflowState::Clarify, WorkflowEvent::RequirementsClear) => WorkflowState::Todos,
            (WorkflowState::Todos, WorkflowEvent::TodosReady) => WorkflowState::Plan,
            (WorkflowState::Plan, WorkflowEvent::PlanReady) => WorkflowState::AwaitingApproval,
            (WorkflowState::AwaitingApproval, WorkflowEvent::Approve) => WorkflowState::Execute,
            (WorkflowState::Execute, WorkflowEvent::ExecutionComplete) => WorkflowState::Feynman,
            (WorkflowState::Feynman, WorkflowEvent::ExplanationAccepted) => WorkflowState::Archive,
            (WorkflowState::Archive, WorkflowEvent::ArchiveComplete) => WorkflowState::Clarify,
            (from, event) => return Err(WorkflowError::InvalidTransition { from, event }),
        };
        Ok(self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_workflow_returns_to_clarify() {
        let mut workflow = Workflow::default();
        let events = [
            WorkflowEvent::RequirementsClear,
            WorkflowEvent::TodosReady,
            WorkflowEvent::PlanReady,
            WorkflowEvent::Approve,
            WorkflowEvent::ExecutionComplete,
            WorkflowEvent::ExplanationAccepted,
            WorkflowEvent::ArchiveComplete,
        ];

        for event in events {
            workflow.apply(event).expect("valid workflow transition");
        }

        assert_eq!(workflow.state(), WorkflowState::Clarify);
    }

    #[test]
    fn approval_is_required_before_execution() {
        let mut workflow = Workflow {
            state: WorkflowState::AwaitingApproval,
        };

        assert_eq!(
            workflow.apply(WorkflowEvent::ExecutionComplete),
            Err(WorkflowError::InvalidTransition {
                from: WorkflowState::AwaitingApproval,
                event: WorkflowEvent::ExecutionComplete,
            })
        );
        assert_eq!(
            workflow.apply(WorkflowEvent::Approve),
            Ok(WorkflowState::Execute)
        );
    }
}
