#!/bin/bash

# Definir el directorio raíz del proyecto
PROJECT_ROOT="./abap-tokenizer"

# Crear el directorio raíz
mkdir -p "$PROJECT_ROOT"

# Definir la estructura de directorios
DIRECTORIES=(
    "src"
    "src/tokenizer"
    "src/config"
    "src/matchers"
    "src/error"
    "src/utils"
    "tests"
    "tests/test_data"
    "benches"
    "examples"
    "config"
)

# Crear los directorios
for dir in "${DIRECTORIES[@]}"; do
    mkdir -p "$PROJECT_ROOT/$dir"
done

# Crear archivos básicos
FILES=(
    "Cargo.toml"
    "README.md"
    "src/main.rs"
    "src/lib.rs"
    "src/tokenizer/mod.rs"
    "src/tokenizer/flexible_tokenizer.rs"
    "src/tokenizer/token.rs"
    "src/tokenizer/token_type.rs"
    "src/config/mod.rs"
    "src/config/tokenizer_config.rs"
    "src/config/toml_loader.rs"
    "src/matchers/mod.rs"
    "src/matchers/keyword_matcher.rs"
    "src/matchers/identifier_matcher.rs"
    "src/matchers/number_matcher.rs"
    "src/matchers/string_matcher.rs"
    "src/matchers/comment_matcher.rs"
    "src/matchers/operator_matcher.rs"
    "src/matchers/special_rule_matcher.rs"
    "src/error/mod.rs"
    "src/error/tokenizer_error.rs"
    "src/utils/mod.rs"
    "src/utils/logger.rs"
    "tests/integration_tests.rs"
    "benches/tokenizer_benchmark.rs"
    "examples/simple_tokenization.rs"
    "config/default_config.toml"
)

# Crear los archivos
for file in "${FILES[@]}"; do
    touch "$PROJECT_ROOT/$file"
done

echo "Estructura de directorios y archivos creada con éxito en $PROJECT_ROOT"