mod animated_sprite_sheet;
mod pet;

use animated_sprite_sheet::{
    animate_sprite_sheet_system, AnimatedSpriteSheetBundle, AnimationFrames, AnimationTimer,
};
use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, PrimaryWindow, WindowLevel},
    winit::WinitWindows,
};
use winit::platform::macos::WindowExtMacOS;

#[derive(Component)]
struct Pet;

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
    fn frames(&self) -> AnimationFrames {
        match self {
            Self::Idle1 => AnimationFrames(0..=3),
            Self::Idle2 => AnimationFrames(8..=11),
            Self::Clean1 => AnimationFrames(16..=19),
            Self::Clean2 => AnimationFrames(24..=27),
            Self::Run1 => AnimationFrames(32..=39),
            Self::Run2 => AnimationFrames(40..=47),
            Self::Sleep => AnimationFrames(48..=51),
            Self::Walk => AnimationFrames(56..=61),
            Self::Leap => AnimationFrames(64..=70),
            Self::Stretch => AnimationFrames(72..=79),
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
        .add_systems(Update, (animate_sprite_sheet_system, sit_at_bottom_system))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_window(mut query: Query<(Entity, &mut Window)>, winit_windows: NonSend<WinitWindows>) {
    for (entity, mut window) in &mut query {
        window.set_maximized(true);
        window.cursor.hit_test = false;

        if let Some(winit_window) = winit_windows.get_window(entity) {
            winit_window.set_has_shadow(false);
        }
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
        AnimatedSpriteSheetBundle {
            sprite_sheet: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout,
                    index: *animation.frames().0.start(),
                },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            },
            frames: animation.frames(),
            timer: AnimationTimer(Timer::from_seconds(1.0 / 5.0, TimerMode::Repeating)),
        },
    ));
}

fn sit_at_bottom_system(
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
