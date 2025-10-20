#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String  // ⭐ String agregado
};


// =====================
// Errores del contrato
// =====================
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}


// =====================
// Claves de almacenamiento
// =====================
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}



// =====================
// Declaración del contrato
// =====================
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // Aquí irán las funciones
}