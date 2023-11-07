use bevy::prelude::*;
use bevy_mod_fixed::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FixedPlugin))
        // must run in set `FixedSet::Update`
        .add_systems(FixedUpdate, my_system.in_set(FixedSet::Update))
        // ...
        .run();
}

// use `FixedInput` instead of `Input`
fn my_system(keys: Res<FixedInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("pressed space!");
    }
}
