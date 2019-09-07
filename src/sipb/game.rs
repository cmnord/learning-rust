struct PlayingState;
struct WonState;
struct LostState;

fn win_game(_p: PlayingState) -> WonState {
    WonState
}

// _prefix for unused variable
fn lose_game(_p: PlayingState) -> LostState {
    LostState
}

fn print_won(w: WonState) -> WonState {
    println!("Won!");
    w
}

fn print_lost(l: LostState) -> LostState {
    println!("Lost!");
    l
}


fn main() {
	// lose_game(32); 			// this should be a compile-time error

	println!("game 0 starting");
	let game0 = PlayingState;
	let won0 = win_game(game0);
	// print_lost(won0);			// This should be a compile-time error
	print_won(won0);

	// lose_game(game0);   		// This should be a compile-time error
	// lose_game(won0);   		// This should be a compile-time error
	// let gameA = game0;			// This should be a compile-time error
	// lose_game(gameA); 		// This should be a compile-time error

	println!("game 1 starting");
	let game1 = PlayingState;
	let lost1 = lose_game(game1);
	print_lost(lost1);
	
}