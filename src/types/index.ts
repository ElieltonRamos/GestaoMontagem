export interface Assembler {
  id: string;
  name: string;
  phone: string;
  cpf?: string;
  address?: string;
  createdAt: string;
}

export interface Assembly {
  id: string;
  assemblerId: string;
  assemblerName: string;
  orderNumber: string;
  orderValue: number;
  percentagePaid: number;
  amountPaid: number;
  furnitureDescription: string;
  date: string;
  createdAt: string;
}
export interface DashboardStats {
  totalAssemblies: number;
  totalValue: number;
  activeAssemblers: number;
}
