struct UniverseDetails {

    universe_name: String,
    universe_winner: String,
    universe_population: u32
}

fn get_universe_details(universe_id: u32) -> Option<UniverseDetails> {

    let three = UniverseDetails {
        universe_name: "Star Wars".to_string(),
        universe_winner: "The Rebellion".to_string(),
        universe_population: 4294967295,
    };
    let five = UniverseDetails {
        universe_name: "Miraculous".to_string(),
        universe_winner: "Hawk Moth".to_string(),
        universe_population: 22,
    };
    let threefive = UniverseDetails {
        universe_name: "Stardew Valley".to_string(),
        universe_winner: "Jojo Corp".to_string(),
        universe_population: 1,
    };

    if universe_id % 3 == 0 && universe_id % 5 == 0 {
        return Some(threefive);
    } else if universe_id % 3 == 0 {
        return Some(three);
    } else if universe_id % 5 == 0 {
        return Some(five);
    } else {
        return None
    }
}


// this main function is fine, except for two gaps
// the print statements could make use of "{variable}" instead of 
// ("{}", variable)
fn main() {
    for id in 1..=15 {
        let universe_details = get_universe_details(id);
        if let Some(universe) = universe_details {
            let name = universe.universe_name;
            let winner = universe.universe_winner;
            let population = universe.universe_population;
            println!("Universe with id {id} is called {name}, won by {winner} and has a population of {population}");
        } else {
            println!("Universe with id {id} is unknown");
        }
    }
}
