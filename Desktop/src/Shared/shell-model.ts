export type WorkflowStageId = 'clarify' | 'plan' | 'execute' | 'archive'

export interface WorkflowStageView {
  id: WorkflowStageId
  label: string
}

export interface ShellViewModel {
  activeStage: WorkflowStageId
  connectionLabel: string
  suggestions: readonly string[]
  providerConfigured: boolean
}

export type HostMode = 'checking' | 'ready' | 'browser-preview' | 'error'

export interface WorkflowStateView {
  id: string
  label: string
  stage: WorkflowStageId
}

export interface WorkflowNextActionView {
  id: string
  label: string
  requiresApproval: boolean
}

export interface WorkflowEventView {
  sequence: number
  actionLabel: string
  state: WorkflowStateView
}

export interface WorkflowSnapshot {
  taskTitle: string
  state: WorkflowStateView
  nextAction: WorkflowNextActionView | null
  completed: boolean
  completionSummary: string | null
  events: WorkflowEventView[]
}

export const workflowStages: readonly WorkflowStageView[] = [
  { id: 'clarify', label: '澄清' },
  { id: 'plan', label: '规划' },
  { id: 'execute', label: '执行' },
  { id: 'archive', label: '总结归档' },
]

export const emptyShellModel: ShellViewModel = {
  activeStage: 'clarify',
  connectionLabel: '正在检查本地 Harness',
  suggestions: ['检索 BIOS / BMC 文档', '整理邮件与待办', '分析问题单'],
  providerConfigured: false,
}
