use bevy::prelude::*;

fn hello_world()
{
    println!("Hello World!!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
        app.add_system(hello_world.system());
    }
}

fn main() 
{
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
