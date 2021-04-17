use bevy::prelude::*;

use crate::component::prelude::*;

pub struct Score(pub usize);


pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_text.system())
            .add_system(update_score.system());
    }
}

fn setup_text(mut commands: Commands, asset_server : Res<AssetServer>, window_data : Res<GameWindow>){

    commands
        .spawn_bundle(TextBundle{
            style : Style{
                align_self : AlignSelf::FlexStart,
                position_type : PositionType::Relative,
                position : Rect{
                    bottom : Val::Px(window_data.1 - 20.),
                    left : Val::Px(35.0),
                    ..Default::default()
                } ,
                ..Default::default()
            },
            text : Text::with_section("Score: 0", TextStyle{
                font : asset_server.load("font/blocks.ttf"),
                font_size : 20.,
                color: Color::BLACK
            }, TextAlignment{
                horizontal : HorizontalAlign::Center,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Score(0));
}


pub fn update_score(mut event_reader : EventReader<BallHitEvent>, mut text_query : Query<(&mut Text,&mut Score)>){

    if event_reader.iter().next().is_some(){
        for (mut text, mut score) in text_query.iter_mut() {

            score.0 = score.0 + 1;
            text.sections[0].value = format!("Score: {:?}", score.0);

        }
    }

}