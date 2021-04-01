use bevy::{
    prelude::*,
    render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
    window::WindowMode,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_startup_system(window_settings.system())
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        //.add_system(paddle_movement.system())
        .run();
}

struct Player {
    speed: f32,
    gravity: f32,
}

fn window_settings(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_mode(WindowMode::BorderlessFullscreen);
    window.set_cursor_visibility(true);
    // window.set_resizable(false);
    window.set_title("Bvy-Game".to_string());
}

fn player_movement (
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    for (
        player, mut transform
    ) in query.iter_mut() 
    {
        let mut direction = 0.0;
        let gravity_speed = 15.0;

        if keyboard_input.pressed(KeyCode::D) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            direction -= 1.0;
        }

        // if keyboard_input.pressed(KeyCode::Space) {
            
        // }

        // movement
        let tranlation = &mut transform.translation;
        tranlation.x += time.delta_seconds() * direction * player.speed;

        // gravity
        tranlation.y += time.delta_seconds() * gravity_speed * player.gravity;
        tranlation.y = tranlation.y.min(10.0).max(-300.0);
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    // add entities
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())

    // player
    .spawn(SpriteBundle {
    material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
    transform: Transform::from_translation(Vec3::new(500.0, -15.0, 0.0)),
    sprite: Sprite::new(Vec2::new(30.0, 70.0)),
    ..Default::default()
    })
    .with( Player { speed: 500.0, gravity: -9.81 } );

    // Text
    //.spawn(TextBundle {
    //text: Text {
    //font: asset_server.load("fonts/Fura Code Light Nerd Font Complete.ttf"),
    //value: "Hello: ".to_string(),
    //style: TextStyle {
    //color: Color::rgb(0.9, 0.9, 0.9),
    //font_size: 40.0,
    //..Default::default()
    //},
    //..Default::default()
    //},
    //style: Style {
    //position_type: PositionType::Absolute,
    //position: Rect {
    //top: Val::Px(10.0),
    //left: Val::Px(10.0),
    //..Default::default()
    //},
    //..Default::default()
    //},

    //..Default::default()
    //});
}
