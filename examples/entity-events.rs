use bevy::prelude::*;

#[derive(EntityEvent)]
struct GreetEvent {
    entity: Entity,
    name: String,
}

#[derive(Component)]
struct Hello;

#[derive(Component)]
struct Bye;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, event_producer.run_if(elapsed(1.)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Hello).observe(on_hello);
    commands.spawn(Bye).observe(on_bye);
}

fn on_hello(event: On<GreetEvent>) {
    println!("Hello, {}", event.name);
}

fn on_bye(event: On<GreetEvent>) {
    println!("Bye, {}", event.name);
}

fn elapsed(threshold: f32) -> impl Fn(Res<Time>) -> bool {
    move |time: Res<Time>| {
        time.elapsed_secs() < threshold && threshold < time.elapsed_secs() + time.delta_secs()
    }
}

fn event_producer(
    mut commands: Commands,
    hello: Single<Entity, With<Hello>>,
    bye: Single<Entity, With<Bye>>,
) {
    commands.trigger(GreetEvent {
        entity: *hello,
        name: "Carl".to_string(),
    });
    commands.trigger(GreetEvent {
        entity: *bye,
        name: "Carl".to_string(),
    });
}
