use bevy::prelude::*;

use bevy_modding_types::*;

fn hello_plugin() {
    println!("Hello, Plugin!");
}

fn move_character(mut query: Query<&mut Transform, With<Character>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.00;
    }
}

#[no_mangle]
pub extern "C" fn setup(app: &mut App) {
    app.add_systems(Update, hello_plugin);
    app.add_systems(Update, move_character);
}
