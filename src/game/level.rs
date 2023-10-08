use super::{
    analog_counter::AnalogCounterSetEvent,
    light::{FlashLight, LevelUpLamp},
    EventState, GameState,
};
use crate::prelude::*;
use std::time::Duration;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PointsEvent>()
            .add_event::<LevelUpEvent>()
            .add_systems(OnEnter(GameState::Ingame), init_resources)
            .add_systems(
                Update,
                (
                    level_up_system,
                    update_points_counter_system,
                    update_level_counter_system,
                    level_up_animation_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_add_points_system, on_level_up_lamp).run_if(in_state(EventState::Active)),
            );
    }
}

fn init_resources(mut cmds: Commands) {
    cmds.init_resource::<PointHub>();
    cmds.init_resource::<LevelHub>();
}

#[derive(Event, Clone, Copy)]
#[repr(u32)]
pub enum PointsEvent {
    BallCollided = 1,
    FlipperHit = 2,
    FoundationHit = 10,
    BallEnemyHit = 15,
    TowerHit = 20,
    EnemyDied = 85,
    TowerUpgrade = 500,
    TowerBuild = 1000,
}

impl PointsEvent {
    fn points(&self) -> Points {
        *self as Points
    }
}

#[cfg(debug_assertions)]
const POINT_FACTOR: u32 = 10;

#[cfg(not(debug_assertions))]
const POINT_FACTOR: u32 = 1;

fn on_add_points_system(mut points_ev: EventReader<PointsEvent>, mut points: ResMut<PointHub>) {
    for ev in points_ev.iter() {
        points.0 += ev.points() * POINT_FACTOR;
    }
}

pub type Points = u32;
pub type Level = u8;

#[derive(Resource, Default, Reflect)]

struct PointHub(Points);

#[derive(Resource, Default, Reflect)]
pub struct LevelHub {
    level: Level,
    points_level_up: Points,
}

impl LevelHub {
    fn is_level_up(&self, points: Points) -> bool {
        points >= self.points_level_up
    }

    fn level_up(&mut self) -> Level {
        self.level += 1;
        let factor = self.level as Points * 10;
        self.points_level_up = factor.pow(2) + factor * 200;
        self.level
    }

    pub fn foundation_hit_progress(&self) -> f32 {
        1. / (self.level as f32 * 3.)
    }
}

#[derive(Event, Clone, Copy)]
pub struct LevelUpEvent(pub Level);
// WIP
fn level_up_system(
    mut lvl_up_ev: EventWriter<LevelUpEvent>,
    mut level: ResMut<LevelHub>,
    points: Res<PointHub>,
) {
    if points.is_changed() && level.is_level_up(points.0) {
        let new_level = level.level_up();
        lvl_up_ev.send(LevelUpEvent(new_level));
        log!("🥳 Level up: {new_level}!");
    }
}

#[derive(Resource)]
pub struct PointCounterId(pub Entity);

