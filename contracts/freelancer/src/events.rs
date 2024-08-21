use soroban_sdk::{Env, Symbol, vec, IntoVal, Val, Address, Vec};
use crate::storage_types::DataKey;

// ------ Projects

// Event for project created
pub (crate) fn project_created(e: &Env, project_id: DataKey, client: Address, freelancer: Address, prices: Vec<u128>) {
    
    // Create message
    let topics = (Symbol::new(e, "The project has been successfully created"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    let client_val: Val = client.into_val(e);
    let freelancer_val: Val = freelancer.into_val(e);
    let prices_val: Val = prices.into_val(e);

    let event_payload = vec![e, project_id_val, client_val, freelancer_val, prices_val];
    e.events().publish(topics, event_payload);
}

// Event for project completed
pub (crate) fn project_completed(e: &Env, project_id: DataKey) {
    
    // Create message
    let topics = (Symbol::new(e, "The project has been successfully completed"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    e.events().publish(topics, project_id_val);
}

// Event for project cancelled
pub (crate) fn project_cancelled(e: &Env, project_id: DataKey) {
    
    // Create message
    let topics = (Symbol::new(e, "The project has been successfully cancelled"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    e.events().publish(topics, project_id_val);
}

// Event for project refunded
pub (crate) fn project_refunded(e: &Env, project_id: DataKey, client: Address, price: u128) {
    
    // Create message
    let topics = (Symbol::new(e, "The project has been successfully refunded"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    let client_val: Val = client.into_val(e);
    let price_val: Val = price.into_val(e);

    let event_payload = vec![e, project_id_val, client_val, price_val];

    e.events().publish(topics, event_payload);
}


// ------ Objectives

// Event for objective added
pub (crate) fn objective_added(e: &Env, project_id: &DataKey, objective_id: u128, price: u128) {
    
    // Create message
    let topics = (Symbol::new(e, "The objective has been successfully added"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    let objective_id_val: Val = objective_id.into_val(e);
    let price_val: Val = price.into_val(e);

    let event_payload = vec![e, project_id_val, objective_id_val, price_val];
    e.events().publish(topics, event_payload);
}

// Event for objective funded
pub (crate) fn objective_funded(e: &Env, project_id: DataKey, objective_id: u128, half_price: u128) {
    
    // Create message
    let topics = (Symbol::new(e, "The objective has been successfully funded"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    let objective_id_val: Val = objective_id.into_val(e);
    let half_price_val: Val = half_price.into_val(e);

    let event_payload = vec![e, project_id_val, objective_id_val, half_price_val];
    e.events().publish(topics, event_payload);
}

// Event for objective funded
pub (crate) fn objective_completed(e: &Env, project_id: DataKey, objective_id: u128, full_price: u128) {
    
    // Create message
    let topics = (Symbol::new(e, "The objective has been successfully completed"),);

    // Convertir los valores a `Val`
    let project_id_val: Val = project_id.into_val(e);
    let objective_id_val: Val = objective_id.into_val(e);
    let full_price_val: Val = full_price.into_val(e);

    let event_payload = vec![e, project_id_val, objective_id_val, full_price_val];
    e.events().publish(topics, event_payload);
}