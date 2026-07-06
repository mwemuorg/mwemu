# PyMwemu Testing Guide

## Overview

Este proyecto ahora incluye tests completos para pymwemu inspirados en los tests de libmwemu. Los tests de Python **NO** se ejecutan automáticamente con `cargo test --release` porque son archivos Python que requieren un entorno virtual específico.

## Estructura de Tests

```
pymwemu/
├── tests/                          # Tests de Python
│   ├── test_basic_functionality.py
│   ├── test_execution_engine.py
│   ├── test_binary_loading.py
│   ├── test_advanced_features.py
│   └── README.md
├── setup_test_env.py              # Script de configuración
├── run_tests.py                   # Runner de tests
└── Makefile                       # Comandos make
```

## Relación con `cargo test --release`

### Tests de Rust (libmwemu)
```bash
# Desde el directorio raíz del proyecto
cargo test --release
```
Esto ejecuta **solo** los tests de Rust en `libmwemu/src/tests.rs`.

### Tests de Python (pymwemu)
Los tests de Python son **independientes** y requieren configuración especial:

```bash
# Desde el directorio pymwemu/
make setup-env    # Primera vez
make test-python  # Ejecutar tests Python
```

## Configuración Inicial (Tu Método Preferido)

```bash
cd pymwemu/

# 1. Crear entorno virtual
python -m venv .env

# 2. Activar entorno
source .env/bin/activate  # Linux/Mac
# o
.env\Scripts\activate     # Windows

# 3. Instalar maturin
pip install maturin

# 4. Construir e instalar pymwemu
maturin develop --release

# 5. Ejecutar tests
python -m unittest discover -v tests/
```

## Métodos de Ejecución

### Método 1: Makefile (Recomendado)
```bash
cd pymwemu/
make setup-env        # Solo primera vez
make test            # Rust + Python
make test-python     # Solo Python
make test-rust       # Solo Rust
```

### Método 2: Script Automático
```bash
cd pymwemu/
python setup_test_env.py  # Solo primera vez
python run_tests.py       # Ejecutar tests
```

### Método 3: Manual
```bash
cd pymwemu/
source .env/bin/activate
python -m unittest discover -v tests/
```

## Tests Disponibles

### Funcionalidad Básica (`test_basic_functionality.py`)
- Inicialización de emuladores 32/64-bit
- Operaciones de registros
- Operaciones de memoria
- Condiciones de límite con modo banzai
- Operaciones de strings
- Operaciones de stack

### Motor de Ejecución (`test_execution_engine.py`)
- Ejecución de instrucciones básicas
- Operaciones aritméticas
- Instrucciones de acceso a memoria
- Funcionalidad de breakpoints
- Conteo de instrucciones

### Carga de Binarios (`test_binary_loading.py`)
- Ejecución de shellcode 32/64-bit
- Patrones complejos de shellcode
- Emulación básica de APIs
- Operaciones de cifrado/descifrado

### Características Avanzadas (`test_advanced_features.py`)
- Operaciones FPU (si disponible)
- Serialización de estado
- Protección de memoria
- Trazado de instrucciones
- Escenarios de rendimiento

## Integración con CI/CD

Para integrar ambos tipos de tests en CI:

```yaml
# Ejemplo para GitHub Actions
- name: Run Rust tests
  run: cargo test --release

- name: Setup Python test environment
  run: |
    cd pymwemu
    python -m venv .env
    source .env/bin/activate
    pip install maturin
    maturin develop --release

- name: Run Python tests
  run: |
    cd pymwemu
    source .env/bin/activate
    python -m unittest discover -v tests/
```

## Comandos Útiles

```bash
# Tests específicos
make test-basic                    # Solo funcionalidad básica
make test-execution               # Solo motor de ejecución
make test-specific TEST=test_name # Test específico

# Reconstruir y probar
make rebuild-test                 # Reconstruir extensión y probar

# Limpiar
make clean                        # Limpiar artifacts
make clean-env                   # Limpiar entorno virtual
make clean-all                   # Limpiar todo
```

## Notas Importantes

1. **Los tests de Python NO se ejecutan con `cargo test`** - son independientes
2. **Requieren entorno virtual** - maturin instala el módulo en el venv activo
3. **Algunos tests pueden fallar** si ciertas APIs no están disponibles en Python
4. **Tests con banzai mode** - habilitado automáticamente para manejo seguro de errores
5. **Tests independientes** - pueden ejecutarse en cualquier orden

## Troubleshooting

### Error: "No module named 'pymwemu'"
```bash
# Asegúrate de estar en el entorno virtual correcto
source .env/bin/activate
maturin develop --release
```

### Error: "maturin not found"
```bash
pip install maturin
```

### Tests fallan por APIs no disponibles
Los tests automáticamente saltan funcionalidades no disponibles en los bindings de Python.

## Resumen

- **Rust tests**: `cargo test --release` (desde raíz)
- **Python tests**: Requieren setup especial (desde pymwemu/)
- **Ambos son independientes** y complementarios
- **Usa el método que prefieras** para configurar el entorno