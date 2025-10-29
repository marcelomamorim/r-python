# RPython 🚀

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![GitHub issues](https://img.shields.io/github/issues/UnBCIC-TP2/r-python)](https://github.com/UnBCIC-TP2/r-python/issues)
[![CI Status](https://img.shields.io/github/actions/workflow/status/UnBCIC-TP2/r-python/ci.yml?branch=main&label=ci-status&color=blue)](https://github.com/UnBCIC-TP2/r-python/actions)

Um compilador/interpretador experimental escrito em Rust para uma linguagem com sintaxe inspirada no Python, mas com regras explícitas de tipagem estática e blocos delimitados por `end`. O projeto nasceu como ferramenta acadêmica para explorar conceitos de construção de compiladores e interpretação de linguagens.

> 🔎 Precisa copiar o README inteiro? A versão crua está disponível em `README.md`, bastando abrir o arquivo em modo *Raw* no GitHub ou executar `cat README.md` após clonar o repositório.

## 🧭 Índice

- [Visão Geral](#-visão-geral)
- [Arquitetura em Alto Nível](#-arquitetura-em-alto-nível)
- [Catálogo de Features da Linguagem](#-catálogo-de-features-da-linguagem)
  - [Literais e Operações Numéricas](#literais-e-operações-numéricas)
  - [Lógica Booleana e Comparações](#lógica-booleana-e-comparações)
  - [Listas e Tuplas](#listas-e-tuplas)
  - [Declarações `val`/`var` e Atribuições](#declarações-valvar-e-atribuições)
  - [Controle de Fluxo (`if`/`elif`/`else`, `while`, `for`)](#controle-de-fluxo-ifelifelse-while-for)
  - [Funções Tipadas, Retorno e Recursão](#funções-tipadas-retorno-e-recursão)
  - [Assertions em Tempo de Execução](#assertions-em-tempo-de-execução)
  - [Blocos de Teste Automatizado](#blocos-de-teste-automatizado)
  - [Valores `Result` e Propagação de Erros](#valores-result-e-propagação-de-erros)
  - [Valores `Maybe` (Opcionais)](#valores-maybe-opcionais)
  - [Construtores de Tipos Algébricos (ADT)](#construtores-de-tipos-algébricos-adt)
- [Guia Rápido](#-guia-rápido)
- [Pipeline de Compilação](#-pipeline-de-compilação)
- [Exemplos `.rpy`](#-exemplos-rpy)
- [Documentação Complementar](#-documentação-complementar)
- [Contribuindo](#-contribuindo)

## 📋 Visão Geral

RPython tem como foco:

- **Aprendizado de compiladores:** o código-fonte serve como laboratório para parsing, análise semântica, verificação de tipos e interpretação.
- **Linguagem familiar:** a sintaxe lembra Python, mas inclui recursos clássicos de linguagens fortemente tipadas (anotações obrigatórias em funções, declarações `var`/`val`, ADTs em desenvolvimento).
- **Ferramentas modernas:** todo o pipeline é escrito em Rust, aproveitando segurança de memória e uma rica base de testes automatizados.

## 🏗️ Arquitetura em Alto Nível

- **Parser (`src/parser`)** – Constrói a Árvore de Sintaxe Abstrata (AST) a partir do código-fonte utilizando `nom`.
- **IR/AST (`src/ir`)** – Define as estruturas centrais da linguagem (expressões, declarações, tipos e construtores).
- **Environment (`src/environment`)** – Implementa pilha de escopos para variáveis, funções e tipos.
- **Type Checker (`src/type_checker`)** – Analisa programas para garantir consistência de tipos antes da execução. (Algumas funcionalidades avançadas ainda estão em desenvolvimento.)
- **Interpreter (`src/interpreter`)** – Avalia a AST produzindo resultados de execução.
- **Pretty Printer (`src/pretty_print`)** – Gera representações legíveis da AST, útil para debugging e tooling.

## 🧪 Catálogo de Features da Linguagem

Os módulos de AST, parser, verificação de tipos e interpretador já oferecem um conjunto sólido de recursos — de operações aritméticas a estruturas de controle, passando por coleções, funções tipadas e tratamento de erros. Consulte `src/ir/ast.rs` e `src/parser/parser_stmt.rs` para ver a implementação completa de cada construção apresentada abaixo. A seguir reunimos exemplos autoexplicativos para cada feature disponível hoje.

### Literais e Operações Numéricas

```text
val inteiro = 42;
val real = 3.14;

val soma = inteiro + 8;
val mix = real * 2.0 - inteiro;
val divisao = inteiro / 2;
```

<!-- -->

### Lógica Booleana e Comparações

```text
val idade = 20;
val possui_autorizacao = False;

val maior_de_idade = idade >= 18;
val pode_participar = maior_de_idade or possui_autorizacao;
val precisa_aviso = not maior_de_idade and possui_autorizacao == False;
```

### Listas e Tuplas

```text
val lista_vazia = [];
val linguagens = ["RPython", "Rust", "Python"];
val pares = [(1, "um"), (2, "dois")];
val coordenada = (latitude, longitude, "Norte");
```

> ℹ️ Listas e tuplas aparecem como `ListValue` e `Tuple` na AST, garantindo suporte a coleções homogêneas e heterogêneas (veja `src/ir/ast.rs`).

### Declarações `val`/`var` e Atribuições

```text
val nome = "RPython";      # imutável
var contador = 0;          # mutável

contador = contador + 1;
val saudacao = "Olá, " + nome;
```

> ℹ️ Declarações `val`/`var` e reatribuições são modeladas por `Statement::ValDeclaration`, `Statement::VarDeclaration` e `Statement::Assignment` em `src/ir/ast.rs`.

### Controle de Fluxo (`if`/`elif`/`else`, `while`, `for`)

```text
if nota >= 9:
    conceito = "Excelente";
elif nota >= 7:
    conceito = "Bom";
else:
    conceito = "Recuperação";
end

var total = 0;
while total < 3:
    total = total + 1;
end

val numeros = [1, 2, 3];
var soma = 0;
for n in numeros:
    soma = soma + n;
end
```

> ℹ️ O núcleo reconhece `if`/`elif`/`else`, laços `while` e `for` como variantes específicas da AST, preservando o bloco associado a cada controle de fluxo (implementações em `src/ir/ast.rs` e `src/parser/parser_stmt.rs`).

### Funções Tipadas, Retorno e Recursão

```text
def fibonacci(n: Int) -> Int:
    if n <= 1:
        return n;
    end
    return fibonacci(n - 1) + fibonacci(n - 2);
end;

val resultado = fibonacci(10);
asserttrue(resultado == 55, "Fib(10) deve ser 55");
```

> ℹ️ Funções são representadas por `Function` + `Statement::FuncDef`, com anotações obrigatórias de tipos de parâmetros e retorno (`src/ir/ast.rs` e `src/parser/parser_stmt.rs`).

### Assertions em Tempo de Execução

```text
asserttrue(condicao, "Mensagem de sucesso");
assertfalse(condicao, "Mensagem de erro");
asserteq(resultado, esperado, "Valores devem ser iguais");
assertneq(id_atual, ultimo_id, "IDs não podem coincidir");
```

> ℹ️ Há variantes dedicadas para cada assertiva (`Assert`, `AssertTrue`, `AssertFalse`, `AssertEQ`, `AssertNEQ`) que permitem mensagens customizadas e integração com o runner de testes (detalhes em `src/ir/ast.rs`).

### Blocos de Teste Automatizado

```text
test soma_basica():
    val resultado = 2 + 3;
    asserttrue(resultado == 5, "2 + 3 deve ser 5");
end
```

> ℹ️ O interpretador expõe `Statement::TestDef` para registrar casos de teste e executá-los isoladamente com coleta de resultados (veja `src/ir/ast.rs` e `src/interpreter/statement_execute.rs`).

### Valores `Result` e Propagação de Erros

```text
def dividir(a: Int, b: Int) -> Result[Int, String]:
    if b == 0:
        return Err("Divisão por zero");
    end
    return Ok(a / b);
end;

def dividir_e_incrementar(a: Int, b: Int, inc: Int) -> Result[Int, String]:
    val quociente = dividir(a, b)?;   # Propaga Err automaticamente
    return Ok(quociente + inc);
end;

val resultado = dividir_e_incrementar(10, 2, 1);
assertfalse(is_error(resultado), "Esperávamos um Ok");
```

> ℹ️ Resultado de erros utiliza as variantes `COk`, `CErr`, `Propagate`, `IsError` e `Unwrap` dentro da AST e do interpretador, cobrindo desde a construção até a propagação de falhas (`src/ir/ast.rs` e `src/interpreter/expression_eval.rs`).

### Valores `Maybe` (Opcionais)

```text
val nome = Just("RPython");

if is_nothing(nome):
    assertfalse(True, "Nome não deveria ser Nothing");
end

val texto = nome!;  # unwrap: lança erro se for Nothing
```

> ℹ️ Valores opcionais contam com `CJust`, `CNothing`, `IsNothing` e `Unwrap`, permitindo presença ou ausência de dados com verificação estática (consulte `src/ir/ast.rs` e `src/type_checker/expression_type_checker.rs`).

### Construtores de Tipos Algébricos (ADT)

```text
data Forma:
    | Circulo Int
    | Retangulo Int Int
end

val figura = Circulo(5);
```

> ℹ️ Tipos algébricos são descritos por `Type::TAlgebraicData` e `ValueConstructor`, habilitando construtores nomeados com múltiplos campos (`src/ir/ast.rs`).

> 💡 A sintaxe usa `;` para separar instruções dentro de blocos e a palavra-chave `end` para finalizar estruturas de controle e definições.

## ⚙️ Guia Rápido

### Pré-requisitos

- Rust (última versão estável)
- Cargo (já incluído na instalação padrão do Rust)
- Git

### Clonando o repositório

```bash
git clone https://github.com/UnBCIC-TP2/r-python.git
cd r-python
```

### Compilação e testes

```bash
# Compilar o projeto
cargo build

# Executar a suíte de testes
cargo test
```

### Executando exemplos

Com a nova CLI é possível compilar arquivos `.rpy` diretamente:

```bash
cargo run -- compile examples/hello_world.rpy
```

Por padrão o comando imprime o TAC gerado no terminal. Use `--emit assembly` para produzir assembly nativo (requer o recurso opcional `llvm-backend`).

> **Nota:** habilitar `--features llvm-backend` exige uma instalação local do LLVM 16 visível para o crate `llvm-sys` (variável `LLVM_SYS_160_PREFIX` ou `llvmenv`). Sem essa dependência a compilação com o backend LLVM falhará durante o `cargo build`.

```bash
cargo run --features llvm-backend -- compile examples/hello_world.rpy --emit assembly
```

Informe `--output <arquivo>` para salvar o resultado em disco.

## 🛠️ Pipeline de Compilação

O módulo `ir::tac` provê o conversor de AST para **Three Address Code (TAC)**. Ele cobre operações aritméticas, controle de fluxo e chamadas de função, gerando uma representação intermediária adequada para otimizações e backends. O mesmo módulo expõe `emit_assembly`, que utiliza LLVM (via `inkwell`) quando o recurso `llvm-backend` está habilitado, permitindo converter um programa TAC em assembly nativo.

O comando `r-python compile` integra todas as etapas:

1. Faz o parsing do arquivo `.rpy`.
2. Constrói a AST correspondente.
3. Converte a AST em TAC com `TacGenerator`.
4. Emite o TAC textual ou delega para o backend LLVM.

Erros de parsing ou compilação são exibidos diretamente no terminal, facilitando o diagnóstico.

## 📁 Exemplos `.rpy`

Dois exemplos atualizados acompanham o repositório e servem como ponto de partida:

- `examples/hello_world.rpy`: demonstra declarações `val`/`var`, condicionais e funções tipadas.
- `examples/fibonacci.rpy`: implementa a função recursiva `fibonacci` com asserts de sanidade.

Use os arquivos como base para explorar a linguagem ou para testar o pipeline de compilação.

## 📚 Documentação Complementar

- **[Environment Module](docs/environment.md)** – Escopos, tabelas de símbolos e política de resolução de nomes.
- **[Parser Component](docs/parser.md)** – Estratégias de parsing com `nom`, gramática suportada e exemplos de AST.
- **[Type Checker Module](docs/type_checker.md)** – Regras de tipagem, tipos suportados e roadmap do verificador. *(Em evolução)*

## 🤝 Contribuindo

Contribuições são muito bem-vindas! Antes de abrir issues ou enviar pull requests, consulte nossos guias:

- [Guia de Contribuição em Português](CONTRIBUTING_pt.md)
- [Contributing Guidelines in English](CONTRIBUTING_en.md)

Fique à vontade para sugerir melhorias na linguagem, adicionar novos exemplos ou expandir a documentação.
