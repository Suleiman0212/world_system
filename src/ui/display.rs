use crate::map::map::{MapConfig, MapState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use rand;

pub struct UIDisplayPlugin;
impl Plugin for UIDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_ui);
    }
}

fn display_ui(
    mut contexts: EguiContexts,
    mut map_config: ResMut<MapConfig>,
    mut map_state: ResMut<MapState>,
    //mut ambient_light: ResMut<AmbientLight>,
    //mut directional_light_query: Query<(&mut DirectionalLight, &mut Transform)>,
) {
    egui::Window::new("MapEditor").show(contexts.ctx_mut(), |ui| {
        ui.label("Seed");
        ui.add(egui::DragValue::new(&mut map_config.seed));
        if ui.button("Random").clicked() {
            map_config.seed = rand::random();
        }

        ui.label("NoiseScale");
        ui.add(egui::DragValue::new(&mut map_config.noise_scale));

        ui.label("Width|Lenght");
        ui.add(egui::Slider::new(&mut map_config.width, 1..=500));
        ui.add(egui::Slider::new(&mut map_config.lenght, 1..=500));

        if ui.button("Regenerate").clicked() {
            map_state.generated = false;
            map_state.tiles_filled = false;
        }

        // ui.label("AmbientLight");
        // ui.add(egui::Slider::new(
        //     &mut ambient_light.brightness,
        //     0.0..=10000.0,
        // ));
        //
        // let (mut directional_light, mut directional_light_transfrom) =
        //     match directional_light_query.get_single_mut() {
        //         Ok(d) => d,
        //         Err(_) => return,
        //     };
        //
        // ui.label("DirectionalLight");
        // ui.add(egui::Slider::new(
        //     &mut directional_light.illuminance,
        //     0.0..=1000000.0,
        // ));
        //
        // // Absolute shit
        // let mut new_x: f32 = 0.0; // directional_light_transfrom.rotation.x.to_degrees();
        // let mut new_y: f32 = 0.0; // directional_light_transfrom.rotation.y.to_degrees();
        // let mut new_z: f32 = 0.0; // directional_light_transfrom.rotation.z.to_degrees();
        //
        // ui.label("DirectionalLightRotation");
        // ui.add(egui::Slider::new(&mut new_x, 0.0..=360.0));
        // ui.add(egui::Slider::new(&mut new_y, 0.0..=360.0));
        // ui.add(egui::Slider::new(&mut new_z, 0.0..=360.0));
        //
        // let new_transform = Transform::from_rotation(Quat::from_euler(
        //     EulerRot::XYZ,
        //     new_x.to_radians(),
        //     new_y.to_radians(),
        //     new_z.to_radians(),
        // ));
        // *directional_light_transfrom = new_transform;
    });
}
