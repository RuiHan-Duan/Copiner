use crate::{Workflow, WorkflowError, WorkflowEvent, WorkflowState};

const MAX_TASK_TITLE_CHARS: usize = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowAction {
    ConfirmRequirements,
    ConfirmTodos,
    SubmitPlan,
    ApprovePlan,
    MarkExecutionComplete,
    AcceptExplanation,
    CompleteArchive,
}

impl WorkflowAction {
    fn workflow_event(self) -> WorkflowEvent {
        match self {
            Self::ConfirmRequirements => WorkflowEvent::RequirementsClear,
            Self::ConfirmTodos => WorkflowEvent::TodosReady,
            Self::SubmitPlan => WorkflowEvent::PlanReady,
            Self::ApprovePlan => WorkflowEvent::Approve,
            Self::MarkExecutionComplete => WorkflowEvent::ExecutionComplete,
            Self::AcceptExplanation => WorkflowEvent::ExplanationAccepted,
            Self::CompleteArchive => WorkflowEvent::ArchiveComplete,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowSessionEvent {
    pub sequence: u32,
    pub action: Option<WorkflowAction>,
    pub state: WorkflowState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowSessionSnapshot {
    pub task_title: String,
    pub state: WorkflowState,
    pub next_action: Option<WorkflowAction>,
    pub completed: bool,
    pub events: Vec<WorkflowSessionEvent>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkflowSessionError {
    EmptyTaskTitle,
    TaskTitleTooLong { max_chars: usize },
    SessionCompleted,
    Workflow(WorkflowError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowSession {
    task_title: String,
    workflow: Workflow,
    completed: bool,
    events: Vec<WorkflowSessionEvent>,
}

impl WorkflowSession {
    pub fn new(task_title: impl Into<String>) -> Result<Self, WorkflowSessionError> {
        let task_title = task_title.into().trim().to_owned();
        if task_title.is_empty() {
            return Err(WorkflowSessionError::EmptyTaskTitle);
        }
        if task_title.chars().count() > MAX_TASK_TITLE_CHARS {
            return Err(WorkflowSessionError::TaskTitleTooLong {
                max_chars: MAX_TASK_TITLE_CHARS,
            });
        }

        let workflow = Workflow::default();
        Ok(Self {
            task_title,
            workflow,
            completed: false,
            events: vec![WorkflowSessionEvent {
                sequence: 1,
                action: None,
                state: WorkflowState::Clarify,
            }],
        })
    }

    pub fn advance(
        &mut self,
        action: WorkflowAction,
    ) -> Result<WorkflowSessionSnapshot, WorkflowSessionError> {
        if self.completed {
            return Err(WorkflowSessionError::SessionCompleted);
        }

        let state = self
            .workflow
            .apply(action.workflow_event())
            .map_err(WorkflowSessionError::Workflow)?;
        if action == WorkflowAction::CompleteArchive {
            self.completed = true;
        }
        self.events.push(WorkflowSessionEvent {
            sequence: (self.events.len() + 1) as u32,
            action: Some(action),
            state,
        });

        Ok(self.snapshot())
    }

    pub fn snapshot(&self) -> WorkflowSessionSnapshot {
        WorkflowSessionSnapshot {
            task_title: self.task_title.clone(),
            state: self.workflow.state(),
            next_action: self.next_action(),
            completed: self.completed,
            events: self.events.clone(),
        }
    }

    fn next_action(&self) -> Option<WorkflowAction> {
        if self.completed {
            return None;
        }

        Some(match self.workflow.state() {
            WorkflowState::Clarify => WorkflowAction::ConfirmRequirements,
            WorkflowState::Todos => WorkflowAction::ConfirmTodos,
            WorkflowState::Plan => WorkflowAction::SubmitPlan,
            WorkflowState::AwaitingApproval => WorkflowAction::ApprovePlan,
            WorkflowState::Execute => WorkflowAction::MarkExecutionComplete,
            WorkflowState::Feynman => WorkflowAction::AcceptExplanation,
            WorkflowState::Archive => WorkflowAction::CompleteArchive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_exposes_a_complete_observable_loop() {
        let mut session = WorkflowSession::new("验证本地 Harness").expect("valid title");
        let actions = [
            WorkflowAction::ConfirmRequirements,
            WorkflowAction::ConfirmTodos,
            WorkflowAction::SubmitPlan,
            WorkflowAction::ApprovePlan,
            WorkflowAction::MarkExecutionComplete,
            WorkflowAction::AcceptExplanation,
            WorkflowAction::CompleteArchive,
        ];

        for action in actions {
            assert_eq!(session.snapshot().next_action, Some(action));
            session.advance(action).expect("valid action");
        }

        let snapshot = session.snapshot();
        assert!(snapshot.completed);
        assert_eq!(snapshot.state, WorkflowState::Clarify);
        assert_eq!(snapshot.next_action, None);
        assert_eq!(snapshot.events.len(), 8);
    }

    #[test]
    fn session_cannot_skip_approval() {
        let mut session = WorkflowSession::new("验证批准边界").expect("valid title");
        for action in [
            WorkflowAction::ConfirmRequirements,
            WorkflowAction::ConfirmTodos,
            WorkflowAction::SubmitPlan,
        ] {
            session.advance(action).expect("valid setup action");
        }

        assert_eq!(
            session.advance(WorkflowAction::MarkExecutionComplete),
            Err(WorkflowSessionError::Workflow(
                WorkflowError::InvalidTransition {
                    from: WorkflowState::AwaitingApproval,
                    event: WorkflowEvent::ExecutionComplete,
                }
            ))
        );
        assert_eq!(session.snapshot().state, WorkflowState::AwaitingApproval);
    }

    #[test]
    fn session_rejects_invalid_titles_and_actions_after_completion() {
        assert_eq!(
            WorkflowSession::new("   "),
            Err(WorkflowSessionError::EmptyTaskTitle)
        );
        assert_eq!(
            WorkflowSession::new("x".repeat(MAX_TASK_TITLE_CHARS + 1)),
            Err(WorkflowSessionError::TaskTitleTooLong {
                max_chars: MAX_TASK_TITLE_CHARS,
            })
        );

        let mut session = WorkflowSession::new("完成后不可继续").expect("valid title");
        for action in [
            WorkflowAction::ConfirmRequirements,
            WorkflowAction::ConfirmTodos,
            WorkflowAction::SubmitPlan,
            WorkflowAction::ApprovePlan,
            WorkflowAction::MarkExecutionComplete,
            WorkflowAction::AcceptExplanation,
            WorkflowAction::CompleteArchive,
        ] {
            session.advance(action).expect("valid action");
        }
        assert_eq!(
            session.advance(WorkflowAction::ConfirmRequirements),
            Err(WorkflowSessionError::SessionCompleted)
        );
    }
}
