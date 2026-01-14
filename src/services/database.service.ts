// Mock service using localStorage (will be replaced with Tauri SQLite later)

import { Assembler, Assembly } from '../types';

class StorageService {
  private getItem<T>(key: string): T | null {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : null;
  }

  private setItem<T>(key: string, value: T): void {
    localStorage.setItem(key, JSON.stringify(value));
  }

  // Assemblers
  getAssemblers(): Assembler[] {
    return this.getItem<Assembler[]>('assemblers') || [];
  }

  assemblerExists(name: string): boolean {
    const assemblers = this.getAssemblers();
    return assemblers.some((a) => a.name.toLowerCase() === name.toLowerCase());
  }

  saveAssembler(name: string, phone: string, cpf?: string, address?: string): void {
    const assemblers = this.getAssemblers();
    const newAssembler: Assembler = {
      id: crypto.randomUUID(),
      name,
      phone,
      cpf,
      address,
      createdAt: new Date().toISOString(),
    };
    assemblers.push(newAssembler);
    this.setItem('assemblers', assemblers);
  }

  getAssemblerById(id: string): Assembler | null {
    const assemblers = this.getAssemblers();
    return assemblers.find((a) => a.id === id) || null;
  }

  updateAssembler(id: string, name: string, phone: string, cpf?: string, address?: string): void {
    const assemblers = this.getAssemblers();
    const index = assemblers.findIndex((a) => a.id === id);

    if (index !== -1) {
      assemblers[index] = {
        ...assemblers[index],
        name,
        phone,
        cpf,
        address,
      };
      this.setItem('assemblers', assemblers);
    }
  }

  deleteAssembler(id: string): void {
    const assemblers = this.getAssemblers();
    const filtered = assemblers.filter((a) => a.id !== id);
    this.setItem('assemblers', filtered);
  }

  assemblerExistsExcept(name: string, excludeId: string): boolean {
    const assemblers = this.getAssemblers();
    return assemblers.some((a) => a.name.toLowerCase() === name.toLowerCase() && a.id !== excludeId);
  }

  getAssemblies(): Assembly[] {
    return this.getItem<Assembly[]>('assemblies') || [];
  }

  assemblyExists(orderNumber: string): boolean {
    const assemblies = this.getAssemblies();
    return assemblies.some((a) => a.orderNumber === orderNumber);
  }

  saveAssembly(
    assemblerId: string,
    assemblerName: string,
    orderNumber: string,
    orderValue: number,
    percentagePaid: number,
    amountPaid: number,
    furnitureDescription: string,
    date: string
  ): void {
    const assemblies = this.getAssemblies();
    const newAssembly: Assembly = {
      id: crypto.randomUUID(),
      assemblerId,
      assemblerName,
      orderNumber,
      orderValue,
      percentagePaid,
      amountPaid,
      furnitureDescription,
      date,
      createdAt: new Date().toISOString(),
    };
    assemblies.push(newAssembly);
    this.setItem('assemblies', assemblies);
  }

  getAssemblyById(id: string): Assembly | null {
    const assemblies = this.getAssemblies();
    return assemblies.find((a) => a.id === id) || null;
  }

  updateAssembly(
    id: string,
    assemblerId: string,
    assemblerName: string,
    orderNumber: string,
    orderValue: number,
    percentagePaid: number,
    amountPaid: number,
    furnitureDescription: string,
    date: string
  ): void {
    const assemblies = this.getAssemblies();
    const index = assemblies.findIndex((a) => a.id === id);

    if (index !== -1) {
      assemblies[index] = {
        ...assemblies[index],
        assemblerId,
        assemblerName,
        orderNumber,
        orderValue,
        percentagePaid,
        amountPaid,
        furnitureDescription,
        date,
      };
      this.setItem('assemblies', assemblies);
    }
  }

  deleteAssembly(id: string): void {
    const assemblies = this.getAssemblies();
    const filtered = assemblies.filter((a) => a.id !== id);
    this.setItem('assemblies', filtered);
  }

  assemblyExistsExcept(orderNumber: string, excludeId: string): boolean {
    const assemblies = this.getAssemblies();
    return assemblies.some((a) => a.orderNumber === orderNumber && a.id !== excludeId);
  }

  getStats(): { totalAssemblies: number; totalValue: number; activeAssemblers: number } {
    const assemblies = this.getAssemblies();
    const assemblers = this.getAssemblers();

    const totalValue = assemblies.reduce((sum, assembly) => sum + assembly.amountPaid, 0);

    return {
      totalAssemblies: assemblies.length,
      totalValue,
      activeAssemblers: assemblers.length,
    };
  }
}

export const storageService = new StorageService();
