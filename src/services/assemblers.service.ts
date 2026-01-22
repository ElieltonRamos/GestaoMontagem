import type { Assembler } from '../types'
import { storageClient } from './storage.client'

const ASSEMBLERS_KEY = 'assemblers'

export class AssemblersService {
  getAll(): Assembler[] {
    return storageClient.getItem<Assembler[]>(ASSEMBLERS_KEY) || []
  }

  existsByName(name: string): boolean {
    const normalized = name.trim().toLowerCase()
    return this.getAll().some(a => a.name.trim().toLowerCase() === normalized)
  }

  existsByNameExceptId(name: string, excludeId: string): boolean {
    const normalized = name.trim().toLowerCase()
    return this.getAll().some(a => a.id !== excludeId && a.name.trim().toLowerCase() === normalized)
  }

  create(name: string, phone: string, document?: string, address?: string): Assembler {
    const assemblers = this.getAll()
    const newAssembler: Assembler = {
      id: crypto.randomUUID(),
      name,
      phone,
      document,
      address,
      createdAt: new Date().toISOString(),
    }

    assemblers.push(newAssembler)
    storageClient.setItem(ASSEMBLERS_KEY, assemblers)
    return newAssembler
  }

  getById(id: string): Assembler | null {
    return this.getAll().find(a => a.id === id) || null
  }

  update(id: string, name: string, phone: string, document?: string, address?: string): void {
    const assemblers = this.getAll()
    const index = assemblers.findIndex(a => a.id === id)
    if (index === -1) return

    assemblers[index] = { ...assemblers[index], name, phone, document, address }
    storageClient.setItem(ASSEMBLERS_KEY, assemblers)
  }

  delete(id: string): void {
    const filtered = this.getAll().filter(a => a.id !== id)
    storageClient.setItem(ASSEMBLERS_KEY, filtered)
  }
}

export const assemblersService = new AssemblersService()
