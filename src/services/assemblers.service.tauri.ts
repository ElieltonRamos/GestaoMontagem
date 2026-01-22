import type { Assembler } from '../types';
import { tauriClient } from './tauri.client';

export class AssemblersService {
  async getAll(): Promise<Assembler[]> {
    try {
      return await tauriClient.invoke<Assembler[]>('get_all_assemblers');
    } catch (error) {
      console.error('[AssemblersService] getAll:', error);
      return [];
    }
  }

  async existsByName(name: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('assembler_exists_by_name', { name });
    } catch (error) {
      console.error('[AssemblersService] existsByName:', error);
      return false;
    }
  }

  async existsByNameExceptId(name: string, excludeId: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('assembler_exists_by_name_except_id', {
        name,
        excludeId,
      });
    } catch (error) {
      console.error('[AssemblersService] existsByNameExceptId:', error);
      return false;
    }
  }

  async create(
    name: string,
    phone: string,
    document?: string, // CPF ou CNPJ limpo
    address?: string,
  ): Promise<Assembler | null> {
    try {
      return await tauriClient.invoke<Assembler>('create_assembler', {
        name,
        phone,
        document, // Renomeado
        address,
      });
    } catch (error) {
      console.error('[AssemblersService] create:', error);
      return null;
    }
  }

  async getById(id: string): Promise<Assembler | null> {
    try {
      return await tauriClient.invoke<Assembler>('get_assembler_by_id', { id });
    } catch (error) {
      console.error('[AssemblersService] getById:', error);
      return null;
    }
  }

  async update(
    id: string,
    name: string,
    phone: string,
    document?: string, // CPF ou CNPJ limpo
    address?: string,
  ): Promise<boolean> {
    try {
      await tauriClient.invoke('update_assembler', {
        id,
        name,
        phone,
        document, // Renomeado
        address,
      });
      return true;
    } catch (error) {
      console.error('[AssemblersService] update:', error);
      return false;
    }
  }

  async delete(id: string): Promise<boolean> {
    try {
      return await tauriClient.invoke<boolean>('delete_assembler', { id });
    } catch (error) {
      console.error('[AssemblersService] delete:', error);
      return false;
    }
  }
}

export const assemblersService = new AssemblersService();
