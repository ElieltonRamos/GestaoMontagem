import { assembliesService } from './assemblies.service'
import { assemblersService } from './assemblers.service'
import { DashboardStats } from '../types'

export class StatsService {
  getStats(): DashboardStats {
    const assemblies = assembliesService.getAll()
    const assemblers = assemblersService.getAll()

    const totalValue = assemblies.reduce((sum, a) => sum + a.amountPaid, 0)

    return {
      totalAssemblies: assemblies.length,
      totalValue,
      activeAssemblers: assemblers.length,
    }
  }
}

export const statsService = new StatsService()
