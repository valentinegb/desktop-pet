use std::ops::RangeInclusive;

use bevy::prelude::*;

#[derive(Component)]
pub(super) struct AnimationFrames(pub(super) RangeInclusive<usize>);

#[derive(Component)]
pub(super) struct AnimationTimer(pub(super) Timer);

#[derive(Bundle)]
pub(super) struct AnimatedSpriteSheetBundle {
    pub(super) sprite_sheet: SpriteSheetBundle,
    pub(super) frames: AnimationFrames,
    pub(super) timer: AnimationTimer,
}

pub(super) fn animate_sprite_sheet_system(
    mut query: Query<(&mut TextureAtlas, &AnimationFrames, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut atlas, frames, mut timer) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            atlas.index = if atlas.index == *frames.0.end() {
                *frames.0.start()
            } else {
                atlas.index + 1
            }
        }
    }
}
