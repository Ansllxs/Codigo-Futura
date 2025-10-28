# 💠 Proyecto: Token BDB — Clase 5

## 🧩 Descripción General

Este proyecto forma parte del módulo **"Clase 5 - Token"** del curso de desarrollo con **Soroban y Stellar**.

El objetivo principal fue **construir un smart contract tipo token (BDB)** utilizando **Rust** y el SDK de **Soroban**, para comprender cómo funcionan los contratos inteligentes básicos dentro del ecosistema Stellar.

El contrato incluye las funcionalidades esenciales de un token:

- Inicialización con nombre, símbolo y decimales.
- Registro de balances.
- Función de `mint` (emisión).
- Función de `transfer` (transferencia de tokens entre usuarios).
- Tests unitarios para verificar el correcto funcionamiento.

---

## ⚙️ Implementaciones Realizadas

### ✅ Configuración del proyecto desde cero

Se creó un proyecto Rust con:
```bash
cargo init --lib
```

Se configuró el archivo `Cargo.toml` con las dependencias correctas:
```toml
[dependencies]
soroban-sdk = "23.0.2"

[dev-dependencies]
soroban-sdk = { version = "23.0.2", features = ["testutils"] }
```

### ✅ Estructura del proyecto completada
```
token_bdb/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── storage.rs
    ├── errors.rs
    └── test.rs
```

### ✅ Compilación exitosa

Se compiló correctamente el contrato y se generó el archivo `.wasm`:
```bash
target/wasm32-none/release/token_bdb.wasm
```

### ✅ Pruebas ejecutadas

Se corrieron los tres tests principales con éxito:
```bash
cargo test test_initialize -- --nocapture
cargo test test_mint_and_balance -- --nocapture
cargo test test_transfer -- --nocapture
```

Todos los tests básicos pasaron satisfactoriamente ✔️

Esto confirma que el token funciona como se esperaba.

---

## 🚧 Parte No Completada

### ❌ Integración con Scaffold Stellar (CLI)

No se logró completar la parte opcional de deploy y conexión con la interfaz Scaffold Stellar, ya que no fue posible instalar correctamente el Stellar CLI.

Los intentos con los comandos:
```bash
stellar scaffold
stellar install scaffold
```

arrojaron el error:
```
error: unrecognized subcommand 'scaffold'
```

Por esta razón, la parte de despliegue en testnet y la integración con frontend no pudo realizarse.

---

## 📚 Conclusión y Aprendizaje

Este ejercicio permitió comprender:

- Cómo se estructura un contrato inteligente en Soroban.
- Cómo definir las funciones y errores dentro de un token.
- Cómo ejecutar tests unitarios para verificar su comportamiento.
- El flujo de compilación hasta la generación del archivo `.wasm`.

Aunque la parte de Scaffold Stellar no pudo realizarse, se completó el núcleo del proyecto con éxito, consolidando los conceptos fundamentales de desarrollo con Soroban.

---

## 📦 Estado del Proyecto

| Tarea | Estado |
|-------|--------|
| Configuración del proyecto | ✅ Completa |
| Implementación del contrato | ✅ Completa |
| Ejecución de tests | ✅ 3/3 aprobados |
| Validaciones opcionales | ✅ Implementadas parcialmente |
| Deploy en testnet (CLI) | ❌ No realizado (error con CLI Scaffold) |

---

## 🧠 Autor

**Angie Mariela Alpízar Porras**
