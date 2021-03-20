use bevy::{
    prelude::*,
    render::pass::ClearColor
};

fn main() 
{
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_startup_system(setup.system())
        .add_system(world_system.system())
        .run();
}

fn setup
(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    // add entities
    commands
    // cameras
    .spawn(Camera2dBundle::default())
    .spawn(CameraUiBundle::default())

    // Text
    .spawn(TextBundle {
        text: Text {
            font: asset_server.load("fonts/Fura Code Light Nerd Font Complete.ttf"),
            value: "Hello: ".to_string(),
            style: TextStyle {
                color: Color::rgb(0.9, 0.9, 0.9),
                font_size: 40.0,
                ..Default::default()
            },
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },

        ..Default::default()
    });
}

fn world_system(mut query: Query<&mut Text>)
{
    for mut text in query.iter_mut()
    {
        text.value = format!("Hello: {}", "World");
    }
}