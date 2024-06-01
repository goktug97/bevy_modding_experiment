use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use bevy_modding_types::Character;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, hello_world_system);
    app.add_systems(Startup, setup);

    unsafe {
        let lib = libloading::Library::new("../plugin/target/release/libplugin.so")?;
        let setup: libloading::Symbol<unsafe extern "C" fn(&mut App)> = lib.get(b"setup")?;
        setup(&mut app);
        std::mem::forget(lib);
    }

    app.run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Character,
        Name::new("Test"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(10.0, 10.0, 0.0),
            ..default()
        },
    ));
}

fn hello_world_system() {
    println!("Hello, World!");
}
