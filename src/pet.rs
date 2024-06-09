use bevy::prelude::*;

#[derive(Component, Default)]
struct Pet;

#[derive(Bundle)]
struct PetBundle {
    sprite_sheet: SpriteSheetBundle,
    #[bundle(ignore)]
    _pet: Pet,
}
