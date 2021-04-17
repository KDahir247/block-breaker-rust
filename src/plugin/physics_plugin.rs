use bevy::prelude::*;
use bevy_rapier2d::na::clamp;
use bevy_rapier2d::physics::*;
use bevy_rapier2d::rapier::dynamics::*;
use bevy_rapier2d::rapier::geometry::*;
use bevy_rapier2d::rapier::parry::na::Isometry2;
use rand::{Rng, thread_rng};


use crate::component::ball_hit_event::BallHitEvent;


pub  struct Brick(pub i8);
pub struct Paddle(i8);



#[derive(Clone)]
pub struct GamePhysicsPlugin {
    pub paddle_sprite : &'static str,
    pub ball_sprite : &'static str,
    pub horizontal_bound_sprite : &'static str,
    pub vertical_bound_sprite : &'static str,
    pub brick_sprite : &'static str
}


impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(self.clone())
            .add_plugin(RapierPhysicsPlugin)
            .add_startup_system(setup_physics.system())
            .add_system_set_to_stage(CoreStage::Update,
                                     SystemSet::new()
                                         .with_system(move_paddle.system())
                                         .with_system(handle_collision_event.system()));
    }
}


fn setup_physics(mut commands:  Commands, asset_server : Res<AssetServer>, mut material : ResMut<Assets<ColorMaterial>>, sprite_data : Res<GamePhysicsPlugin>) {

    spawn_paddle(&mut commands, &asset_server, &mut material, &sprite_data);
    spawn_ball(&mut commands, &asset_server, &mut material, &sprite_data);
    spawn_boundaries(&mut commands, &asset_server, &mut material, &sprite_data);
    spawn_bricks(&mut commands, Vec2::new(3.,2.), Vec2::new(100.,65.), &asset_server, &mut material, &sprite_data);

}

//Spawning
fn spawn_paddle(commands: &mut Commands,
                asset_server: &Res<AssetServer>,
                material: &mut ResMut<Assets<ColorMaterial>>,
                sprite_data: &Res<GamePhysicsPlugin>) {
    let paddle_rigidbody = RigidBodyBuilder::new_kinematic()
        .lock_rotations()
        .translation(0., -300.);

    let paddle_collider = ColliderBuilder::cuboid(35.0, 5.0)
        .friction(0.);

    commands.spawn_bundle(SpriteBundle {
        material: material.add(asset_server.load(sprite_data.paddle_sprite).into()),
        ..Default::default()
    }).
        insert_bundle((paddle_rigidbody, paddle_collider))
        .insert(Paddle(8));
}

fn spawn_ball(commands: &mut Commands,
              asset_server: &Res<AssetServer>,
              material: &mut ResMut<Assets<ColorMaterial>>,
              sprite_data: &Res<GamePhysicsPlugin>) {

    let rand_linear_x:f32 = thread_rng().gen_range(-100..=100) as f32;
    let rand_linear_y:f32  = thread_rng().gen_range(-30..=-5) as f32;

    let rigid_body2 = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 300.)
        .gravity_scale(9.81)
        .linvel(rand_linear_x, rand_linear_y);

    let collider2 = ColliderBuilder::ball(10.).friction(0.).restitution(2.01);

    commands.spawn_bundle(SpriteBundle { material: material.add(asset_server.load(sprite_data.ball_sprite).into()), ..Default::default() })
        .insert_bundle((rigid_body2, collider2));
}

