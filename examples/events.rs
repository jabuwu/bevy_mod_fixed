use bevy::prelude::*;
use bevy_mod_fixed::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FixedPlugin))
        // use `add_fixed_event` instead of `add_event`
        .add_fixed_event::<MyFixedEvent>()
        // ...
        .add_systems(Update, send_event)
        .add_systems(FixedUpdate, receive_event)
        .run();
}

#[derive(Event)]
pub struct MyFixedEvent;

fn send_event(mut events: EventWriter<MyFixedEvent>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        events.send(MyFixedEvent);
    }
}

fn receive_event(mut events: EventReader<MyFixedEvent>) {
    for _ in events.read() {
        println!("received event!");
    }
}
