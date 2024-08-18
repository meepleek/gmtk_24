use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<GoalBundle>("Goal")
        .init_resource::<LevelIndex>()
        .register_type::<LevelIndex>()
        .add_systems(OnEnter(Screen::Game), update_level_selection)
        .add_systems(Update, check_goal_reached.run_if(level_ready));
}

#[derive(Component, Debug, Default)]
pub struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    goal: Goal,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
struct LevelIndex(usize);

fn update_level_selection(lvl_index: Res<LevelIndex>, mut selected_lvl: ResMut<LevelSelection>) {
    *selected_lvl = LevelSelection::index(lvl_index.0);
}

fn check_goal_reached(
    goal_q: Query<&GridCoords, With<Goal>>,
    player_q: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut lvl_index: ResMut<LevelIndex>,
    mut cmd: Commands,
) {
    let player_coords = or_return_quiet!(player_q.get_single());
    let goal_coords = or_return_quiet!(goal_q.get_single());
    if player_coords == goal_coords {
        lvl_index.0 += 1;
        cmd.transition_to_screen(Screen::RestartGame);
    }
}
