# KWIC em Rust - Implementação Funcional

![Rust](https://img.shields.io/badge/lang-Rust-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

## 📌 Visão Geral

Implementação do algoritmo **Keyword In Context (KWIC)** em Rust, seguindo princípios de programação funcional. O sistema processa textos, extrai palavras-chave (ignorando stop words) e gera concordâncias ordenadas alfabeticamente.

## ✨ Funcionalidades

- ✅ Processamento eficiente de textos com Rust
- ✅ Filtragem de stop words personalizável
- ✅ Suporte completo a Unicode (acentos, caracteres especiais)
- ✅ Ordenação case-sensitive ou case-insensitive
- ✅ Saída para console ou arquivo
- ✅ Testes unitários abrangentes

## 🛠️ Como Usar

### Pré-requisitos

- Rust 1.70+ ([instalação](https://www.rust-lang.org/tools/install))
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


| Opção           | Comando                        | Descrição                               |
| ----------------- | ------------------------------ | ----------------------------------------- |
| Stop words        | `--stop-words meu_arquivo.txt` | Arquivo com palavras para ignorar         |
| Case sensitive    | `--case-sensitive`             | Ativa ordenação sensível a maiúsculas |
| Saída em arquivo | `--output resultado.txt`       | Salva resultados em arquivo               |

### 🔧 Testando

```bash
cargo test  # Executa todos os testes unitários
```

## 📚 Estrutura do Código

```rust
src/
├── main.rs          // Interface de linha de comando
├── lib.rs           // Lógica principal do KWIC
└── tests/           // Testes unitários
```

### Princípios Implementados

1. **Imutabilidade**: Dados nunca são alterados, apenas transformados
2. **Funções Puras**: Nenhum estado externo é modificado
3. **Composição**: Pequenas funções combinadas para formar operações complexas
4. **Pattern Matching**: Tratamento elegante de erros e opções

## 📊 Exemplo de Saída

Para o arquivo `input.txt`:

```
The quick brown fox
A brown cat sat
```

A saída será:

```
brown: brown fox the quick (from "The quick brown fox")
brown: brown cat sat a (from "A brown cat sat")
cat: cat sat a brown (from "A brown cat sat")
fox: fox the quick brown (from "The quick brown fox")
quick: quick brown fox the (from "The quick brown fox")
sat: sat a brown cat (from "A brown cat sat")
```

## 📝 Formatos de Entrada Válidos

1. **Frases simples** (uma por linha)
2. **Com pontuação** (apóstrofes, hífens)
3. **Unicode** (acentos, caracteres especiais)
4. **Stop words** (serão automaticamente filtradas)

Exemplo:

```
O rato roeu a roupa
E-mail messaging system
Don't stop believing
```

## 📜 Licença

MIT - Consulte o arquivo [LICENSE](LICENSE) para detalhes.

---

Desenvolvido por Carlos Eduardo Xavier | 200036131
Disciplina: Técnicas de Programação 2 | 2025.1
