use bevy::prelude::*;

#[derive(Event)]
struct MyEvent(usize);

#[derive(Resource)]
struct Counter(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Counter(0))
        .add_systems(Update, (start_update, event_producer))
        .add_observer(event_consumer)
        .run();
}

fn start_update(counter: Res<Counter>, mut close_writer: MessageWriter<AppExit>) {
    if 3 <= counter.0 {
        close_writer.write(AppExit::Success);
    } else {
        println!("start_update ({})!", counter.0);
    }
}

fn event_producer(mut commands: Commands, mut counter: ResMut<Counter>) {
    println!("Triggering MyEvent({})!", counter.0);
    commands.trigger(MyEvent(counter.0));
    counter.0 += 1;
}

fn event_consumer(event: On<MyEvent>) {
    println!("MyEvent({}) received!", event.0);
}
