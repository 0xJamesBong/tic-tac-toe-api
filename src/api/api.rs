use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::{error, info};
use uuid::Uuid;

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
    gameId: Uuid,
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
}

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
        gameId: game_id,
    };

    Json(response)
}

// pub async fn make_move(
//     Path(game_id): Path<Uuid>,
//     Json(request): Json<MoveRequest>,
//     game_store: axum::extract::Extension<GameStore>,
// ) -> Result<Json<GameStateResponse>, String> {
//     // Mark the move in the game
//     game_store.make_move(&game_id, request.space)?;

//     // Retrieve the game state from the store
//     let game = game_store.get_game(&game_id).ok_or("Game not found")?;

//     // Access the board directly and transform it into the desired response format
//     let board: [Option<String>; 9] = game
//         .board
//         .iter() // Assuming board is a collection of Option<Mark>
//         .map(|&mark| match mark {
//             Some(crate::mark::Mark::X) => Some("X".to_string()),
//             Some(crate::mark::Mark::O) => Some("O".to_string()),
//             None => None,
//         })
//         .collect::<Vec<_>>()
//         .try_into()
//         .unwrap();

//     // Prepare the response, including the current turn
//     let response = GameStateResponse {
//         board,
//         turn: if game.turn == crate::mark::Mark::X {
//             "X".to_string()
//         } else {
//             "O".to_string()
//         },
//     };

//     // Return the response as JSON
//     Ok(Json(response))
// }

// pub async fn get_game_state(
//     Path(game_id): Path<Uuid>,
//     game_store: axum::extract::Extension<GameStore>,
// ) -> Result<Json<GameStateResponse>, String> {
//     let game = game_store.get_game(&game_id).ok_or("Game not found")?;

//     let board: [Option<String>; 9] = game
//         .board
//         .board()
//         .iter()
//         .map(|&mark| match mark {
//             Some(crate::mark::Mark::X) => Some("X".to_string()),
//             Some(crate::mark::Mark::O) => Some("O".to_string()),
//             None => None,
//         })
//         .collect::<Vec<_>>()
//         .try_into()
//         .unwrap();

//     let response = GameStateResponse {
//         board,
//         turn: if game.turn == crate::mark::Mark::X {
//             "X".to_string()
//         } else {
//             "O".to_string()
//         },
//     };

//     Ok(Json(response))
// }

// pub async fn get_all_game_ids(game_store: axum::extract::Extension<GameStore>) -> Result<Json<GameStat

// pub async fn get_game_state(
//     Path(game_id): Path<Uuid>,
//     game_store: axum::extract::Extension<GameStore>,
// ) -> Result<Json<GameStateResponse>, String> {
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
//         gameId: game_id,
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
        // .route("game/state/:game_id", get(get_game_state)) // Route for getting the game state
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
