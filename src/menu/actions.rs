use super::confirm_popup::{self, ConfirmPopup};
use super::{MenuState, SavedInPauseMenu, SettingsMenuState, SettingsReturnMenu};
use crate::AppState;
use crate::game::ResumeGameEvent;
use crate::game::{LevelHub, PendingLoad, PointHub, SAVE_DIR};
use crate::prelude::*;
use bevy::app::AppExit;
use moonshine_save::prelude::*;

#[derive(Message, Component, Debug, Clone, Default)]
pub enum MenuAction {
    #[default]
    Continue,
    NewGame,
    LoadGame,
    SaveGame,
    Save(String),
    Load(String),
    Settings,
    Back,
    Controls,
    Graphics,
    Sound,
    Quit,
    BackToMainMenu,
    ConfirmBackToMainMenu,
    CancelBackToMainMenu,
}

impl MenuAction {
    pub fn label(&self) -> &'static str {
        match self {
            MenuAction::Continue => "Continue",
            MenuAction::NewGame => "New Game",
            MenuAction::LoadGame => "Load Game",
            MenuAction::SaveGame => "Save Game",
            MenuAction::Save(_) => "Save",
            MenuAction::Load(_) => "Load",
            MenuAction::Settings => "Settings",
            MenuAction::Back => "Back",
            MenuAction::Controls => "Controls",
            MenuAction::Graphics => "Graphics",
            MenuAction::Sound => "Sound",
            MenuAction::Quit => "Quit",
            MenuAction::BackToMainMenu => "Main Menu",
            MenuAction::ConfirmBackToMainMenu => "Back to Main Menu",
            MenuAction::CancelBackToMainMenu => "Cancel",
        }
    }
}

pub fn on_menu_action(
    mut evr: MessageReader<MenuAction>,
    menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit_ev: MessageWriter<AppExit>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut settings_return: ResMut<SettingsReturnMenu>,
    mut resume_ev: MessageWriter<ResumeGameEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_popup: Query<Entity, With<ConfirmPopup>>,
    saved_in_pause_menu: Option<Res<SavedInPauseMenu>>,
) {
    for action in evr.read() {
        use MenuAction as MA;
        match action {
            MA::Continue => {
                next_menu_state.set(MenuState::None);
                resume_ev.write(ResumeGameEvent);
            }
            MA::NewGame => {
                next_menu_state.set(MenuState::None);
                app_state.set(AppState::Game);
            }
            MA::LoadGame => next_menu_state.set(MenuState::LoadGame),
            MA::SaveGame => next_menu_state.set(MenuState::SaveGame),
            MA::Save(path) => {
                let _ = std::fs::create_dir_all(SAVE_DIR);
                cmds.trigger_save(
                    SaveWorld::default_into_file(path)
                        .include_resource::<PointHub>()
                        .include_resource::<LevelHub>()
                        .exclude_component::<Mesh3d>()
                        .exclude_component::<MeshMaterial3d<StandardMaterial>>()
                        .exclude_component::<Name>()
                        .exclude_component::<LinearVelocity>()
                        .exclude_component::<AngularVelocity>()
                        .exclude_component::<Collider>()
                        .exclude_component::<RigidBody>()
                        .exclude_component::<CollisionLayers>()
                        .exclude_component::<CollisionEventsEnabled>()
                        .exclude_component::<DebugRender>()
                        .exclude_component::<Sensor>()
                        .exclude_component::<Mass>()
                        .exclude_component::<Restitution>()
                        .exclude_component::<Friction>()
                        .exclude_component::<SweptCcd>()
                        .exclude_component::<MaxLinearSpeed>()
                        .exclude_component::<SleepingDisabled>()
                        .exclude_component::<ColliderOf>()
                        .exclude_component::<RigidBodyColliders>()
                        .exclude_component::<ColliderMassProperties>()
                        .exclude_component::<ColliderDensity>()
                        .exclude_component::<ColliderTransform>()
                        .exclude_component::<ColliderAabb>()
                        .exclude_component::<ColliderMarker>()
                        .exclude_component::<ComputedMass>()
                        .exclude_component::<ComputedAngularInertia>()
                        .exclude_component::<ComputedCenterOfMass>()
                        .exclude_component::<Sleeping>()
                        .exclude_component::<SleepThreshold>()
                        .exclude_component::<SleepTimer>()
                        .exclude_component::<Position>()
                        .exclude_component::<Rotation>()
                        .exclude_component::<avian2d::collision::collider::EnlargedAabb>()
                        .exclude_component::<avian2d::dynamics::integrator::VelocityIntegrationData>()
                        .exclude_component::<avian2d::dynamics::rigid_body::forces::AccumulatedLocalAcceleration>()
                        .exclude_component::<avian2d::dynamics::solver::solver_body::SolverBody>()
                        .exclude_component::<avian2d::dynamics::solver::solver_body::SolverBodyInertia>()
                        .exclude_component::<avian2d::physics_transform::PreSolveDeltaPosition>()
                        .exclude_component::<avian2d::physics_transform::PreSolveDeltaRotation>()
                        .exclude_component::<ChildOf>()
                        .exclude_component::<Children>()
                        .exclude_component::<GlobalTransform>()
                        .exclude_component::<bevy::camera::visibility::ViewVisibility>()
                        .exclude_component::<bevy::camera::visibility::InheritedVisibility>()
                        .exclude_component::<bevy::camera::visibility::VisibilityClass>(),
                );
                cmds.insert_resource(SavedInPauseMenu);
                next_menu_state.set(MenuState::PauseMenu);
            }
            MA::Load(path) => {
                cmds.insert_resource(PendingLoad(path.clone()));
                next_menu_state.set(MenuState::None);
                app_state.set(AppState::Game);
            }
            MA::Settings => {
                settings_return.0 = menu_state.get().clone();
                next_menu_state.set(MenuState::Settings);
            }
            MA::Back => {
                settings_state.set(SettingsMenuState::None);
                let target = match menu_state.get() {
                    MenuState::LoadGame => MenuState::MainMenu,
                    MenuState::SaveGame => MenuState::PauseMenu,
                    MenuState::Settings => settings_return.0.clone(),
                    _ => MenuState::MainMenu,
                };
                next_menu_state.set(target);
            }
            MA::Controls => settings_state.set(SettingsMenuState::KeyboardControls),
            MA::Graphics => settings_state.set(SettingsMenuState::Graphics),
            MA::Sound => settings_state.set(SettingsMenuState::Sound),
            MA::Quit => {
                exit_ev.write(AppExit::Success);
            }
            MA::BackToMainMenu => {
                if saved_in_pause_menu.is_some() {
                    next_menu_state.set(MenuState::None);
                    app_state.set(AppState::MainMenu);
                } else {
                    confirm_popup::spawn(&mut cmds, &assets);
                }
            }
            MA::ConfirmBackToMainMenu => {
                confirm_popup::despawn(&mut cmds, &q_popup);
                next_menu_state.set(MenuState::None);
                app_state.set(AppState::MainMenu);
            }
            MA::CancelBackToMainMenu => {
                confirm_popup::despawn(&mut cmds, &q_popup);
            }
        }
    }
}
