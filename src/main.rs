use bevy::prelude::*;

fn main() {
    App::build()
    .add_resource(Msaa {samples: 4})
    .add_resource(WindowDescriptor {
        title: "Chess!".to_string(),
        width: 1600.0,
        height: 1600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins).run();
}
