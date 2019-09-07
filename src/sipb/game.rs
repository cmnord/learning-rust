struct PlayingState(i8);

impl Drop for PlayingState {
    fn drop(&mut self) {
        println!("Dropping a Playing")
    }
}

impl Clone for PlayingState {
    fn clone(&self) -> Self {
        PlayingState(self.0 + 1)
    }
}

struct WonState(i8);
struct LostState(i8);

fn win_game(p: PlayingState) -> WonState {
    WonState(p.0)
}

// _prefix for unused variable
fn lose_game(p: PlayingState) -> LostState {
    LostState(p.0)
}

fn print_won(w: &WonState) -> &WonState {
    println!("savepoint {} won!", w.0);
    &w
}

fn print_lost(l: &LostState) -> &LostState {
    println!("savepoint {} lost!", l.0);
    &l
}

fn main() {
    // lose_game(32); 			// this should be a compile-time error

    println!("game 0 starting");
    let game0 = PlayingState(0);
    let game0_1 = game0.clone();
    let won0 = win_game(game0);
    // print_lost(won0);			// This should be a compile-time error
    print_won(&won0);
    print_won(&won0);

    // lose_game(game0);   		// This should be a compile-time error
    // lose_game(won0);   		// This should be a compile-time error
    // let gameA = game0;			// This should be a compile-time error
    // lose_game(gameA); 		// This should be a compile-time error

    println!("game 1 starting");
    let lost1 = lose_game(game0_1);
    print_lost(&lost1);
    print_lost(&lost1);
}
