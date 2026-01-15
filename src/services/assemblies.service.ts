import type { Assembly } from '../types'
import { storageClient } from './storage.client'

const ASSEMBLIES_KEY = 'assemblies'

export class AssembliesService {
  getAll(): Assembly[] {
    return storageClient.getItem<Assembly[]>(ASSEMBLIES_KEY) || []
  }

  existsByOrderNumber(orderNumber: string): boolean {
    const normalized = orderNumber.trim()
    return this.getAll().some(a => a.orderNumber.trim() === normalized)
  }

  existsByOrderNumberExceptId(orderNumber: string, excludeId: string): boolean {
    const normalized = orderNumber.trim()
    return this.getAll().some(a => a.id !== excludeId && a.orderNumber.trim() === normalized)
  }

  create(input: Omit<Assembly, 'id' | 'createdAt'>): Assembly {
    const assemblies = this.getAll()
    const newAssembly: Assembly = {
      ...input,
      id: crypto.randomUUID(),
      createdAt: new Date().toISOString(),
    }

    assemblies.push(newAssembly)
    storageClient.setItem(ASSEMBLIES_KEY, assemblies)
    return newAssembly
  }

  getById(id: string): Assembly | null {
    return this.getAll().find(a => a.id === id) || null
  }

  update(id: string, patch: Omit<Assembly, 'id' | 'createdAt'>): void {
    const assemblies = this.getAll()
    const index = assemblies.findIndex(a => a.id === id)
    if (index === -1) return

    assemblies[index] = { ...assemblies[index], ...patch }
    storageClient.setItem(ASSEMBLIES_KEY, assemblies)
  }

  delete(id: string): void {
    const filtered = this.getAll().filter(a => a.id !== id)
    storageClient.setItem(ASSEMBLIES_KEY, filtered)
  }
}

export const assembliesService = new AssembliesService()
