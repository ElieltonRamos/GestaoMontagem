import { DashboardStats } from '../types'
import { tauriClient } from './tauri.client'

export class StatsService {
  async getStats(): Promise<DashboardStats> {
    try {
      return await tauriClient.invoke<DashboardStats>('get_dashboard_stats')
    } catch (error) {
      console.error('[StatsService] getStats:', error)
      return {
        totalAssemblies: 0,
        totalValue: 0,
        activeAssemblers: 0
      }
    }
  }
}

export const statsService = new StatsService()
