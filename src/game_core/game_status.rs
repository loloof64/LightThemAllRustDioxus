pub fn is_game_won(lights_state: &Vec<bool>) -> bool {
    lights_state.iter().all(|&elt| elt == true)
}