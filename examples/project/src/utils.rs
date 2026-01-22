use lithe::{browser, client, server};

#[client]
pub fn alert_from_utils() {
    browser::alert("Hello from src/utils.rs!");
}

#[server]
pub async fn get_server_data(id: i32) -> String {
    println!("Fetching data for ID: {}", id);
    format!("Server data for ID {}: Success!", id)
}
