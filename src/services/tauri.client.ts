import { invoke } from '@tauri-apps/api/core'

export class TauriClient {
  async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    try {
      return await invoke<T>(command, args)
    } catch (error) {
      console.error(`[TauriClient] Erro ao executar ${command}:`, error)
      throw error
    }
  }
}

export const tauriClient = new TauriClient()
