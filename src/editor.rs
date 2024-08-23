mod editor_bundle;
mod editor_system;
mod number_input;
mod object_selector_bundle;
mod object_selector_system;

use bevy::prelude::*;
pub use editor_bundle::*;
use editor_system::*;
pub use object_selector_bundle::*;
use object_selector_system::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_editor_number_input,
                on_object_selector_input,
                on_selected_object_change,
                spawn_selected_object,
            ),
        )
        .init_resource::<SelectedObjectType>();
    }
}