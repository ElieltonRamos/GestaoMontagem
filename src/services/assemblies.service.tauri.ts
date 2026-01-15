import type { Assembly } from '../types'
import { tauriClient } from './tauri.client'

export class AssembliesService {
  async getAll(): Promise<Assembly[]> {
    try {
      return await tauriClient.invoke<Assembly[]>('get_all_assemblies')
    } catch (error) {
      console.error('[AssembliesService] getAll:', error)
      return []
    }
  }

  async existsByOrderNumber(orderNumber: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('assembly_exists_by_order_number', { orderNumber })
    } catch (error) {
      console.error('[AssembliesService] existsByOrderNumber:', error)
      return false
    }
  }

  async existsByOrderNumberExceptId(orderNumber: string, excludeId: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('assembly_exists_by_order_number_except_id', {
        orderNumber,
        excludeId
      })
    } catch (error) {
      console.error('[AssembliesService] existsByOrderNumberExceptId:', error)
      return false
    }
  }

  async create(input: {
    assemblerId: string
    assemblerName: string
    orderNumber: string
    orderValue: number
    percentagePaid: number
    amountPaid: number
    furnitureDescription: string
    date: string
  }): Promise<Assembly | null> {
    try {
      return await tauriClient.invoke<Assembly>('create_assembly', input)
    } catch (error) {
      console.error('[AssembliesService] create:', error)
      return null
    }
  }

  async getById(id: string): Promise<Assembly | null> {
    try {
      return await tauriClient.invoke<Assembly>('get_assembly_by_id', { id })
    } catch (error) {
      console.error('[AssembliesService] getById:', error)
      return null
    }
  }

  async update(id: string, patch: {
    assemblerId: string
    assemblerName: string
    orderNumber: string
    orderValue: number
    percentagePaid: number
    amountPaid: number
    furnitureDescription: string
    date: string
  }): Promise<boolean> {
    try {
      await tauriClient.invoke('update_assembly', { id, ...patch })
      return true
    } catch (error) {
      console.error('[AssembliesService] update:', error)
      return false
    }
  }

  async delete(id: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('delete_assembly', { id })
    } catch (error) {
      console.error('[AssembliesService] delete:', error)
      return false
    }
  }
}

export const assembliesService = new AssembliesService()
