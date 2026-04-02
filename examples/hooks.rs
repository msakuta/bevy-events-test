use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Component)]
struct Inventory(HashSet<Entity>);

#[derive(Clone, Component)]
struct Item {
    inventory: Entity,
    name: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                start_update.run_if(elapsed(1.)),
                delete_item.run_if(elapsed(2.)),
                delete_inventory.run_if(elapsed(3.)),
                exit.run_if(elapsed(4.)),
            ),
        )
        .run();
}

fn setup(world: &mut World) {
    world
        .register_component_hooks::<Inventory>()
        .on_replace(|mut world, context| {
            info!("on_replace<Inventory>: {:?}", context.entity);
            let value = world.get::<Inventory>(context.entity).unwrap().0.clone();
            for target in value {
                world.commands().entity(target).despawn();
            }
        });
    world
        .register_component_hooks::<Item>()
        .on_replace(|mut world, context| {
            info!("on_replace<Item>: {:?}", context.entity);
            let inventory = world.get::<Item>(context.entity).unwrap().inventory;
            if let Some(mut target) = world.get_mut::<Inventory>(inventory) {
                target.0.remove(&context.entity);
            }
        });
}

fn elapsed(threshold: f32) -> impl Fn(Res<Time>) -> bool {
    move |time: Res<Time>| {
        time.elapsed_secs() < threshold && threshold < time.elapsed_secs() + time.delta_secs()
    }
}

fn start_update(mut commands: Commands) {
    let inventory = commands.spawn(()).id();
    let item = commands
        .spawn(Item {
            inventory: inventory,
            name: "Item A".to_string(),
        })
        .id();
    commands
        .entity(inventory)
        .insert(Inventory(std::iter::once(item).collect()));
    info!("Spawned an inventory");
}

fn delete_item(mut commands: Commands, items: Query<(Entity, &Item)>) {
    for (entity, item) in items {
        if item.name == "Item A" {
            info!("Deleting item!");
            commands.entity(entity).despawn();
        }
    }
}

fn delete_inventory(mut commands: Commands, inventories: Query<(Entity, &Inventory)>) {
    for (entity, inventory) in inventories {
        info!("Deleting inventory: {inventory:?}!");
        commands.entity(entity).despawn();
    }
}

fn exit(mut writer: MessageWriter<AppExit>) {
    writer.write(AppExit::Success);
}
