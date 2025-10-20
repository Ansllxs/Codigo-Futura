# 🦈 Hello Tiburona Profesional 

## Clase 4 – Rust Avanzado con Soroban/Stellar

Contrato "Hello Tiburona" con manejo de errores, storage organizado, TTL, control de acceso y tests.  
Incluye retos opcionales: contador por usuaria, transferencia de admin y límite configurable.

---

## 📌 De qué trató la tarea

Construir un contrato profesional paso a paso usando Soroban (Stellar) SDK:

1. **Inicializar el contrato** con admin y contador global.
2. **Implementar `hello()`** con validaciones y persistencia del último saludo por Address.
3. **Exponer consultas** (`get_contador`, `get_ultimo_saludo`).
4. **Agregar una función administrativa** (`reset_contador`) con control de acceso.
5. **Escribir tests** con testutils.
6. **(Opcional)** Ampliar con estadísticas por usuaria, transferencia de admin y límite configurable.

---

## ⚠️ Update importante (19/10/2025)

`Symbol` no tiene `.len()` ni `.to_string()` en soroban-sdk.

**Solución:** usar `String` para el parámetro `nombre` (permite validar longitud).

Ajustes realizados en imports, firma de `hello()` y tests (`String::from_str(&env, "...")`).

**Créditos:** Tiburona Karen 🦈⚡

---

## 🧰 Entorno y herramientas

### Rust + Cargo

Instalado vía `rustup`.

**Target WASM recomendado por la CLI actual:**

```bash
rustup target add wasm32v1-none
```

> 📝 Algunas guías usan `wasm32-unknown-unknown`; con la CLI moderna se emplea `wasm32v1-none`.

### Windows

Instalar **MSVC/link.exe** (Visual Studio Installer → Desarrollo de escritorio con C++).

**Alternativa:** `.cargo/config.toml` con `linker = "rust-lld.exe"`.

### Stellar CLI

Renombrada desde Soroban CLI:

```bash
cargo install --locked stellar-cli
stellar --version
```

---

## 📁 Estructura del proyecto

```
.
├─ Cargo.toml                # Workspace
├─ README.md                 # Este documento
└─ contracts/
   └─ hello-world/           # (era helloe-tiburona, pero solo lo pude  hacer  con ese otro nombre)
      ├─ Cargo.toml
      └─ src/
         └─ lib.rs
```

Si usas `hello-tiburona`, en `Cargo.toml` raíz:

```toml
[workspace]
members = ["contracts/hello-tiburona"]
```

---

## 🏗️ Qué implementé

### Errores personalizados

```rust
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}
```

### Claves de almacenamiento (DataKey)

```rust
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,                     // instance
    ContadorSaludos,           // instance
    UltimoSaludo(Address),     // persistent
    // Retos opcionales:
    ContadorPorUsuario(Address),
    LimiteCaracteres,
}
```

### API del contrato

#### `initialize(env, admin) -> Result<(), Error>`

- Evita reinicialización (`has()` en instance storage)
- Guarda `Admin` y `ContadorSaludos=0` en instance
- Extiende TTL

#### `hello(env, usuario, nombre: String) -> Result<Symbol, Error>`

- Valida: `nombre.len() > 0` y `len <= límite` (por defecto 32; configurable)
- Incrementa contador global (instance)
- Persiste último saludo por Address (persistent)
- Extiende TTL (persistent + instance)
- Retorna `Symbol::new(&env, "Hola")`

#### Consultas

- `get_contador(env) -> u32`
- `get_ultimo_saludo(env, usuario) -> Option<String>`

#### Administración

- `reset_contador(env, caller) -> Result<(), Error>` (solo admin)

---

## 🧪 Pruebas

**Framework:** `soroban-sdk` con `features = ["testutils"]`.

### Casos cubiertos:

- ✅ `initialize` (contador inicia en 0)
- ✅ Bloqueo de reinicialización
- ✅ `hello` exitoso (contador + persistencia)
- ✅ Validación de nombre vacío
- ✅ Reset solo por admin

### Ejecutar:

```bash
cargo test
```

---

## 🚀 Build & Optimize

### Compilar

```bash
# Compilar apuntando al contrato correcto (ajusta la ruta si es hello-tiburona)
stellar contract build --manifest-path "contracts/hello-world/Cargo.toml"

# Ver artefactos
# target/wasm32v1-none/release/hello_world.wasm  (o hello_tiburona.wasm)
```

### Optimizar

```bash
stellar contract optimize --wasm "target/wasm32v1-none/release/hello_world.wasm" \
  --output "target/wasm32v1-none/release/hello_world.opt.wasm"
```

---

## 🧩 Retos opcionales (implementados o listos)

### 1) Contador por usuaria

**DataKey:** `ContadorPorUsuario(Address)`

En `hello()`:
- Leer `u32` (default 0), incrementar y guardar en persistent
- `extend_ttl` de esa key

**Getter:**

```rust
pub fn get_contador_usuario(env: Env, usuario: Address) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::ContadorPorUsuario(usuario))
        .unwrap_or(0)
}
```

### 2) Transferencia de admin

```rust
pub fn transfer_admin(env: Env, caller: Address, nuevo_admin: Address) -> Result<(), Error> {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NoInicializado)?;
    if caller != admin { return Err(Error::NoAutorizado); }
    env.storage().instance().set(&DataKey::Admin, &nuevo_admin);
    Ok(())
}
```

### 3) Límite de longitud configurable

**DataKey:** `LimiteCaracteres`

**Setter (solo admin):**

```rust
pub fn set_limite(env: Env, caller: Address, limite: u32) -> Result<(), Error> {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NoInicializado)?;
    if caller != admin { return Err(Error::NoAutorizado); }
    env.storage().instance().set(&DataKey::LimiteCaracteres, &limite);
    Ok(())
}
```

En `hello()`: usar `limite` con `unwrap_or(32)` y validar `nombre.len()`.

---

## 🎓 Lo que aprendí

- ✅ Errores robustos con `#[contracterror]` y `Result<_, Error>`.
- ✅ Diferenciar **Instance** vs **Persistent** storage y cuándo usar cada uno.
- ✅ **Validar antes de mutar:** `String` permite validar longitud (vs `Symbol`).
- ✅ **TTL:** mantener vivas las claves que se usan frecuentemente.
- ✅ Control de acceso con `Address` (admin) y checks claros.
- ✅ Pruebas realistas con testutils (`Env::default()`, `Address::generate`, client generado).
- ✅ **Stellar CLI** (renombrada desde Soroban CLI) y el target `wasm32v1-none` moderno.
- ✅ Particularidades de Windows: MSVC/linker y rutas con espacios.

---

## ✅ Checklist final

### Implementación

- ✅ `Error` (4 variantes)
- ✅ `DataKey`: `Admin`, `ContadorSaludos`, `UltimoSaludo(Address)`
- ✅ `initialize()` con verificación y TTL
- ✅ `hello()` con `String`, validaciones, storage y TTL
- ✅ `get_contador(
