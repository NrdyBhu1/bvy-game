use bevy::{
    prelude::*,
    render::pass::ClearColor,
    // window::prelude::Window,
    window::WindowMode,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_startup_system(window_settings.system())
        .add_startup_system(setup.system())
        //.add_system(paddle_movement.system())
        .run();
}

fn window_settings(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    // window.set_mode(WindowMode::Fullscreen { use_size: false });
    // window.set_cursor_lock_mode(true);
    window.set_resizable(false);
    window.set_title("Bvy-Game".to_string());
}

fn setup(
    commands: &mut Commands,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    // add entities
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    // paddle
    //.spawn(SpriteBundle {
    //material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
    //transform: Transform::from_translation(Vec3::new(500.0, -15.0, 0.0)),
    //sprite: Sprite::new(Vec2::new(30.0, 120.0)),
    //..Default::default()
    //})
    //.with(Paddle{ speed: 800.0, player: true })

    //.spawn(SpriteBundle {
    //material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
    //transform: Transform::from_translation(Vec3::new(-500.0, -15.0, 0.0)),
    //sprite: Sprite::new(Vec2::new(30.0, 120.0)),
    //..Default::default()
    //})
    //.with(Paddle{ speed: 800.0, player: false });

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

//fn paddle_movement(
//time: Res<Time>,
//keyboard_input: Res<Input<KeyCode>>,
//mut query: Query<(&Paddle, &mut Transform)>,
//) {
//for(paddle, mut transform) in query.iter_mut()
//{
//if paddle.player {
//let mut direction = 0.0;

//if keyboard_input.pressed(KeyCode::Up){
//direction += 1.0;
//}

//if keyboard_input.pressed(KeyCode::Down) {
//direction -= 1.0;
//}

//let translation = &mut transform.translation;

//translation.y += time.delta_seconds() * direction * paddle.speed;

//}
//}
//}
