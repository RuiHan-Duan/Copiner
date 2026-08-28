import { invoke, isTauri } from '@tauri-apps/api/core'

import type { WorkflowSnapshot } from './shell-model'

export interface RuntimeStatus {
  mode: 'local_harness'
  modelConfigured: false
}

export function hasDesktopHost(): boolean {
  return isTauri()
}

export function getRuntimeStatus(): Promise<RuntimeStatus> {
  return invoke<RuntimeStatus>('runtime_status')
}

export function getLocalHarnessSnapshot(): Promise<WorkflowSnapshot | null> {
  return invoke<WorkflowSnapshot | null>('local_harness_snapshot')
}

export function startLocalHarness(taskTitle: string): Promise<WorkflowSnapshot> {
  return invoke<WorkflowSnapshot>('start_local_harness', { taskTitle })
}

export function advanceLocalHarness(action: string): Promise<WorkflowSnapshot> {
  return invoke<WorkflowSnapshot>('advance_local_harness', { action })
}

export function resetLocalHarness(): Promise<void> {
  return invoke<void>('reset_local_harness')
}

export function commandErrorMessage(error: unknown): string {
  if (typeof error === 'object' && error !== null && 'message' in error) {
    const message = (error as { message?: unknown }).message
    if (typeof message === 'string') return message
  }
  if (typeof error === 'string') return error
  return '本地 Harness 暂时不可用，请重试。'
}
