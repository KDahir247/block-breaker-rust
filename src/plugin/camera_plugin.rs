use bevy::prelude::*;

#[derive(SystemLabel, Debug, Hash, Eq, PartialEq, Clone)]
enum CameraOrder{
    GameCamera,
    UICamera,
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup_game_camera.system()
                .label(CameraOrder::GameCamera)
                .before(CameraOrder::UICamera))
            .add_startup_system(setup_ui_camera.system().label(CameraOrder::UICamera)
                .after(CameraOrder::GameCamera));
    }
}

fn setup_game_camera(mut command : Commands){
    command.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_ui_camera(mut command : Commands){
    command.spawn_bundle(UiCameraBundle::default());
}