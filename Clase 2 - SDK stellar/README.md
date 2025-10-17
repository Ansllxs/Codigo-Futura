# Clase 2 — SDK stellar 🦈

> En esta sesión trabajaste con el SDK de JavaScript y la CLI de Stellar para **crear cuentas**, **enviar pagos** y **consultar datos on-chain**. Aquí queda todo listo para pegar como `README.md`.

---

## 🧾 Resumen
Se exploraron **dos vías de desarrollo** sobre Stellar:
1. **JavaScript + `@stellar/stellar-sdk`** para generar llaves, construir/firmar transacciones y consultar Horizon.
2. **Stellar CLI** para operar desde terminal sin escribir código JavaScript.

Además, se construyeron tres scripts prácticos: **creación masiva de cuentas**, **pagos a múltiples destinatarios** y **monitor de balances**.

---

## ✅ Requisitos
- Node.js 18+ y npm
- Acceso a **Testnet** (Friendbot para fondeo)
- Editor de texto y terminal

Instala el SDK:
```bash
npm install @stellar/stellar-sdk
```

---

## 🧰 Herramientas clave

- **SDK de JavaScript (@stellar/stellar-sdk):**
Generación de Keypair, construcción de transacciones con TransactionBuilder, firma y envío vía Horizon.

- **Stellar CLI:**
Operaciones comunes (cuentas, pagos, consultas) desde línea de comandos.
Docs: https://developers.stellar.org/docs/tools/cli/install-cli

---
## Como crear carpetas:
```bash
mkdir javascript-sdk stellar-cli hello-contract
```
---
## 🧪 Prácticas realizadas
1) Generación masiva de cuentas — ```crear-cuenta.js ```

**Objetivo:** 
Crear 5 cuentas nuevas en testnet, fondearlas y registrar sus datos.

Requisitos:

- Bucle for para producir 5 Keypairs.

- Fondear cada cuenta con Friendbot.

- Mostrar public key, secret key y balance inicial.

- Guardar la info en un array para reutilizar.

Esqueleto:
``` for (let i = 1; i <= 5; i++) {
  console.log(`Creando cuenta ${i}...`);
  // 1) Generar keypair
  // 2) Fondear con Friendbot
  // 3) Consultar balance inicial
  // 4) Guardar en un array
}
```


**Resultado esperado:** 
5 cuentas activas en testnet con 10,000 XLM de prueba, datos listos para usarse en otros scripts.

2) Pagos automáticos — ```enviar-pago.js```

**Objetivo:** 
 Enviar 2 XLM a 3 destinatarios en la misma ejecución.

**Requisitos:**

- Lista de destinatarios con memo único por pago.

- Verificar el éxito de cada transacción antes de continuar.

- Loggear el hash de la transacción (para revisarla en el explorador).

Estructura de datos:
```
const destinatarios = [
  { publicKey: "G...1", memo: "Pago-001" },
  { publicKey: "G...2", memo: "Pago-002" },
  { publicKey: "G...3", memo: "Pago-003" },
];
```

**Resultado esperado:** 
3 pagos confirmados con sus hashes registrados y balances actualizados.

3) Monitor de balances — ```consultar-balance.js```

**Objetivo:** Consultar datos clave de varias cuentas y mostrarlos en formato legible.

**Requisitos:**

- Entrada: array de public keys.

- Para cada cuenta mostrar:

 - Balance de XLM

 - Trustlines activas

 - Sequence number

- Salida formateada tipo panel.

***Ejemplo de salida:**
```
=== MONITOR DE CUENTAS ===
Cuenta: GBXXX...123
  Balance: 100.50 XLM
  Trustlines: 2
  Sequence: 123456789
```

**Resultado esperado:**
Panel de estado para múltiples cuentas que sirva de diagnóstico rápido.

---

## ▶️ Ejecución de scripts

Ejecutar cualquier archivo JS:

```
node nombre-del-archivo.js
```

---
## 📚 Recursos de referencia

- SDK JS: https://stellar.github.io/js-stellar-sdk/

- Stellar CLI: https://developers.stellar.org/docs/tools/cli/stellar-cli

- Soroban (smart contracts): https://developers.stellar.org/docs/build/smart-contracts

- Horizon API: https://developers.stellar.org/api/horizon

- Laboratory: https://laboratory.stellar.org

- Stellar Expert (testnet): https://stellar.expert/explorer/testnet

- Friendbot (testnet): https://friendbot.stellar.org


---


## 🎯 Resultados de aprendizaje

JavaScript + Stellar

- Construcción de transacciones con múltiples operaciones.

- Programación asíncrona y manejo de errores.

- Consumo de streams/consultas en tiempo real.

Stellar CLI

- Automatización desde terminal (bash).

- Gestión de identidades y cuentas.

- Primeros pasos con despliegue de contratos.

Pensamiento crítico

- Investigación técnica con fuentes oficiales.

- Identificación de fallas y cuellos de botella.

- Propuesta de soluciones e iteración.










