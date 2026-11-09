use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};
use logic_core::{CommandHistory, CommandPending, NandSimulation};

#[derive(Resource, Deref, DerefMut)]
pub struct SimRes(pub NandSimulation);

#[derive(Resource, Deref, DerefMut)]
pub struct HistoryRes(pub CommandHistory);

#[derive(Resource, Deref, DerefMut)]
pub struct PendingRes(pub CommandPending);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimRes(Default::default()))
        .insert_resource(HistoryRes(Default::default()))
        .insert_resource(PendingRes(Default::default()))
        .add_systems(
            FixedUpdate,
            (apply_pending_commands, tick_simulation).chain(),
        )
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            top: px(128),
            left: px(128),
            width: px(64),
            height: px(64),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(122, 22, 22)),
    ));

    // Spawn a Button
    commands.spawn((
        Button,
        Hovered::default(),
        Node {
            width: px(64),
            height: px(64),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(22, 22, 22)),
    ));
}

fn apply_pending_commands(
    mut pending: ResMut<PendingRes>,
    mut history: ResMut<HistoryRes>,
    mut sim: ResMut<SimRes>,
) {
    for cmd in pending.0.0.drain(..) {
        history.push(cmd, &mut sim);
    }
}

fn tick_simulation() {}
