use bevy::prelude::*;

use crate::component::ball_hit_event::BallHitEvent;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(play_hit_audio.system());
    }
}


fn play_hit_audio(mut event_reader: EventReader<BallHitEvent>,
                  asset_server: Res<AssetServer>,
                  audio_res: Res<Audio>) {
    for ball_hit in event_reader.iter() {
        let audio_clip = match ball_hit.destroyed_entity {
            true => { asset_server.load("audio/destroy.mp3") }
            false => { asset_server.load("audio/hit.mp3") }
        };

        audio_res.play(audio_clip);
    }
}