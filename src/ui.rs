use bevy::prelude::*;

use crate::components::{Health, Player};
use crate::concurrency::stats::SharedStats;
use crate::state::GameState;

pub struct UiPlugin;

#[derive(Component)]
struct HudText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud)
            .add_systems(OnEnter(GameState::GameOver), show_game_over);
    }
}

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        HudText,
        Text::new("PV : ..."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(8.0),
            ..default()
        },
    ));
}

fn update_hud(
    player: Query<&Health, With<Player>>,
    stats: Res<SharedStats>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let (Ok(hp), Ok(mut text)) = (player.single(), text.single_mut()) else {
        return;
    };
    let kills = stats.0.lock().map(|s| s.kills).unwrap_or(0);
    text.0 = format!("PV : {:.0}   Kills : {kills}", hp.0.max(0.0));
}

fn show_game_over(mut commands: Commands) {
    commands.spawn((
        Text::new("GAME OVER"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(45.0),
            left: Val::Percent(42.0),
            ..default()
        },
    ));
}
