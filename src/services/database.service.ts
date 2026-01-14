// Mock service using localStorage (will be replaced with Tauri SQLite later)
class StorageService {
  private getItem<T>(key: string): T | null {
    const item = localStorage.getItem(key)
    return item ? JSON.parse(item) : null
  }

  private setItem<T>(key: string, value: T): void {
    localStorage.setItem(key, JSON.stringify(value))
  }

  // Assemblers
  getAssemblers(): string[] {
    return this.getItem<string[]>('assemblers') || []
  }

  saveAssembler(name: string): void {
    const assemblers = this.getAssemblers()
    assemblers.push(name)
    this.setItem('assemblers', assemblers)
  }

  // Assemblies
  getAssemblies(): Array<{ orderId: string; assembler: string; value: number; date: string }> {
    return this.getItem('assemblies') || []
  }

  saveAssembly(orderId: string, assembler: string, value: number): void {
    const assemblies = this.getAssemblies()
    assemblies.push({
      orderId,
      assembler,
      value,
      date: new Date().toISOString()
    })
    this.setItem('assemblies', assemblies)
  }

  // Stats
  getStats(): { totalAssemblies: number; totalValue: number; activeAssemblers: number } {
    const assemblies = this.getAssemblies()
    const assemblers = this.getAssemblers()
    
    const totalValue = assemblies.reduce((sum, assembly) => sum + assembly.value, 0)
    
    return {
      totalAssemblies: assemblies.length,
      totalValue,
      activeAssemblers: assemblers.length
    }
  }
}

export const storageService = new StorageService()
