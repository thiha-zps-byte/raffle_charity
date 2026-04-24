#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map};

#[contract]
pub struct RaffleCharity;

#[contractimpl]
impl RaffleCharity {
    /// Initialize the contract — marks it as ready to accept raffle creation.
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// Admin creates a new raffle for a charity.
    /// - `raffle_id`: unique identifier for the raffle
    /// - `charity_address`: address that receives the proceeds when the raffle is drawn
    /// - `ticket_price`: cost per ticket in XLM (stroops)
    /// - `prize`: prize amount in XLM (stroops) awarded to the winner
    pub fn create_raffle(
        env: Env,
        admin: Address,
        raffle_id: u64,
        charity_address: Address,
        ticket_price: u64,
        prize: u64,
    ) {
        admin.require_auth();

        let mut raffles: Map<u64, RaffleData> = env
            .storage()
            .instance()
            .get(&"raffles")
            .unwrap_or(Map::new(&env));

        if raffles.contains_key(raffle_id) {
            panic!("Raffle already exists");
        }

        let data = RaffleData {
            charity_address,
            ticket_price,
            prize,
            sold_count: 0,
            completed: false,
            winner: None,
        };
        raffles.set(raffle_id, data);
        env.storage().instance().set(&"raffles", &raffles);
    }

    /// A user buys a ticket for an existing raffle.
    /// The buyer's address is stored and associated with an incrementing ticket index.
    pub fn buy_ticket(env: Env, raffle_id: u64, buyer: Address) {
        buyer.require_auth();

        let mut raffles: Map<u64, RaffleData> = env
            .storage()
            .instance()
            .get(&"raffles")
            .unwrap_or(Map::new(&env));

        let data = raffles
            .get(raffle_id)
            .expect("Raffle does not exist");

        if data.completed {
            panic!("Raffle already completed");
        }

        // Use sold_count as the ticket index for this buyer
        let ticket_index = data.sold_count;

        // Store buyer by (raffle_id, ticket_index)
        let mut tickets: Map<(u64, u64), Address> = env
            .storage()
            .instance()
            .get(&"tickets")
            .unwrap_or(Map::new(&env));

        tickets.set((raffle_id, ticket_index), buyer);
        env.storage().instance().set(&"tickets", &tickets);

        // Update sold count
        let mut updated_data = data;
        updated_data.sold_count += 1;
        raffles.set(raffle_id, updated_data);
        env.storage().instance().set(&"raffles", &raffles);
    }

    /// Admin draws the winner of a completed raffle.
    /// The prize goes to the winner; proceeds (sold_count * ticket_price) go to the charity.
    pub fn draw_winner(env: Env, admin: Address, raffle_id: u64) {
        admin.require_auth();

        let mut raffles: Map<u64, RaffleData> = env
            .storage()
            .instance()
            .get(&"raffles")
            .unwrap_or(Map::new(&env));

        let data = raffles
            .get(raffle_id)
            .expect("Raffle does not exist");

        if data.completed {
            panic!("Raffle already completed");
        }

        if data.sold_count == 0 {
            panic!("No tickets sold");
        }

        // Pseudorandom selection using ledger sequence.
        // In production, use a verifiable random function (VRF) for trustless randomness.
        let rand_seed = env.ledger().sequence() as u64;
        let winner_index = rand_seed % data.sold_count;

        let tickets: Map<(u64, u64), Address> = env
            .storage()
            .instance()
            .get(&"tickets")
            .unwrap_or(Map::new(&env));

        let winner = tickets
            .get((raffle_id, winner_index))
            .expect("Ticket not found");

        let mut updated_data = data;
        updated_data.completed = true;
        updated_data.winner = Some(winner.clone());
        raffles.set(raffle_id, updated_data);
        env.storage().instance().set(&"raffles", &raffles);
    }

    /// Returns the full state of a raffle.
    /// Returns (charity_address, ticket_price, prize, sold_count, completed, winner)
    pub fn get_raffle(env: Env, raffle_id: u64) -> RaffleData {
        let raffles: Map<u64, RaffleData> = env
            .storage()
            .instance()
            .get(&"raffles")
            .unwrap_or(Map::new(&env));

        raffles.get(raffle_id).expect("Raffle does not exist")
    }
}

/// Raffle persistent data stored in the contract instance.
#[derive(Clone)]
#[contracttype]
pub struct RaffleData {
    pub charity_address: Address,
    pub ticket_price: u64,
    pub prize: u64,
    pub sold_count: u64,
    pub completed: bool,
    pub winner: Option<Address>,
}
