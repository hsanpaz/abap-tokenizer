# Definir el directorio raíz del proyecto
$projectRoot = ".\abap-tokenizer"

# Crear el directorio raíz
New-Item -ItemType Directory -Path $projectRoot -Force

# Definir la estructura de directorios
$directories = @(
    "src",
    "src\tokenizer",
    "src\config",
    "src\matchers",
    "src\error",
    "src\utils",
    "tests",
    "tests\test_data",
    "benches",
    "examples",
    "config"
)

# Crear los directorios
foreach ($dir in $directories) {
    New-Item -ItemType Directory -Path (Join-Path $projectRoot $dir) -Force
}

# Crear archivos básicos
$files = @(
    "Cargo.toml",
    "README.md",
    "src\main.rs",
    "src\lib.rs",
    "src\tokenizer\mod.rs",
    "src\tokenizer\flexible_tokenizer.rs",
    "src\tokenizer\token.rs",
    "src\tokenizer\token_type.rs",
    "src\config\mod.rs",
    "src\config\tokenizer_config.rs",
    "src\config\toml_loader.rs",
    "src\matchers\mod.rs",
    "src\matchers\keyword_matcher.rs",
    "src\matchers\identifier_matcher.rs",
    "src\matchers\number_matcher.rs",
    "src\matchers\string_matcher.rs",
    "src\matchers\comment_matcher.rs",
    "src\matchers\operator_matcher.rs",
    "src\matchers\special_rule_matcher.rs",
    "src\error\mod.rs",
    "src\error\tokenizer_error.rs",
    "src\utils\mod.rs",
    "src\utils\logger.rs",
    "tests\integration_tests.rs",
    "benches\tokenizer_benchmark.rs",
    "examples\simple_tokenization.rs",
    "config\default_config.toml"
)

# Crear los archivos
foreach ($file in $files) {
    New-Item -ItemType File -Path (Join-Path $projectRoot $file) -Force
}

Write-Host "Estructura de directorios y archivos creada con éxito en $projectRoot"