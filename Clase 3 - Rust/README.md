#  Clase 3 - Introduccion a Rust

> **Idea clave:** Una **base de datos** es una colección organizada de datos accesibles electrónicamente que permite **gestionar grandes volúmenes de información** de forma eficiente. 🧠⚡

---

## 🧩 Componentes Principales

### 🛠️ Sistema de Gestión de Bases de Datos (SGBD)
Funciones esenciales:
- 🧱 Definición de estructuras de datos  
- ✍️ Manipulación de información  
- 🔐 Control de acceso y seguridad  
- 💾 Respaldo y recuperación  
- 👥 Control de concurrencia

### 🧭 Modelos de Datos

**🧮 Modelo Relacional**
- Tablas (relaciones) con **filas** (tuplas) y **columnas** (atributos)
- Llaves **primarias** y **foráneas**
- Ejemplos: `MySQL`, `PostgreSQL`, `Oracle`

**🧱 No Relacional (NoSQL)**
- Estructura flexible
- Tipos: **documentos**, **clave-valor**, **grafos**, **columnas**
- Ejemplos: `MongoDB`, `Redis`, `Cassandra`

---

## 🧾 Lenguaje SQL

### 🔤 Tipos de comandos

**DDL — _Data Definition Language_**
- `CREATE` 🆕 · `ALTER` ✏️ · `DROP` 🗑️ · `TRUNCATE` 🧼

**DML — _Data Manipulation Language_**
- `SELECT` 🔎 · `INSERT` ➕ · `UPDATE` ♻️ · `DELETE` ➖

**DCL — _Data Control Language_**
- `GRANT` 🎟️ · `REVOKE` 🚫

> 🧪 **Mini-demo**
> ```sql
> CREATE TABLE productos (
>   id SERIAL PRIMARY KEY,
>   nombre VARCHAR(100) NOT NULL,
>   precio DECIMAL(10,2) NOT NULL
> );
> INSERT INTO productos (nombre, precio) VALUES ('Café', 2.50);
> SELECT * FROM productos;
> ```

---

## 🔑 Conceptos Clave

### 🧼 Normalización
Para **reducir redundancia** y **mejorar integridad**:
- ✅ 1FN · 2FN · 3FN · **FNBC**
- 👍 Facilita mantenimiento y consultas

### 🔁 Transacciones (ACID)
- **A**tomicidad: _todo o nada_ 🧨
- **C**onsistencia: respeta reglas 📐
- **I**solamiento: independientes 🧊
- **D**urabilidad: cambios permanentes 🧱

### ⚡ Índices
- 🚀 Aceleran `SELECT`
- 🐢 Pueden ralentizar `INSERT/UPDATE/DELETE`
- Tipos: **primarios**, **secundarios**, **únicos**, **compuestos**

### 🗝️ Claves
- **Clave Primaria (PK):** única, no `NULL`, identifica registros 🎯  
- **Clave Foránea (FK):** conecta tablas y cuida la **integridad referencial** 🔗

---

## 🔗 Tipos de Relaciones
- **1:1** — un registro ↔️ un registro  
- **1:N** — uno ↔️ muchos  
- **N:M** — muchos ↔️ muchos (usando tabla puente) 🌉

> 📝 **Ejemplo FK**
> ```sql
> CREATE TABLE categorias (
>   id SERIAL PRIMARY KEY,
>   nombre TEXT NOT NULL
> );
> CREATE TABLE productos (
>   id SERIAL PRIMARY KEY,
>   nombre TEXT NOT NULL,
>   categoria_id INT REFERENCES categorias(id)
> );
> ```

---

## 🏗️ Diseño de Bases de Datos — *Pasos*
1. 🔍 **Análisis de requisitos**  
2. 🧭 **Modelo conceptual** (Diagrama ER)  
3. 🗂️ **Modelo lógico** (tablas, claves)  
4. 💿 **Modelo físico** (implementación en SGBD)  
5. ⚙️ **Optimización** (índices, consultas, recursos)

---

## 🌟 Buenas Prácticas
- 🏷️ Nomenclatura clara y consistente  
- 🧲 Restricciones de integridad (`NOT NULL`, `UNIQUE`, `CHECK`)  
- 📚 Documentar estructura y relaciones  
- 🛟 Respaldos periódicos  
- 🔎 Optimizar consultas frecuentes  
- 🧼 Aplicar normalización con criterio  
- 🔐 Gestión de accesos y permisos

---

## 🛡️ Consideraciones de Seguridad
- 🔑 Autenticación robusta (MFA si es posible)
- 🧷 Cifrado de datos sensibles (en reposo y en tránsito)
- 🧾 Auditorías y logs
- 🪪 Principio de **menor privilegio**
- 🧱 Prevención de **inyección SQL** (queries preparados)
- 🔒 Respaldos **cifrados**

---

### 🎯 Resumen Visual Rápido
- **SQL** = define + manipula + controla  
- **PK/FK** = identidad 🔑 + relación 🔗  
- **ACID** = confiabilidad de transacciones 🧱  
- **Índices** = consultas rápidas ⚡  
- **Normalización** = datos limpios 🧼

> 💡 **Tip final:** empieza con un diseño claro (ER), crea claves e índices donde aporten valor, y mide el rendimiento de tus consultas con `EXPLAIN/EXPLAIN ANALYZE`. 📈
