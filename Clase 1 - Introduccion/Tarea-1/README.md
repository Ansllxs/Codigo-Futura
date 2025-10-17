# Clase 1 — Introduccion a Stellar 🦈

> Bienvenida a la red Stellar: pagos globales en segundos, costos bajísimos y herramientas para desarrolladoras.  
> En esta clase se creo la primera cuenta en testnet, la fondearás y enviarás tu primer pago.

---

## 🚀 ¿Qué es Stellar (en una frase)?
Una red blockchain **descentralizada** pensada para **mover valor de forma rápida y barata** entre personas y países.

### Por qué importa
- **Velocidad:** confirmaciones típicas en **3–5 s**.
- **Costo:** tarifas mínimas (≈ **0.00001 XLM** por transacción).
- **Alcance:** transferencias **transfronterizas** sin intermediarios.
- **Escala:** miles de operaciones por segundo.
- **Transparencia:** software **open-source**.

---

## 🧠 Lo esencial que debes saber

### 1) Lumens (XLM)
- Moneda nativa de la red.
- Se usa para tarifas y para “puentear” activos distintos.

### 2) Cuentas
Cada cuenta tiene dos claves:
- Clave pública (empieza con G): GBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
- Clave secreta (empieza con S): SBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
  - La **pública** es tu “dirección”.
  - La **secreta** da control total: **nunca** se comparte.

### 3) Testnet (red de pruebas)
- Entorno seguro para practicar.
- XLM de prueba gratis (vía **Friendbot**).
- **Cero** riesgo económico.

---

## 🏗️ Componentes de la red

**Horizon**  
API REST que tu app usa para consultar cuentas, enviar transacciones y leer el historial.

**Stellar Core**  
Nodos que corren el protocolo de consenso y mantienen el **ledger** sincronizado.

**Friendbot**  
Servicio que deposita XLM de **testnet** en cuentas nuevas para que puedas desarrollar.

---

## 🛠️ Herramientas que se utilizaron

- **Stellar Laboratory** — construir/firmar transacciones, generar llaves, probar endpoints.  
  <https://laboratory.stellar.org>
- **Stellar Expert (testnet)** — explorador para ver cuentas, transacciones y estadísticas.  
  <https://stellar.expert/explorer/testnet>
- **Friendbot** — fondeo en testnet.  
  <https://friendbot.stellar.org>

---

## ✍️ Primeros pasos prácticos

### A) Crear tu par de llaves
1. Entra a **Stellar Laboratory**.
2. Cambia a **Testnet** (esquina superior).
3. Ve a **Account → Generate Keypair**.
4. Guarda **pública** y **secreta** en un lugar seguro.

### B) Fondear tu cuenta (testnet)
Tienes 3 opciones:
- Botón de Friendbot en el Laboratory.  
- Petición HTTP a `<https://friendbot.stellar.org?addr=TU_CLAVE_PUBLICA>`.  
- Alguna CLI/SDK.

> Resultado esperado: **10,000 XLM** de prueba.

### C) Consultar tu balance
1. En Laboratory: **Endpoints → Accounts**.  
2. Pega tu **clave pública**.  
3. Revisa **balances**, **sequence number**, etc.

---

## 🧾 Operaciones frecuentes en Stellar

- **Create Account** — crear una cuenta nueva con XLM inicial.
- **Payment** — enviar activos (XLM u otros).
- **Path Payment** — pagos con conversión automática de activo A → B.
- **Manage Offer** — órdenes en el DEX integrado.
- **Set Options** — configurar tu cuenta (flags, firmantes, etc.).
- **Change Trust** — añadir confianza a un activo emitido.
- **Account Merge** — fusionar y cerrar cuentas.

---

## 🧬 Anatomía de una transacción

- **Source Account:** quién origina.
- **Sequence Number:** contador incremental exacto por cuenta.
- **Operations:** hasta **100** por transacción.
- **Fee:** base de **100 stroops** (0.00001 XLM).
- **Memo (opcional):** nota corta.
- **Time Bounds:** ventana de validez.

### Conceptos clave
- **Sequence Number:** previene duplicados; debe ser el **siguiente** al actual.
- **Stroops:** mínima unidad de XLM. `1 XLM = 10,000,000 stroops`.
- **Memos:**  
  - `MEMO_TEXT` (hasta 28 bytes)  
  - `MEMO_ID` (numérico)  
  - `MEMO_HASH` (32 bytes)  
  - `MEMO_RETURN` (devoluciones)
  -

---

## 🔐 Buenas prácticas

**Seguridad**
- Nunca compartas tu **clave secreta**.
- Usa **testnet** para aprender y probar.
- Verifica direcciones y memos antes de enviar.
- Respalda llaves de forma segura (gestor de contraseñas).

**Desarrollo**
- Empieza siempre en **testnet**.
- Prueba cada operación antes de ir a producción.
- Maneja errores de Horizon.
- Documenta lo que haces y enlaza hashes/transacciones.

---

## 📚 Recursos de referencia

- Docs: <https://developers.stellar.org>  
- API: <https://developers.stellar.org/api>  
- JS SDK: <https://stellar.github.io/js-stellar-sdk/>  
- Comunidad: Discord (#soroban-dev), Stack Overflow (`stellar`)  
- Código: <https://github.com/stellar>

---

## 🏁 Logros de esta clase

- [x] Entendiste el propósito de Stellar.  
- [x] Creaste tu **cuenta en testnet**.  
- [x] La fondeaste con **Friendbot**.  
- [x] Consultaste **balance** y **sequence**.  
- [x] Enviastes tu **primer pago**.  
- [x] Navegaste el explorador **Stellar Expert**.  
- [x] Te familiarizaste con **Laboratory**.

---

## 📖 Glosario exprés

| Término     | Qué es                                                                 |
|-------------|-------------------------------------------------------------------------|
| Ledger      | Estado global de cuentas y balances en la blockchain.                  |
| Account     | Entidad que firma y envía operaciones.                                 |
| Asset       | Cualquier activo en Stellar (XLM o emitidos por terceros).             |
| Trustline   | Permiso para **recibir** un activo específico.                         |
| Operation   | Acción individual (ej. Payment, Change Trust).                         |
| Fee         | Costo de procesamiento por transacción.                                |
| Horizon     | API para interactuar con la red.                                       |
| Testnet     | Red de pruebas con XLM gratuitos.                                      |

---

## ⚠️ Notas importantes

- En Stellar **no hay gas variable** como en otras cadenas.  
- Las confirmaciones suelen ser **casi instantáneas**.  
- Puedes agrupar **múltiples operaciones** en una sola transacción.  
- En **mainnet**, cada cuenta requiere un **mínimo** de XLM reservado.


