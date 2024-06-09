use std::ops::RangeInclusive;

use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, PrimaryWindow, WindowLevel},
};

#[derive(Component)]
struct Pet;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
enum CatAnimation {
    Idle1,
    Idle2,
    Clean1,
    Clean2,
    Run1,
    Run2,
    Sleep,
    Walk,
    Leap,
    Stretch,
}

impl CatAnimation {
    fn indices(&self) -> RangeInclusive<usize> {
        match self {
            Self::Idle1 => 0..=3,
            Self::Idle2 => 8..=11,
            Self::Clean1 => 16..=19,
            Self::Clean2 => 24..=27,
            Self::Run1 => 32..=39,
            Self::Run2 => 40..=47,
            Self::Sleep => 48..=51,
            Self::Walk => 56..=61,
            Self::Leap => 64..=70,
            Self::Stretch => 72..=79,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                        resizable: false,
                        decorations: false,
                        transparent: true,
                        window_level: WindowLevel::AlwaysOnTop,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, (setup_camera, setup_window, setup_pet))
        .add_systems(Update, (animate_sprite, sit_at_bottom))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_window(mut query: Query<&mut Window>) {
    for mut window in &mut query {
        window.set_maximized(true);
        window.cursor.hit_test = false;
    }
}

fn setup_pet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("cat-sprite-sheet.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(16.0, 16.0),
        8,
        10,
        Some(Vec2::new(16.0, 16.0)),
        Some(Vec2::new(7.0, 16.0)),
    ));
    let animation = CatAnimation::Sleep;

    commands.spawn((
        Pet,
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout,
                index: *animation.indices().start(),
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation,
        AnimationTimer(Timer::from_seconds(1.0 / 5.0, TimerMode::Repeating)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&CatAnimation, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (animation, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            atlas.index = if atlas.index == *animation.indices().end() {
                *animation.indices().start()
            } else {
                atlas.index + 1
            }
        }
    }
}

fn sit_at_bottom(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut pet_query: Query<(&mut Transform, &TextureAtlas), With<Pet>>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    match primary_window_query.get_single() {
        Ok(primary_window) => {
            for (mut transform, texture_atlas) in &mut pet_query {
                if let Some(texture_atlas_layout) = texture_atlas_layouts.get(&texture_atlas.layout)
                {
                    transform.translation = Vec3::new(
                        transform.translation.x,
                        -(primary_window.height() / 2.0)
                            + texture_atlas_layout.textures[texture_atlas.index].size().y
                                * transform.scale.y
                                / 2.0,
                        transform.translation.z,
                    );
                }
            }
        }
        Err(err) => error!("{err}"),
    }
}
