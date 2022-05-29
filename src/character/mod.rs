mod player;

use bevy::prelude::*;
use bevy_input_actionmap::ActionPlugin;
use bevy_inspector_egui::RegisterInspectable;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<player::Player>()
            .add_plugin(ActionPlugin::<player::Action>::default())
            .add_startup_system(player::setup)
            .add_startup_system(player::spawn_player)
            .add_system_to_stage(CoreStage::Update, player::movement)
            .add_system_to_stage(CoreStage::Update, player::attack)
            .add_system_to_stage(CoreStage::Update, player::roll)
            .add_system_to_stage(CoreStage::PostUpdate, player::state);
    }
}
