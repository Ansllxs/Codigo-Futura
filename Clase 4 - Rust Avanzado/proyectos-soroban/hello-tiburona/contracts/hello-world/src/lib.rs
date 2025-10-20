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
    ContadorPorUsuario(Address),

}


// =====================
// Declaración del contrato
// =====================
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        // 3.2: Evitar reinicializar: más barato con has() que con get()
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }
    
        // 3.3: Guardar admin en Instance storage
        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
    
        // 3.4: Inicializar contador en 0
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
    
        // 3.5: Extender TTL del instance storage
        // (ttl_threshold = 100, extend_to = 100)
        env.storage()
            .instance()
            .extend_ttl(100, 100);
    
        Ok(())
    }
    
    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String, // ⭐ usamos String para poder validar .len()
    ) -> Result<Symbol, Error> {
        // 4.2: validación - no vacío
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }
        // 4.3: validación - límite de longitud
        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }
    
        // 4.4: incrementar contador (patrón: leer → modificar → guardar)
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env
            .storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));
    
        // 4.5: guardar último saludo del usuario (dato crítico → persistent)
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);


    pub fn get_contador_usuario(env: Env, usuario: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ContadorPorUsuario(usuario))
            .unwrap_or(0)
    }

        // Leer contador del usuario
        let key_usuario = DataKey::ContadorPorUsuario(usuario.clone());
        let contador_usuario: u32 = env
            .storage()
            .persistent()
            .get(&key_usuario)
            .unwrap_or(0);

        // Incrementar contador del usuario
        env.storage()
            .persistent()
            .set(&key_usuario, &(contador_usuario + 1));

        // Extender TTL también para el contador del usuario
        env.storage()
            .persistent()
            .extend_ttl(&key_usuario, 100, 100);

    
        // 4.6: extender TTL (primero persistent, luego instance)
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        env.storage().instance().extend_ttl(100, 100);
    
        // 4.7: retornar un símbolo simple
        Ok(Symbol::new(&env, "Hola"))
    }
    

    // 5.1: contador global (nunca falla)
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    // 5.2: último saludo por usuario (puede no existir)
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }

    // 6.1: resetear el contador (solo admin)
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        // 6.2: obtener admin desde instance storage
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

        // 6.3: verificar permisos
        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        // 6.4: resetear contador
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);

        Ok(())
    }


    pub fn transfer_admin(env: Env, caller: Address, nuevo_admin: Address) -> Result<(), Error> {
        // Obtener admin actual
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
    
        // Verificar permisos
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
    
        // Cambiar admin
        env.storage()
            .instance()
            .set(&DataKey::Admin, &nuevo_admin);
    
        Ok(())
    }
    




}


// =====================
// Módulo de pruebas
// =====================
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, Symbol, String, Address};
    // 👇 Importa el trait que habilita Address::generate(&env)
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);

        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic] // no imponemos el texto del panic
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.initialize(&admin); // debe panickear
    }

    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);

        client.initialize(&admin);

        let nombre = String::from_str(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);

        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }

    #[test]
    #[should_panic] // nombre vacío debe panickear
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);

        client.initialize(&admin);

        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio); // panick
    }

    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);

        client.initialize(&admin);

        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);

        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic] // no-admin debe panickear
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let otro = Address::generate(&env);

        client.initialize(&admin);

        client.reset_contador(&otro); // panick
    }
}
