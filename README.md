# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

Este projeto tem como objetivo desenvolver um sistema de busca eficiente e escalável para o catálogo de produtos da MegaStore, uma gigante do varejo com milhões de itens disponíveis online. O sistema foi implementado utilizando a linguagem de programação **Rust**, aproveitando suas características de desempenho, segurança e concorrência.

## 🚀 Funcionalidades

- Indexação eficiente de produtos em memória
- Suporte a buscas combinadas por nome, marca e categoria
- Filtros flexíveis (pode buscar apenas por nome, apenas por categoria, etc.)
- Estrutura otimizada com HashMap para desempenho rápido
- Modularização do código

---

## 🛠️ Tecnologias Utilizadas

- **Rust** (linguagem principal)
- `std::collections::HashMap` (estrutura principal)
- Cargo (gerenciador de pacotes e build do Rust)

---

## 📁 Estrutura do Projeto

```bash
megastore_search/
├── src/
│   ├── main.rs           # Arquivo principal com o sistema de busca
│   └── product.rs        # Módulo com definição da struct Product
├── Cargo.toml            # Arquivo de configuração do projeto Rust
├── README.md             # Documentação do projeto
▶️ Como Executar o Projeto
Pré-requisitos
Rust instalado

Passo a passo
bash
Copiar
Editar
# Clone o repositório (ou navegue até sua pasta local)
cd megastore_search

# Compila e executa o projeto
cargo run
✅ Como Executar os Testes
Caso adicione testes (ex: tests/ ou #[cfg(test)]), você poderá rodar com:

bash
Copiar
Editar
cargo test
💡 Exemplos de Uso
O sistema exibe resultados no terminal para diferentes tipos de busca:

text
Copiar
Editar
Busca por 'smartphone', marca 'Samsung':
Product { id: 1, name: "Smartphone Galaxy A14", brand: "Samsung", category: "Eletrônicos" }

Busca por categoria 'Vestuário':
Product { id: 3, name: "Camiseta Esportiva", brand: "Nike", category: "Vestuário" }
Product { id: 4, name: "Tênis de Corrida", brand: "Adidas", category: "Vestuário" }
🧠 Arquitetura do Sistema
Product: Estrutura que representa cada item do catálogo.

matches_filter(): Função que verifica se um produto atende aos critérios da busca.

main.rs: Simula uma base de dados e aplica os filtros combinados.

📈 Desempenho e Escalabilidade
Uso de HashMap torna a busca por categorias, marcas e nomes extremamente rápida (complexidade O(1) para acesso direto).

Sistema modularizado, facilitando extensão para buscas mais complexas e integração futura com banco de dados.

🤝 Contribuições
Este projeto faz parte de uma atividade acadêmica da disciplina Data Structures Strategy and Implementation. Contribuições futuras são bem-vindas para expandir as funcionalidades, testes automatizados, interface CLI, etc.