fn spawn_boundaries(commands: &mut Commands,
                    asset_server: &Res<AssetServer>,
                    material: &mut ResMut<Assets<ColorMaterial>>,
                    sprite_data : &Res<GamePhysicsPlugin>) {
//Top Wall
    let top_rigidbody = RigidBodyBuilder::new_static()
        .translation(0., 300.);

    let top_collider = ColliderBuilder::cuboid(400., 35.)
        .friction(0.);

    commands.spawn_bundle(SpriteBundle { material: material.add(asset_server.load(sprite_data.horizontal_bound_sprite).into()), ..Default::default() })
        .insert_bundle((top_rigidbody, top_collider));

    //Left Wall
    let lhs_rigidbody = RigidBodyBuilder::new_static()
        .translation(-400., 0.)
        .can_sleep(true);

    let lhs_collider = ColliderBuilder::cuboid(35., 400.)
        .friction(0.);

    commands.spawn_bundle(SpriteBundle { material: material.add(asset_server.load(sprite_data.vertical_bound_sprite).into()), ..Default::default() })
        .insert_bundle((lhs_rigidbody, lhs_collider));


    //Right Wall
    let rhs_rigidbody = RigidBodyBuilder::new_static()
        .translation(400., 0.)
        .can_sleep(true);

    let rhs_collider = ColliderBuilder::cuboid(35., 400.)
        .friction(0.);

    commands
        .spawn_bundle(SpriteBundle { material: material.add(asset_server.load(sprite_data.vertical_bound_sprite).into()), ..Default::default() })
        .insert_bundle((rhs_rigidbody, rhs_collider));
}

fn spawn_bricks(commands : &mut Commands,
                row_col : Vec2,
                padding : Vec2,
                asset_server : &Res<AssetServer>,
                material :&mut  ResMut<Assets<ColorMaterial>>,
                sprite_data: &Res<GamePhysicsPlugin>){
    for column in (-row_col.y as i8)..=row_col.y as i8 {
        for row in (-row_col.x as i8)..=row_col.x as i8 {

            let texture_handle = asset_server.load(sprite_data.brick_sprite);

            let brick_rigidbody = RigidBodyBuilder::new_kinematic()
                .translation(row as f32 * padding.x , column as f32 * padding.y + 50.);

            let brick_collider = ColliderBuilder::cuboid(25., 5.)
                .friction(0.);


            commands.spawn_bundle(SpriteBundle{
                material : material.add(texture_handle.into()),
                ..Default::default()
            })
                .insert_bundle((brick_rigidbody, brick_collider))
                .insert(Brick(3));
        }
    }
}


//Action
fn move_paddle(key_board : Res<Input<KeyCode>>,
               mut rigid_bodies : ResMut<RigidBodySet>,
               mut physics_query : Query<(&Transform, &Paddle,
                                          &RigidBodyHandleComponent)>){
    for (transform, paddle, rigidbody_component) in physics_query.iter_mut() {

        let x_axis = (-(key_board.pressed(KeyCode::A) as i8) + (key_board.pressed(KeyCode::D) as i8)) * paddle.0;

        if let Some(rb) = rigid_bodies.get_mut(rigidbody_component.handle()){
            if x_axis != 0 {
                rb.set_next_kinematic_position(Isometry2::translation(clamp(transform.translation.x + x_axis as f32, -335.,335.), -300.0));
            }
        }
    }
}


fn handle_collision_event(mut commands : Commands,
                          mut event_writer : EventWriter<BallHitEvent>,
                          event : Res<EventQueue>,
                          mut destroyable_query : Query<(Entity, &ColliderHandleComponent,  &mut Brick)>){
    while let Ok(evt) = event.contact_events.pop(){

        let mut collider_pair : (ColliderHandle, ColliderHandle) = (ColliderHandle::invalid(), ColliderHandle::invalid());

        match evt {
            ContactEvent::Stopped(collider_a, collider_b) => { collider_pair = (collider_a, collider_b);}
            _ => {}
        }

        for (entity, collider,  mut brick) in destroyable_query.iter_mut() {

            if collider.handle().eq(&collider_pair.1) {


                if brick.0 <= 0 {
                    event_writer.send(BallHitEvent { destroyed_entity: true });

                    commands.entity(entity).despawn();
                }else{

                    event_writer.send(BallHitEvent { destroyed_entity: false });

                    brick.0 -= 1;
                }
            }
        }
    }
}