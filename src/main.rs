mod plugin;
mod component;

use plugin::prelude::*;

use bevy::prelude::*;
use bevy::winit::*;
use component::prelude::*;

fn main() {

    let physics_plugin = GamePhysicsPlugin {
        paddle_sprite: "sprite/paddle.png",
        ball_sprite: "sprite/ball.png",
        horizontal_bound_sprite: "sprite/horizontal_boundary.png",
        vertical_bound_sprite: "sprite/vertical_boundary.png",
        brick_sprite: "sprite/brick.png"
    };


    App::build()
        .insert_resource(WindowDescriptor{
            title : "BlockSmash".to_string(),
            width : GAME_WIDTH,
            height : GAME_HEIGHT,
            resizable : false,
            ..Default::default()
        })
        .insert_resource(GameWindow(GAME_WIDTH, GAME_HEIGHT))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_event::<BallHitEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WinitPlugin::default())
        .add_plugin(GameCameraPlugin)
        .add_plugin(physics_plugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(GameUIPlugin)
        .run();
}
