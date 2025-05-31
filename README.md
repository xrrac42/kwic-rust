# KWIC em Rust - Implementação Funcional

![Rust](https://img.shields.io/badge/lang-Rust-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

## 📌 Visão Geral

Implementação do algoritmo **KWIC (Keyword In Context)** em Rust, com foco em princípios de **programação funcional**. O sistema lê textos, ignora palavras irrelevantes (*stop words*) e gera um índice de palavras-chave em seus contextos originais, ordenado alfabeticamente.

## ✨ Funcionalidades

- ✅ Processamento eficiente de textos com Rust
- ✅ Filtragem personalizada de *stop words*
- ✅ Suporte completo a Unicode (acentos, emojis, caracteres especiais)
- ✅ Ordenação case-sensitive ou case-insensitive
- ✅ Saída no console ou em arquivo
- ✅ Testes unitários organizados e abrangentes

## 🛠️ Como Usar

### Pré-requisitos

- Rust 1.70+ ([como instalar](https://www.rust-lang.org/tools/install))
- Git (opcional)

### 🚀 Execução Básica

1. Clone o repositório:

   ```bash
   git clone https://github.com/seu-usuario/kwic-rust.git
   cd kwic-rust
   ```

2. Execute com o arquivo padrão `input.txt`:

   ```bash
   cargo run --release -- input.txt
   ```

### ⚙️ Opções Avançadas

| Opção             | Comando                            | Descrição                                      |
|-------------------|-------------------------------------|------------------------------------------------|
| *Stop words*      | `--stop-words meu_arquivo.txt`      | Usa um arquivo com palavras a serem ignoradas |
| Case sensitive    | `--case-sensitive`                  | Ativa ordenação sensível a maiúsculas         |
| Saída em arquivo  | `--output resultado.txt`            | Salva a saída no arquivo especificado         |

---

## 🧪 Executando os Testes

Os testes estão localizados no diretório `tests/`, fora de `src/`.

Para rodar todos os testes:

```bash
cargo test
```

Para rodar um teste específico (por nome):

```bash
cargo test nome_do_teste
```

Ou para rodar apenas os testes do arquivo `tests/kwic_tests.rs`:

```bash
cargo test --test kwic_tests
```

---

## 📁 Estrutura do Projeto

```text
kwic-rust/
├── src/
│   ├── main.rs          # Interface de linha de comando
│   └── lib.rs           # Lógica principal do algoritmo KWIC
├── tests/
│   └── kwic_tests.rs    # Testes integrados e unitários
├── input.txt            # Arquivo de entrada padrão (exemplo)
├── Cargo.toml           # Configuração do projeto
└── README.md            # Este arquivo
```

---

## 🧩 Princípios Funcionais Aplicados

1. **Imutabilidade**: Dados nunca são alterados diretamente
2. **Funções puras**: Saída depende apenas da entrada
3. **Composição**: Funções pequenas combinadas em soluções maiores
4. **Pattern matching**: Tratamento de erros claro e idiomático

---

## 📝 Exemplo de Entrada

Arquivo `input.txt`:

```
The quick brown fox
A brown cat sat
```

Saída:

```
brown: brown fox the quick
brown: brown cat sat a
cat: cat sat a brown
fox: fox the quick brown
quick: quick brown fox the
sat: sat a brown cat
```

---

## 📜 Licença

MIT - Consulte o arquivo [LICENSE](LICENSE) para detalhes.

---

Desenvolvido por Carlos Eduardo Xavier | 200036131  
Disciplina: Técnicas de Programação 2 | 2025.1
