use rand::Rng;

pub fn toggle_light(lights_state: &mut Vec<bool>, index: usize) {
    let state_len = lights_state.len() as u16;
    let side_size = (state_len as f64).sqrt() as usize;
    assert_eq!(
        (side_size * side_size) as u16,
        state_len,
        "lights_state's length should be a square"
    );

    let col = index % side_size;
    let row = index / side_size;

    lights_state[index] = !lights_state[index];
    if row > 0 {
        lights_state[index - side_size] = !lights_state[index - side_size];
    }
    if row < (side_size - 1) {
        lights_state[index + side_size] = !lights_state[index + side_size];
    }
    if col > 0 {
        lights_state[index - 1] = !lights_state[index - 1];
    }
    if col < (side_size - 1) {
        lights_state[index + 1] = !lights_state[index + 1];
    }
}

pub fn scramble(mut lights_state: &mut Vec<bool>, iterations: u8) {
    let lights_count = lights_state.len();
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        toggle_light(&mut lights_state, rng.gen_range(0..lights_count));
    }
}