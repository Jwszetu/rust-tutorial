

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: --- Ticket types

#[derive(Clone, Debug, Serialize)]
pub struct  Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketFromCreate {
    pub title: String,
}

// endregion: --- Ticket types

// region: --- Model Controller

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor


impl ModelController {
    pub async fn new() -> Result<Self> {
        return Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl  ModelController {
    // Create
    pub async fn create_ticket(&self, ticket_fc: TicketFromCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        return Ok(ticket);
    }

    //Read
    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        return Ok(tickets);
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        return ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}


// endregion: --- Model Controller



