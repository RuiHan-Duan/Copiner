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

export const workflowStages: readonly WorkflowStageView[] = [
  { id: 'clarify', label: '澄清' },
  { id: 'plan', label: '规划' },
  { id: 'execute', label: '执行' },
  { id: 'archive', label: '总结归档' },
]

export const emptyShellModel: ShellViewModel = {
  activeStage: 'clarify',
  connectionLabel: '本地模式 · 未连接模型',
  suggestions: ['检索 BIOS / BMC 文档', '整理邮件与待办', '分析问题单'],
  providerConfigured: false,
}
