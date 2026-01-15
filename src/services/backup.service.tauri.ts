import { tauriClient } from './tauri.client'
import { save } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

export interface BackupInfo {
  filename: string
  path: string
  size: number
  createdAt: number
}

export class BackupService {
  async createBackup(): Promise<string | null> {
    try {
      return await tauriClient.invoke<string>('create_backup')
    } catch (error) {
      console.error('[BackupService] createBackup:', error)
      return null
    }
  }

  async createAutoBackup(): Promise<{ created: boolean; message: string }> {
    try {
      const result = await tauriClient.invoke<string>('create_auto_backup')
      
      // Se retornou "Backup de hoje já existe", não foi criado
      if (result.includes('já existe')) {
        return { created: false, message: result }
      }
      
      return { created: true, message: `Backup criado: ${result}` }
    } catch (error) {
      console.error('[BackupService] createAutoBackup:', error)
      return { created: false, message: 'Erro ao criar backup automático' }
    }
  }

  async listBackups(): Promise<BackupInfo[]> {
    try {
      return await tauriClient.invoke<BackupInfo[]>('list_backups')
    } catch (error) {
      console.error('[BackupService] listBackups:', error)
      return []
    }
  }

  async restoreBackup(backupPath: string): Promise<boolean> {
    try {
      await tauriClient.invoke('restore_backup', { backupPath })
      return true
    } catch (error) {
      console.error('[BackupService] restoreBackup:', error)
      return false
    }
  }

  async deleteBackup(backupPath: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('delete_backup', { backupPath })
    } catch (error) {
      console.error('[BackupService] deleteBackup:', error)
      return false
    }
  }

  async exportBackup(): Promise<string | null> {
    try {
      const filePath = await save({
        defaultPath: `backup_${new Date().toISOString().split('T')[0]}.db`,
        filters: [{ 
          name: 'Database', 
          extensions: ['db'] 
        }]
      })

      if (!filePath) return null

      return await invoke<string>('export_backup_to_location', {
        destination: filePath
      })
    } catch (error) {
      console.error('[BackupService] exportBackup:', error)
      return null
    }
  }

  async importBackup(): Promise<boolean> {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog')
      
      const filePath = await open({
        multiple: false,
        filters: [{ 
          name: 'Database', 
          extensions: ['db'] 
        }]
      })

      if (!filePath) return false

      await invoke('import_backup_from_location', {
        source: filePath
      })

      return true
    } catch (error) {
      console.error('[BackupService] importBackup:', error)
      return false
    }
  }

  formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`
  }

  formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString('pt-BR')
  }
}

export const backupService = new BackupService()
