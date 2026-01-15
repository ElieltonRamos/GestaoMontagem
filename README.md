# 🔧 Gestor de Montagens

Aplicação desktop para gerenciar cadastro de montadores e cálculo de comissões sobre montagens, usando Tauri com backend em Rust e SQLite.

**Versão atual:** 0.2.0

---

## 📋 Recursos Principais

- ✅ **CRUD completo de montadores** (nome, telefone, CPF, endereço)
- ✅ **CRUD de montagens** (número do pedido, valor, % pago, descrição de móveis, data)
- ✅ **Cálculo automático de comissões** baseadas em montagens
- ✅ **Backups automáticos e manuais** do banco SQLite, com restauração e exportação
- ✅ **Estatísticas e visualizações** com Chart.js

---

## 🛠 Tecnologias

### Frontend
- **Vue 3** - Framework JavaScript progressivo
- **TypeScript** - Tipagem estática
- **Tailwind CSS** - Framework CSS utilitário
- **Vite** - Build tool e dev server
- **Vue Router** - Roteamento
- **Chart.js** - Gráficos e visualizações

### Backend
- **Rust** - Linguagem de programação do backend
- **Tauri** - Framework para aplicações desktop
- **SQLite** - Banco de dados local

### APIs e Plugins
- `@tauri-apps/api` - API core do Tauri
- `@tauri-apps/plugin-dialog` - Diálogos nativos
- `@tauri-apps/plugin-opener` - Abrir arquivos/URLs

---

## 📦 Pré-requisitos

Antes de começar, certifique-se de ter instalado:

- **Rust** (com cargo-tauri) - [Instalar Rust](https://www.rust-lang.org/tools/install)
- **Node.js** (v16 ou superior) - [Instalar Node.js](https://nodejs.org/)
- **npm** ou **yarn** - Gerenciador de pacotes

### Plataformas Suportadas
- Windows (x86_64)
- Linux (DEB)

---

## 🚀 Instalação Rápida

```bash
# Clonar repositório
git clone <url-do-repositorio> gestor-montagens
cd gestor-montagens

# Instalar dependências do frontend
npm install

# Modo desenvolvimento
npm run tauri:dev
```

---

## 🏗 Comandos de Build

| Comando | Descrição | Saída |
|---------|-----------|-------|
| `npm run tauri:dev` | Executa em modo desenvolvimento | - |
| `npm run tauri:build` | Build para plataforma atual | `src-tauri/target/release/bundle/` |
| `npm run tauri:build-windows` | Build para Windows | `src-tauri/target/release/bundle/msi/*.exe` |
| `npm run tauri:build-linux` | Build para Linux | `src-tauri/target/release/bundle/deb/*.deb` |

### Outputs de Build

Após executar o build, os instaladores estarão em:

**Windows:**
```
src-tauri/target/release/bundle/msi/
└── gestor-montagens_0.2.0_x64_pt-BR.msi
```

**Linux:**
```
src-tauri/target/release/bundle/deb/
└── gestor-montagens_0.2.0_amd64.deb
```

---

## 📁 Estrutura do Projeto

```
gestor-montagens/
├── src/                          # Frontend Vue
│   ├── components/              # Componentes Vue
│   ├── views/                   # Páginas/Views
│   ├── services/                # Serviços Tauri
│   │   ├── database.ts          # Serviço de banco de dados
│   │   └── backup.service.tauri.ts  # Serviço de backups
│   ├── router/                  # Configuração de rotas
│   └── assets/                  # Recursos estáticos
├── src-tauri/                   # Backend Rust
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── lib.rs              # Biblioteca principal
│   │   ├── db.rs               # Módulo de banco de dados
│   │   └── backup.rs           # Módulo de backups
│   ├── Cargo.toml              # Dependências Rust
│   └── tauri.conf.json         # Configuração Tauri
├── package.json                 # Dependências Node
└── README.md                    # Este arquivo
```

---

## 💾 Banco de Dados

O banco de dados SQLite é criado automaticamente na primeira execução:

**Windows:**
```
C:\Users\{Usuario}\AppData\Roaming\gestor-montagens\database.db
```

**Linux:**
```
~/.local/share/gestor-montagens/database.db
```

### Backups Automáticos

Os backups são salvos em:

**Windows:**
```
C:\Users\{Usuario}\AppData\Roaming\gestor-montagens\backups\
```

**Linux:**
```
~/.local/share/gestor-montagens/backups/
```

O sistema mantém automaticamente os **2 backups mais recentes** e cria um backup diário automaticamente.

---

## 🎯 Público-Alvo

Projeto privado desenvolvido para a **Rede Norte**, focado em operações internas de marcenaria e gestão de montagens.

---

## 🔧 Desenvolvimento

### Comandos Úteis

```bash
# Instalar dependências
npm install

# Modo desenvolvimento com hot-reload
npm run tauri:dev

# Build de produção
npm run tauri:build

# Lint do código
npm run lint

# Formatar código
npm run format
```

### Configuração do Rust

Certifique-se de ter as dependências do Tauri instaladas:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

---

## 📝 Comandos Tauri Disponíveis

### Montadores (Assemblers)
- `create_assembler` - Criar novo montador
- `get_all_assemblers` - Listar todos os montadores
- `get_assembler_by_id` - Buscar montador por ID
- `update_assembler` - Atualizar montador
- `delete_assembler` - Excluir montador
- `assembler_exists_by_name` - Verificar se nome existe

### Montagens (Assemblies)
- `create_assembly` - Criar nova montagem
- `get_all_assemblies` - Listar todas as montagens
- `get_assembly_by_id` - Buscar montagem por ID
- `update_assembly` - Atualizar montagem
- `delete_assembly` - Excluir montagem
- `assembly_exists_by_order_number` - Verificar se pedido existe

### Backups
- `create_auto_backup` - Criar backup automático
- `export_backup_to_location` - Exportar backup para local específico
- `import_backup_from_location` - Importar backup
- `list_backups` - Listar todos os backups
- `restore_backup` - Restaurar um backup
- `delete_backup` - Excluir um backup

### Dashboard
- `get_dashboard_stats` - Obter estatísticas gerais

---

## 🐛 Troubleshooting

### Erro ao compilar no Windows
```bash
# Instale o Visual Studio Build Tools
# https://visualstudio.microsoft.com/downloads/
```

### Erro "command not found: cargo-tauri"
```bash
# Instale o CLI do Tauri
cargo install tauri-cli
```

### Banco de dados não é criado
- Verifique as permissões da pasta `AppData/Roaming`
- Execute o aplicativo com permissões de administrador (Windows)

---

## 📄 Licença

Este projeto é de uso privado para a **Rede Norte**.

---

## 🤝 Contribuições

Para contribuições ou issues:

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abra um Pull Request

---

## 📧 Contato

Para dúvidas ou suporte, entre em contato com a equipe de desenvolvimento da Rede Norte.

---

## 🗺 Roadmap

- [ ] Relatórios em PDF
- [ ] Exportação para Excel
- [ ] Multi-usuário com autenticação
- [ ] Dashboard com mais gráficos
- [ ] Notificações de pagamento
- [ ] Backup automático na nuvem

---

**Desenvolvido usando Tauri + Vue + Rust**