impl Default for PointCounterId {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

fn update_points_counter_system(
    points: Res<PointHub>,
    mut ac_set_ev: EventWriter<AnalogCounterSetEvent>,
    pc_id: Res<PointCounterId>,
) {
    if points.is_changed() {
        ac_set_ev.send(AnalogCounterSetEvent::new(pc_id.0, points.0));
    }
}

#[derive(Resource)]
pub struct LevelCounterId(pub Entity);

impl Default for LevelCounterId {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

fn update_level_counter_system(
    level: Res<LevelHub>,
    mut ac_set_ev: EventWriter<AnalogCounterSetEvent>,
    lc_id: Res<LevelCounterId>,
) {
    if level.is_changed() {
        ac_set_ev.send(AnalogCounterSetEvent::new(lc_id.0, level.level as u32));
    }
}

#[derive(Component)]
struct LevelUpAnimation(Timer);

fn on_level_up_lamp(
    mut cmds: Commands,
    mut q_lvl_up_lamp: Query<(Entity, &mut Visibility), With<LevelUpLamp>>,
    level_up_ev: EventReader<LevelUpEvent>,
) {
    if !level_up_ev.is_empty() {
        let (lamp_id, mut visi) = q_lvl_up_lamp.single_mut();
        *visi = Visibility::Inherited;
        cmds.entity(lamp_id)
            .insert(FlashLight)
            .insert(LevelUpAnimation(Timer::new(
                Duration::from_secs(4),
                TimerMode::Once,
            )));
    }
}

fn level_up_animation_system(
    mut cmds: Commands,
    mut q_anim: Query<(Entity, &mut Visibility, &mut LevelUpAnimation)>,
    time: Res<Time>,
) {
    for (lamp_id, mut visi, mut anim) in &mut q_anim {
        if anim.0.finished() {
            *visi = Visibility::Hidden;
            cmds.entity(lamp_id)
                .remove::<FlashLight>()
                .remove::<LevelUpAnimation>();
        } else {
            anim.0.tick(time.delta());
        }
    }
}

//#[derive(Component)]
//struct PointDisplay;

//const SIZE: UVec2 = UVec2::new(224, 116);

//pub fn spawn_point_display(
//parent: &mut ChildBuilder,
//materials: &mut Assets<StandardMaterial>,
//images: &mut Assets<Image>,
//assets: &PinballDefenseAssets,
//) -> Handle<Image> {
//let size = Extent3d {
//width: SIZE.x,
//height: SIZE.y,
//..default()
//};

//// This is the texture that will be rendered to.
//let mut image = Image {
//texture_descriptor: TextureDescriptor {
//label: None,
//size,
//dimension: TextureDimension::D2,
//format: TextureFormat::Bgra8UnormSrgb,
//mip_level_count: 1,
//sample_count: 1,
//usage: TextureUsages::TEXTURE_BINDING
//| TextureUsages::COPY_DST
//| TextureUsages::RENDER_ATTACHMENT,
//view_formats: &[],
//},
//..default()
//};

//// fill image.data with zeroes
//image.resize(size);

//let image_handle = images.add(image);

//// This material has the texture that has been rendered.
//let material_handle = materials.add(StandardMaterial {
//base_color_texture: Some(image_handle.clone()),
//reflectance: 0.2,
//unlit: false,
//..default()
//});

//// Main pass cube, with material containing the rendered first pass texture.
//parent.spawn((
//PbrBundle {
//mesh: assets.world_1_point_display.clone(),
//material: material_handle,
//// Do not know, how to rotate the generated texture, so I rotate the object
//transform: Transform::from_rotation(Quat::from_rotation_y(PI / 2.))
//.with_translation(Vec3::new(0.98, -0.56, 0.051)),
//..default()
//},
//PointDisplay,
//));

//image_handle
//}

//const RENDER_LAYER: RenderLayers = RenderLayers::layer(2);

//#[derive(Component)]
//struct PointDisplayText;

//pub fn spawn_point_display_ui_and_cam(
//cmds: &mut Commands,
//assets: &PinballDefenseAssets,
//image_handle: Handle<Image>,
//) {
//cmds.spawn((NodeBundle {
//style: Style {
//border: UiRect::all(Val::Px(10.)),
//width: Val::Px(SIZE.x as f32),
//height: Val::Px(SIZE.y as f32),
//..default()
//},
//..default()
//},))
//.with_children(|parent| {
//let text = TextBundle::from_section(
//format!("Points: {}\nLevel: {}\n\nUpgrade ready", 0, 0),
//TextStyle {
//font: assets.digital_font.clone(),
//font_size: 12.0,
//color: Color::WHITE,
//},
//)
//.with_text_alignment(TextAlignment::Left);
//parent.spawn((
//Name::new("Points Display Text Bundle"),
//text,
//RENDER_LAYER,
//PointDisplayText,
//));
//});
//// The cube that will be rendered to the texture.
//cmds.spawn((
//Name::new("Points Display Texture Camera"),
//Camera2dBundle {
//camera_2d: Camera2d {
//clear_color: ClearColorConfig::Custom(Color::BLACK),
//},
//camera: Camera {
//// render before the "main pass" camera
//order: -1,
//target: RenderTarget::Image(image_handle.clone()),
//..default()
//},
//..default()
//},
//RENDER_LAYER,
//));
//}

//fn update_display_system(
//mut q_text: Query<&mut Text, With<PointDisplayText>>,
//points: Res<Points>,
//level: Res<Level>,
//) {
//if points.is_changed() || level.is_changed() {
//for mut text in q_text.iter_mut() {
//text.sections[0].value = format!("Points: {}\n\nLevel: {}\n", points.0, level.0);
//}
//}
//}
