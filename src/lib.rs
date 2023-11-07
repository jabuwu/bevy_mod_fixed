use std::{hash::Hash, marker::PhantomData};

use bevy::{input::InputSystem, prelude::*, reflect::Reflect};

pub mod prelude {
    pub use super::{AddFixedEvent, FixedInput, FixedPlugin, FixedSet};
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FixedSystem {
    Input,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum FixedSet {
    Update,
    UpdateFlush,
    PostUpdate,
}

pub struct FixedPlugin;

impl Plugin for FixedPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_input::<KeyCode>()
            .add_fixed_input::<ScanCode>()
            .add_fixed_input::<MouseButton>()
            .add_fixed_input::<GamepadButton>()
            .add_systems(FixedUpdate, apply_deferred.in_set(FixedSet::UpdateFlush))
            .configure_sets(FixedUpdate, FixedSet::Update.before(FixedSet::UpdateFlush))
            .configure_sets(
                FixedUpdate,
                FixedSet::UpdateFlush.before(FixedSet::PostUpdate),
            )
            .insert_resource(Time::<Fixed>::from_seconds(1. / 60.));
    }
}

pub trait AddFixedInput {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl AddFixedInput for App {
    fn add_fixed_input<T: Copy + Eq + Hash + Send + Sync + 'static>(&mut self) -> &mut Self {
        self.init_resource::<FixedInput<T>>()
            .add_systems(PreUpdate, input_update::<T>.after(InputSystem))
            .add_systems(
                FixedUpdate,
                input_clear::<T>
                    .in_set(FixedSystem::Input)
                    .in_set(FixedSet::PostUpdate),
            );
        self
    }
}

#[derive(Clone, Deref, DerefMut, Debug, Reflect, Resource)]
#[reflect(Default)]
pub struct FixedInput<T: Copy + Eq + Hash + Send + Sync + 'static>(Input<T>);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Default for FixedInput<T> {
    fn default() -> Self {
        Self(Input::default())
    }
}

pub fn input_update<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
    input: Res<Input<T>>,
) {
    for pressed in input.get_just_pressed() {
        fixed_input.press(*pressed);
    }
    for released in input.get_just_released() {
        fixed_input.release(*released);
    }
}

pub fn input_clear<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mut fixed_input: ResMut<FixedInput<T>>,
) {
    fixed_input.clear();
}

pub trait AddFixedEvent {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self;
}

impl AddFixedEvent for App {
    fn add_fixed_event<T: Event>(&mut self) -> &mut Self {
        self.init_resource::<EventClearFlag<T>>()
            .init_resource::<Events<T>>()
            .add_systems(FixedUpdate, events_clear_flag::<T>)
            .add_systems(Last, events_clear::<T>);
        self
    }
}

#[derive(Resource)]
pub struct EventClearFlag<T: Event> {
    clear: bool,
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventClearFlag<T> {
    fn default() -> Self {
        Self {
            clear: false,
            _marker: PhantomData,
        }
    }
}

pub fn events_clear_flag<T: Event>(mut event_clear_flag: ResMut<EventClearFlag<T>>) {
    event_clear_flag.clear = true;
}

pub fn events_clear<T: Event>(
    mut event_clear_flag: ResMut<EventClearFlag<T>>,
    mut fixed_events: ResMut<Events<T>>,
) {
    if event_clear_flag.clear {
        fixed_events.update();
        event_clear_flag.clear = false;
    }
}
