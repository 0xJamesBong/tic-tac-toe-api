use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::{error, info};
use uuid::Uuid;

use crate::game::board::Space;
use crate::game::game::Game;
use crate::game::mark::Mark;

use tokio::net::TcpListener;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MoveRequest {
    space: u8, // The space to mark (1-9)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameStateResponse {
    board: [Option<String>; 9],
    turn: String,
    game_id: Uuid,
}

#[derive(Clone)]
pub struct GameStore {
    games: Arc<Mutex<std::collections::HashMap<Uuid, Game>>>,
}

impl GameStore {
    pub fn new() -> Self {
        GameStore {
            games: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub fn add_game(&self, game: Game) -> Uuid {
        let game_id = Uuid::new_v4();
        self.games.lock().unwrap().insert(game_id, game);
        game_id
    }

    pub fn get_game(&self, game_id: &Uuid) -> Option<Game> {
        self.games.lock().unwrap().get(game_id).cloned()
    }

    pub fn make_move(&self, game_id: &Uuid, space: u8) -> Result<(), String> {
        let mut games = self.games.lock().unwrap();
        if let Some(game) = games.get_mut(game_id) {
            game.mark(space).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Game not found".to_string())
        }
    }

    pub fn update_game(&self, game_id: &Uuid, game: Game) -> Result<(), String> {
        let mut games = self.games.lock().unwrap();
        if games.contains_key(game_id) {
            games.insert(*game_id, game);
            Ok(())
        } else {
            Err("Game not found".to_string())
        }
    }
}

// curl -X POST http://localhost:50051/game/start
pub async fn start_game(
    game_store: axum::extract::Extension<GameStore>,
) -> Json<GameStateResponse> {
    println!("   ⭐ Starting a new game...");
    // info!("  ⭐ Starting a new game...");
    let game = Game::new();
    let game_id = game_store.add_game(game);
    let board = [None, None, None, None, None, None, None, None, None];

    let response = GameStateResponse {
        board, // board: [None; 9],
        turn: "X".to_string(),
        game_id: game_id,
    };

    Json(response)
}

#[axum::debug_handler]
pub async fn make_move(
    Path(game_id): Path<Uuid>,
    game_store: axum::extract::Extension<GameStore>,
    Json(request): Json<MoveRequest>,
) -> Result<Json<GameStateResponse>, String> {
    println!("   ⭐ Making a move in game ID: {}", game_id);
    // Mark the move in the game
    game_store.make_move(&game_id, request.space)?;

    // Retrieve the game state from the store
    let mut game = game_store.get_game(&game_id).ok_or("Game not found")?;

    // Update the game state
    game_store.update_game(&game_id, game)?;

    // Access the board and transform it into the desired format
    let board: [Option<String>; 9] = (0..9)
        .map(|i| {
            let space = Space::try_from(i as u8).unwrap();
            match game.board.get(space).unwrap() {
                Mark::X => Some("X".to_string()),
                Mark::O => Some("O".to_string()),
                Mark::Blank => None,
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Board should always have 9 spaces");

    // Prepare the response
    let response = GameStateResponse {
        board,
        turn: match game.turn {
            Mark::X => "X".to_string(),
            Mark::O => "O".to_string(),
            _ => "Unknown".to_string(),
        },
        game_id: game_id,
    };

    Ok(Json(response))
}

// returns an array of all game ids
pub async fn get_all_game_ids(game_store: axum::extract::Extension<GameStore>) -> Json<Vec<Uuid>> {
    let games = game_store.games.lock().unwrap();
    let game_ids = games.keys().cloned().collect();
    Json(game_ids)
}

// curl -v -X GET http://localhost:50051/game/state/{game_id}
pub async fn get_game_state(
    Path(game_id): Path<Uuid>,
    game_store: axum::extract::Extension<GameStore>,
) -> Result<Json<GameStateResponse>, String> {
    println!("⭐ Getting game state for ID: {}", game_id);

    // Retrieve the game from the store
    let game = game_store.get_game(&game_id).ok_or("Game not found")?;

    // Create the board array
    let board: [Option<String>; 9] = (0..9)
        .map(|i| {
            let space = Space::try_from(i as u8).unwrap();
            match game.board.get(space).unwrap() {
                Mark::X => Some("X".to_string()),
                Mark::O => Some("O".to_string()),
                Mark::Blank => None,
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("Board should always have 9 spaces");

    // Construct the response
    let response = GameStateResponse {
        board,
        turn: match game.turn {
            Mark::X => "X".to_string(),
            Mark::O => "O".to_string(),
            _ => "Unknown".to_string(),
        },
        game_id: game_id,
    };

    Ok(Json(response))
}

// pub async fn get_game_state(
//     Path(game_id): Path<Uuid>,
//     game_store: axum::extract::Extension<GameStore>,
// ) -> Result<Json<GameStateResponse>, String> {
//     println!("   ⭐ Getting game state... of {}", game_id);
//     // Retrieve the game from the store
//     let game = game_store.get_game(&game_id).ok_or("Game not found")?;

//     // Create the board array by iterating over all spaces (0 to 8)
//     let board: [Option<String>; 9] = (0..9)
//         .map(|i| {
//             let space = Space::try_from(i as u8).unwrap(); // Convert index to Space
//             match game.board.get(space).unwrap() {
//                 // Use `get` to fetch the mark
//                 Mark::X => Some("X".to_string()),
//                 Mark::O => Some("O".to_string()),
//                 Mark::Blank => None,
//             }
//         })
//         .collect::<Vec<_>>() // Collect into a vector first
//         .try_into() // Convert the vector to an array
//         .unwrap();

//     // Determine whose turn it is
//     let response = GameStateResponse {
//         board,
//         turn: match game.turn {
//             Mark::X => "X".to_string(),
//             Mark::O => "O".to_string(),
//             _ => "Unknown".to_string(), // Handle unexpected cases gracefully
//         },
//         game_id: game_id,
//     };

//     // Return the JSON response
//     Ok(Json(response))
// }

pub async fn start_server() {
    // Extract the address into a variable for easy modification
    let addr = "0.0.0.0:50051".to_string();
    println!("Starting the server on address: {}", addr); // Log the address being used

    // Initialize the game store
    let game_store = GameStore::new();
    println!("Game store initialized.");

    // Create the application router
    let app = Router::new()
        .route("/game/start", post(start_game)) // Route for starting a game
        .route("/game/state/:game_id", get(get_game_state)) // Route for getting the game state
        .route("/game/ids", get(get_all_game_ids)) // Route for getting all game IDs
        .route("/game/:game_id/move", post(make_move)) // Route for making a move
        .layer(axum::extract::Extension(game_store)); // Pass the game store extension
    println!("Router configured with /game/start route.");

    // Create a TcpListener from the address
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("TCP listener bound to address: {}", addr); // Log when the listener is successfully bound

    // Serve the app using the listener
    println!("Starting the server...");
    axum::serve(listener, app.into_make_service()) // Pass both TcpListener and app
        .await
        .unwrap();
}

//

// pub async fn start_server() {
//     // build our application with a single route
//     let app = Router::new().route("/", get(|| async { "Hello, World!" }));

//     // run our app with hyper, listening globally on port 3000
//     println!("Listening on port 3000");
//     let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }
