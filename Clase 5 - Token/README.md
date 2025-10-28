# 💠 Resumen: Clases 5 y 6 - Token BDB en Stellar

## 📋 Índice
- [Clase 5: Construcción del Token BDB](#-clase-5-construcción-del-token-bdb)
- [Clase 6: Frontend e Integración con Wallet](#-clase-6-frontend-e-integración-con-wallet)
- [Autor](#-autor)

---

## 🔷 Clase 5: Construcción del Token BDB

### 🎯 Objetivo Principal
Construir un **smart contract tipo token (BDB)** completo desde cero utilizando **Rust** y el **SDK de Soroban**, siguiendo el estándar **CAP-46** de Stellar para tokens fungibles.

### 📚 Conceptos Clave Aprendidos

#### ¿Qué es un Token?
Un token es una representación digital de valor que vive en la blockchain. Es dinero completamente programable donde puedes definir reglas sobre cómo se crean, transfieren, gastan o destruyen.

**Ejemplos del mundo real:**
- Stablecoins (USDC, USDT)
- Créditos de videojuegos (V-Bucks en Fortnite)
- Millas aéreas (programas de lealtad)
- Créditos de carbono

#### CAP-46: El Estándar de Stellar
CAP-46 es el estándar oficial que define cómo deben comportarse los tokens en Stellar, similar a ERC-20 en Ethereum pero optimizado para Stellar.

**Ventajas vs Ethereum:**
- ⚡ **Velocidad**: 5 segundos vs 15 segundos
- 💸 **Costos**: ~$0.00001 vs $5-$50 por transacción
- 🔒 **Finalidad**: Instantánea vs múltiples confirmaciones
- 🦀 **Lenguaje**: Rust vs Solidity

### 🏗️ Estructura del Proyecto
```
token_bdb/
├── Cargo.toml          # Dependencias y configuración
└── src/
    ├── lib.rs          # Contrato principal con TokenTrait
    ├── storage.rs      # Tipos de datos (DataKey, estructuras)
    ├── errors.rs       # Enum TokenError
    └── test.rs         # Suite de tests unitarios
```

### ⚙️ Funciones Implementadas

El contrato incluye todas las funciones del estándar CAP-46:

**Configuración:**
- `initialize()` - Configura nombre, símbolo, decimales y admin (una sola vez)

**Operaciones de Tokens:**
- `mint()` - Crea nuevos tokens (solo admin)
- `burn()` - Destruye tokens existentes
- `transfer()` - Transfiere tokens entre cuentas

**Sistema de Permisos:**
- `approve()` - Permite que terceros muevan tokens en tu nombre
- `transfer_from()` - Transferencia autorizada por terceros

**Consultas:**
- `balance()` - Consulta el balance de una cuenta
- `allowance()` - Verifica permisos delegados
- `name()`, `symbol()`, `decimals()` - Metadatos del token
- `total_supply()` - Supply total en circulación

### 🔒 Principios de Seguridad

La clase enfatizó 4 pilares fundamentales:

1. **Autorización Estricta**: Usar `require_auth()` para verificar permisos
2. **Protección contra Overflow**: Usar `checked_add()` y `checked_sub()`
3. **Gestión de TTL**: Extender el Time To Live de los datos críticos
4. **Eventos Ricos**: Emitir eventos detallados para auditoría

**Regla de oro:** Nunca usar `panic!` en producción. Siempre retornar `Result<T, Error>`.

### 🧪 Testing Exhaustivo

Se implementaron tests unitarios cubriendo:
- ✅ Inicialización correcta y prevención de doble inicialización
- ✅ Mint con validaciones de autorización y overflow
- ✅ Transferencias válidas y casos edge
- ✅ Sistema approve/transfer_from
- ✅ Burn de tokens
- ✅ Consultas de metadata
- ✅ Manejo de errores sin inicializar

**Comandos ejecutados:**
```bash
cargo test test_initialize -- --nocapture
cargo test test_mint_and_balance -- --nocapture
cargo test test_transfer -- --nocapture
```

### 📦 Compilación

El contrato fue compilado exitosamente:
```bash
stellar contract build
```

Generando el archivo WASM optimizado:
```
target/wasm32-none/release/token_bdb.wasm
```

---

## 🔶 Clase 6: Frontend e Integración con Wallet

### 🎯 Objetivo Principal
Llevar el token BDB del terminal al mundo real, creando una **interfaz web funcional** con integración de **Freighter Wallet** para que cualquier persona pueda interactuar con el token.

### 🌐 Conceptos Clave Aprendidos

#### Del Localhost a la Blockchain Pública

**Diferencia fundamental:**
- **Localhost**: Documento privado, solo en tu computadora
- **Testnet**: Documento público, accesible desde cualquier parte del mundo

**Características de Stellar Testnet:**
- Blockchain real con la misma tecnología que Mainnet
- Transacciones visibles públicamente
- Contratos compartibles con link público
- 100% gratis con XLM de prueba

#### Glosario Técnico

- **WASM**: Código Rust compilado en formato que blockchain puede ejecutar
- **Contract ID**: Identificador único del contrato (empieza con "C")
- **Network Passphrase**: Define si trabajas en Testnet o Mainnet
- **Public Key**: Dirección pública (empieza con "G")
- **Secret Key**: Contraseña privada (empieza con "S") - nunca compartir

### 🚀 Proceso de Deploy a Testnet (Teoría)

Aunque no completaste el deploy, la clase cubrió el proceso completo:

#### 1. Compilación y Optimización
```bash
stellar contract build
wasm-opt -Oz -o optimized.wasm target/wasm32v1-none/release/token_bdb.wasm
```

#### 2. Configuración de Cuenta
```bash
stellar keys generate testnet --network testnet --fund
```

#### 3. Deploy del Contrato
```bash
stellar contract deploy \
    --wasm optimized.wasm \
    --source testnet \
    --network testnet
```

#### 4. Inicialización
```bash
stellar contract invoke \
    --id $CONTRACT_ID \
    --source testnet \
    --network testnet \
    -- initialize \
    --admin testnet \
    --name "BuilderToken" \
    --symbol "BDB" \
    --decimals 7
```

#### 5. Verificación
Contrato visible en: `stellar.expert/explorer/testnet`

### 💻 Scaffold Stellar

**¿Qué es Scaffold Stellar?**
Un boilerplate que proporciona:
- React + TypeScript preconfigurado
- Conexión a Stellar lista para usar
- Componentes de ejemplo incluidos
- Hot reload automático
- Clientes TypeScript generados automáticamente

**Comparativa:**

| Opción A: Desde Cero | Opción B: Scaffold Stellar |
|---------------------|---------------------------|
| Configurar webpack manualmente | React + TS preconfigurado |
| Instalar 40+ librerías | Dependencias listas |
| Resolver CORS | Conexión a Stellar ready |
| Configurar TypeScript | Clientes auto-generados |
| 🕒 3 meses de frustración | ⚡ 10 minutos + personalización |

### 🔧 Setup de Scaffold Stellar

#### 1. Inicialización
```bash
stellar scaffold init mi-token-bdb
cd mi-token-bdb
```

#### 2. Integrar Contrato
```bash
cp -r ../token_bdb ./contracts/buen_dia_token
```

#### 3. Configurar Entorno (.env)
```env
VITE_BDB_CONTRACT_ID=CXXXXXX...
VITE_RPC_URL=https://soroban-testnet.stellar.org
VITE_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
```

#### 4. Instalar Dependencias
```bash
npm install
npm install @stellar/freighter-api @stellar/stellar-sdk
```

#### 5. Generar Cliente TypeScript
```bash
npm run build:contracts
```

#### 6. Ejecutar
```bash
npm run dev
```

Aplicación corriendo en `http://localhost:5173/`

### 🔐 Freighter Wallet

**¿Qué es Freighter?**
Una extensión de navegador (como MetaMask para Ethereum) que permite:
- Guardar claves privadas de forma segura
- Firmar transacciones cuando las autorizas
- Conectarte con dApps de Stellar
- Funcionar en Testnet y Mainnet

**¿Por qué es necesaria?**
Tu frontend puede **leer** datos públicos sin problema, pero para **escribir** (transferencias, mint, etc.) necesitas:
- ✅ Firmar transacciones criptográficamente
- ✅ Pagar fees (~$0.00001 XLM)

**Seguridad:**
- Freighter NUNCA comparte tu secret key
- Solo firma con tu autorización explícita
- Tú mantienes control total

### 🔗 Integración con Freighter

#### Código de Conexión
```typescript
import { isConnected, getPublicKey } from '@stellar/freighter-api'

const connectWallet = async () => {
    if (await isConnected()) {
        const pk = await getPublicKey()
        setPublicKey(pk)
        setConnected(true)
    } else {
        alert('Instala Freighter desde https://freighter.app')
    }
}
```

#### Flujo de Conexión
1. Usuario click en "Conectar Wallet"
2. Freighter muestra popup
3. Usuario aprueba compartir public key
4. Freighter devuelve la key de forma segura
5. App puede firmar transacciones

### 📊 Funcionalidad: Consultar Balance
```typescript
const getBalance = async () => {
    const client = new BuenDiaTokenClient({
        contractId: import.meta.env.VITE_BDB_CONTRACT_ID,
        networkPassphrase: 'Test SDF Network ; September 2015',
        rpcUrl: 'https://soroban-testnet.stellar.org'
    })
    
    const result = await client.balance({ id: publicKey })
    setBalance(result.toString())
}
```

**Flujo:**
1. Conectar wallet
2. Click en "Ver balance"
3. Frontend consulta blockchain
4. Resultado en pantalla en tiempo real

### 🏗️ Arquitectura Completa
```
lib.rs (Rust) 
    ↓
Compilación WASM
    ↓
Deploy a Testnet
    ↓
Frontend React ←→ Freighter Wallet
    ↓
Usuario (Tú en control)
```

### 🎨 Desafío Tiburona

**Nivel 1 - Básico:**
- Personalizar interfaz (colores, título, mensaje)
- Agregar marca personal
- 📸 Entregable: Screenshot

**Nivel 2 - Intermedio:**
- Formulario de transferencia
- Validación de direcciones
- Mensajes de éxito/error
- Actualización de balance
- 🔗 Entregable: GitHub + Deploy + Video

### 🐛 Troubleshooting Común

**Error: "WASM file not found"**
- Ejecutar `stellar contract build`
- Verificar ruta con `ls target/wasm32v1-none/release/`

**Error: "Invalid WASM file"**
- Usar target `wasm32v1-none` (NO `wasm32-unknown-unknown`)
- Verificar con `rustup show`

**Error: "Cannot find module"**
- Ejecutar `npm run build:contracts`
- Verificar que exista `contracts/buen_dia_token/src/lib.rs`

**Error: "Network error"**
- Verificar `.env.local` con URL correcta
- Confirmar Contract ID válido

---

## 📚 Resumen de Logros

### Clase 5: Backend Blockchain
- ✅ Smart contract completo en Rust
- ✅ Estándar CAP-46 implementado
- ✅ Sistema de seguridad robusto
- ✅ Tests unitarios exhaustivos
- ✅ Compilación WASM exitosa

### Clase 6: Frontend Full-Stack
- ✅ Conceptos de deploy a Testnet (teoría)
- ✅ Scaffold Stellar configurado
- ✅ Integración con Freighter Wallet
- ✅ Generación de clientes TypeScript
- ✅ Consulta de balance funcional
- ✅ Arquitectura completa de dApp

### Transformación Completa
**De Builder Backend a Full-Stack Blockchain Builder**

---

## 🧠 Autor

**Angie Mariela Alpízar Porras**

---

## 📝 Notas Importantes

**Estado del Deploy:**
Aunque el contrato fue compilado exitosamente y todos los tests pasaron, el deploy a Testnet no fue completado debido a problemas técnicos con el Stellar CLI y el plugin Scaffold.

**Conocimiento Adquirido:**
A pesar de no completar el deploy, se adquirió conocimiento completo sobre:
- El proceso teórico de deployment
- La arquitectura de dApps en Stellar
- La integración de wallets
- El flujo completo de desarrollo blockchain

**Próximos Pasos:**
- Resolver issues técnicos con Stellar CLI
- Completar deploy a Testnet
- Implementar funcionalidad de transferencias
- Personalizar la interfaz

---

*Resumen de Clases 5 y 6 - Código Futura by Buen Día Builders*
