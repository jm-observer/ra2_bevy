mod component;
pub mod res;
mod res_loader;
mod system;

use bevy::prelude::*;
use res::*;
use res_loader::*;
pub struct CursorCamera;
pub use component::*;
use ra2_asset::GameState;
use system::*;

/// 光标和相机
impl Plugin for CursorCamera {
    fn build(&self, app: &mut App) {
        app.add_system(cursor_status_new.in_schedule(OnEnter(GameState::Loading))
                       // SystemSet::on_enter(GameState::Loading).with_system(cursor_status_new),
        ).add_system(
            cursor_status_init.in_schedule(OnExit(GameState::Loading))
        )
            .add_system(
                cursor_draw.in_schedule(OnEnter(GameState::PlayTime))
            ) .add_systems((mouse_events_system, camera_move_system, keyboard_event_system
        )
            .in_set(OnUpdate(GameState::Prepare))
        );
        // .add_system_set(
        //     SystemSet::on_update(GameState::PlayTime)
        //         .with_system(mouse_events_system.label("cursor_move"))
        //         .with_system(
        //             camera_move_system
        //
        //                 .label("camera_move_system")
        //                 .after("cursor_move"),
        //         )
        //         .with_system(keyboard_event_system),
        // );
    }
}
