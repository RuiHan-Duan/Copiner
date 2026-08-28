use std::sync::Mutex;

use copiner_core_engine::{
    WorkflowAction, WorkflowSession, WorkflowSessionError, WorkflowSessionSnapshot, WorkflowState,
};
use serde::{Deserialize, Serialize};
use tauri::State;

const COMPLETION_SUMMARY: &str = "本地 Harness 状态闭环已运行；本次未调用模型、工具或外部系统。";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStatusDto {
    mode: &'static str,
    model_configured: bool,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowActionDto {
    ConfirmRequirements,
    ConfirmTodos,
    SubmitPlan,
    ApprovePlan,
    MarkExecutionComplete,
    AcceptExplanation,
    CompleteArchive,
}

impl From<WorkflowActionDto> for WorkflowAction {
    fn from(value: WorkflowActionDto) -> Self {
        match value {
            WorkflowActionDto::ConfirmRequirements => Self::ConfirmRequirements,
            WorkflowActionDto::ConfirmTodos => Self::ConfirmTodos,
            WorkflowActionDto::SubmitPlan => Self::SubmitPlan,
            WorkflowActionDto::ApprovePlan => Self::ApprovePlan,
            WorkflowActionDto::MarkExecutionComplete => Self::MarkExecutionComplete,
            WorkflowActionDto::AcceptExplanation => Self::AcceptExplanation,
            WorkflowActionDto::CompleteArchive => Self::CompleteArchive,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStateDto {
    id: &'static str,
    label: &'static str,
    stage: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowNextActionDto {
    id: &'static str,
    label: &'static str,
    requires_approval: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowEventDto {
    sequence: u32,
    action_label: &'static str,
    state: WorkflowStateDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowSnapshotDto {
    task_title: String,
    state: WorkflowStateDto,
    next_action: Option<WorkflowNextActionDto>,
    completed: bool,
    completion_summary: Option<&'static str>,
    events: Vec<WorkflowEventDto>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommandErrorDto {
    code: &'static str,
    message: String,
}

#[derive(Default)]
pub struct LocalHarnessState {
    session: Mutex<Option<WorkflowSession>>,
}

impl LocalHarnessState {
    fn start(&self, task_title: String) -> Result<WorkflowSnapshotDto, CommandErrorDto> {
        let session = WorkflowSession::new(task_title).map_err(map_session_error)?;
        let snapshot = map_snapshot(session.snapshot());
        *self.lock_session()? = Some(session);
        Ok(snapshot)
    }

    fn snapshot(&self) -> Result<Option<WorkflowSnapshotDto>, CommandErrorDto> {
        Ok(self
            .lock_session()?
            .as_ref()
            .map(|session| map_snapshot(session.snapshot())))
    }

    fn advance(&self, action: WorkflowActionDto) -> Result<WorkflowSnapshotDto, CommandErrorDto> {
        let mut guard = self.lock_session()?;
        let session = guard.as_mut().ok_or(CommandErrorDto {
            code: "session_not_started",
            message: "请先创建本地 Harness 任务。".to_owned(),
        })?;
        session
            .advance(action.into())
            .map(map_snapshot)
            .map_err(map_session_error)
    }

    fn reset(&self) -> Result<(), CommandErrorDto> {
        *self.lock_session()? = None;
        Ok(())
    }

    fn lock_session(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, Option<WorkflowSession>>, CommandErrorDto> {
        self.session.lock().map_err(|_| CommandErrorDto {
            code: "state_unavailable",
            message: "本地 Harness 状态暂时不可用，请重启应用。".to_owned(),
        })
    }
}

#[tauri::command]
pub fn runtime_status() -> RuntimeStatusDto {
    RuntimeStatusDto {
        mode: "local_harness",
        model_configured: false,
    }
}

#[tauri::command]
pub fn start_local_harness(
    task_title: String,
    state: State<'_, LocalHarnessState>,
) -> Result<WorkflowSnapshotDto, CommandErrorDto> {
    state.start(task_title)
}

#[tauri::command]
pub fn local_harness_snapshot(
    state: State<'_, LocalHarnessState>,
) -> Result<Option<WorkflowSnapshotDto>, CommandErrorDto> {
    state.snapshot()
}

#[tauri::command]
pub fn advance_local_harness(
    action: WorkflowActionDto,
    state: State<'_, LocalHarnessState>,
) -> Result<WorkflowSnapshotDto, CommandErrorDto> {
    state.advance(action)
}

#[tauri::command]
pub fn reset_local_harness(state: State<'_, LocalHarnessState>) -> Result<(), CommandErrorDto> {
    state.reset()
}

fn map_snapshot(snapshot: WorkflowSessionSnapshot) -> WorkflowSnapshotDto {
    WorkflowSnapshotDto {
        task_title: snapshot.task_title,
        state: map_state(snapshot.state),
        next_action: snapshot.next_action.map(map_action),
        completed: snapshot.completed,
        completion_summary: snapshot.completed.then_some(COMPLETION_SUMMARY),
        events: snapshot
            .events
            .into_iter()
            .map(|event| WorkflowEventDto {
                sequence: event.sequence,
                action_label: event.action.map(action_label).unwrap_or("创建本地会话"),
                state: map_state(event.state),
            })
            .collect(),
    }
}

fn map_state(state: WorkflowState) -> WorkflowStateDto {
    let (id, label, stage) = match state {
        WorkflowState::Clarify => ("clarify", "澄清目标", "clarify"),
        WorkflowState::Todos => ("todos", "确认 Todo", "plan"),
        WorkflowState::Plan => ("plan", "确认 Plan", "plan"),
        WorkflowState::AwaitingApproval => ("awaiting_approval", "等待执行批准", "plan"),
        WorkflowState::Execute => ("execute", "本地执行验证", "execute"),
        WorkflowState::Feynman => ("feynman", "解释与验收", "archive"),
        WorkflowState::Archive => ("archive", "总结归档", "archive"),
    };
    WorkflowStateDto { id, label, stage }
}

fn map_action(action: WorkflowAction) -> WorkflowNextActionDto {
    WorkflowNextActionDto {
        id: action_id(action),
        label: action_label(action),
        requires_approval: action == WorkflowAction::ApprovePlan,
    }
}

fn action_id(action: WorkflowAction) -> &'static str {
    match action {
        WorkflowAction::ConfirmRequirements => "confirm_requirements",
        WorkflowAction::ConfirmTodos => "confirm_todos",
        WorkflowAction::SubmitPlan => "submit_plan",
        WorkflowAction::ApprovePlan => "approve_plan",
        WorkflowAction::MarkExecutionComplete => "mark_execution_complete",
        WorkflowAction::AcceptExplanation => "accept_explanation",
        WorkflowAction::CompleteArchive => "complete_archive",
    }
}

fn action_label(action: WorkflowAction) -> &'static str {
    match action {
        WorkflowAction::ConfirmRequirements => "目标已明确",
        WorkflowAction::ConfirmTodos => "Todo 已确认",
        WorkflowAction::SubmitPlan => "提交 Plan",
        WorkflowAction::ApprovePlan => "批准本地执行",
        WorkflowAction::MarkExecutionComplete => "完成本地验证",
        WorkflowAction::AcceptExplanation => "接受说明",
        WorkflowAction::CompleteArchive => "完成归档",
    }
}

fn map_session_error(error: WorkflowSessionError) -> CommandErrorDto {
    match error {
        WorkflowSessionError::EmptyTaskTitle => CommandErrorDto {
            code: "empty_task_title",
            message: "请输入任务名称。".to_owned(),
        },
        WorkflowSessionError::TaskTitleTooLong { max_chars } => CommandErrorDto {
            code: "task_title_too_long",
            message: format!("任务名称不能超过 {max_chars} 个字符。"),
        },
        WorkflowSessionError::SessionCompleted => CommandErrorDto {
            code: "session_completed",
            message: "本次闭环已完成；请重置后创建新任务。".to_owned(),
        },
        WorkflowSessionError::Workflow(_) => CommandErrorDto {
            code: "invalid_transition",
            message: "当前阶段不允许执行该动作。".to_owned(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_state_runs_a_complete_session_and_resets() {
        let state = LocalHarnessState::default();
        let started = state.start("验证桌面闭环".to_owned()).expect("start");
        assert_eq!(started.state.id, "clarify");

        for action in [
            WorkflowActionDto::ConfirmRequirements,
            WorkflowActionDto::ConfirmTodos,
            WorkflowActionDto::SubmitPlan,
            WorkflowActionDto::ApprovePlan,
            WorkflowActionDto::MarkExecutionComplete,
            WorkflowActionDto::AcceptExplanation,
            WorkflowActionDto::CompleteArchive,
        ] {
            state.advance(action).expect("advance");
        }

        let completed = state.snapshot().expect("snapshot").expect("session");
        assert!(completed.completed);
        assert_eq!(completed.completion_summary, Some(COMPLETION_SUMMARY));
        state.reset().expect("reset");
        assert!(state.snapshot().expect("snapshot").is_none());
    }

    #[test]
    fn local_state_rejects_advancing_without_a_session() {
        let state = LocalHarnessState::default();
        assert_eq!(
            state
                .advance(WorkflowActionDto::ConfirmRequirements)
                .expect_err("must reject")
                .code,
            "session_not_started"
        );
    }
}
