use bevy::{prelude::*, window::WindowCloseRequested};

#[derive(Message)]
struct MyMessage(usize);

#[derive(Resource)]
struct Counter(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<MyMessage>()
        .insert_resource(Counter(0))
        .add_systems(Update, (start_update, event_consumer, event_producer))
        .run();
}

fn start_update(
    counter: Res<Counter>,
    window: Single<Entity, With<Window>>,
    mut close_writer: MessageWriter<WindowCloseRequested>,
) {
    if 3 <= counter.0 {
        close_writer.write(WindowCloseRequested { window: *window });
    } else {
        println!("start_update ({})!", counter.0);
    }
}

fn event_producer(mut counter: ResMut<Counter>, mut writer: MessageWriter<MyMessage>) {
    println!("Sending MyMessage({})!", counter.0);
    writer.write(MyMessage(counter.0));
    counter.0 += 1;
}

fn event_consumer(mut reader: MessageReader<MyMessage>) {
    for event in reader.read() {
        println!("MyMessage({}) received!", event.0);
    }
}
