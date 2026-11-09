use bevy::{prelude::*, ui::Pressed};
use logic_core::{CommandHistory, CommandPending, NandSimulation};

use crate::components::{LED_OFF, LED_ON, Led, LedId, Switch, SwitchState};

#[derive(Resource, Deref, DerefMut)]
pub struct SimRes(pub NandSimulation);

#[derive(Resource, Deref, DerefMut)]
pub struct HistoryRes(pub CommandHistory);

#[derive(Resource, Deref, DerefMut)]
pub struct PendingRes(pub CommandPending);

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimRes(Default::default()))
        .insert_resource(HistoryRes(Default::default()))
        .insert_resource(PendingRes(Default::default()))
        .add_systems(
            FixedUpdate,
            (
                update_inputs,
                apply_pending_commands,
                tick_simulation,
                update_outputs,
            )
                .chain(),
        )
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(Switch::new(0, 32.0, 32.0));
    commands.spawn(Switch::new(1, 32.0, 128.0));

    commands.spawn(Led::new(2, 160.0, 80.0));
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

fn update_outputs(mut led_querry: Query<(&LedId, &mut BackgroundColor)>, sim: Res<SimRes>) {
    for (id, mut bg_color) in &mut led_querry {
        if id.0 != 2 {
            continue;
        }
        *bg_color = if sim.0.get_out()[0] {
            BackgroundColor(LED_ON)
        } else {
            BackgroundColor(LED_OFF)
        };
    }
}

fn update_inputs(
    mut led_querry: Query<(&LedId, &mut SwitchState, &mut BackgroundColor), Changed<Pressed>>,
    mut pending: ResMut<PendingRes>,
) {
    for (id, mut state, mut bg_color) in &mut led_querry {
        if id.0 == 2 {
            continue;
        }
        state.toggle();
        pending.0.0.push(logic_core::Command::ToggleInput(id.0));
        *bg_color = if state.0 {
            BackgroundColor(LED_ON)
        } else {
            BackgroundColor(LED_OFF)
        };
    }
}

fn tick_simulation(mut sim: ResMut<SimRes>) {
    sim.tick();
}
