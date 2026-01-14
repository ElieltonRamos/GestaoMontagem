export interface Assembler {
  id: string
  name: string
  createdAt: string
}

export interface Assembly {
  id: string
  assemblerId: string
  orderId: string
  value: number
  createdAt: string
}

export interface DashboardStats {
  totalAssemblies: number
  totalValue: number
  activeAssemblers: number
}
