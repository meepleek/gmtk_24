use crate::prelude::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (animate,).run_if(level_ready));
}

#[derive(Component, Debug, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum PlayerAnimation {
    #[default]
    Idle,
    SwingAnticipation,
    SwingAnticipationIdle,
    Swing,
    SwingFast,
}

impl PlayerAnimation {
    fn len(&self) -> usize {
        match self {
            PlayerAnimation::Idle => 5,
            PlayerAnimation::SwingAnticipation => 3,
            PlayerAnimation::SwingAnticipationIdle => 4,
            PlayerAnimation::Swing => 5,
            PlayerAnimation::SwingFast => 3,
        }
    }

    pub(super) fn frame_base_duration_ms(&self, frame: usize) -> u64 {
        match self {
            PlayerAnimation::Swing => match frame {
                0 => 90,
                _ => 60,
            },
            PlayerAnimation::SwingFast => 80,
            _ => 100,
        }
    }

    fn is_idle(&self) -> bool {
        matches!(
            self,
            PlayerAnimation::Idle | PlayerAnimation::SwingAnticipationIdle
        )
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer);

fn animate(
    time: Res<Time>,
    mut player_q: Query<(&mut AnimationTimer, &mut PlayerAnimation, &mut Sprite), With<Player>>,
    mut word_tile_msg_r: MessageReader<WordTileEvent>,
    sprites: Res<SpriteAssets>,
) {
    let (mut timer, mut player_anim, mut sprite) = or_return!(player_q.get_single_mut());

    for ev in word_tile_msg_r.read() {
        let mut atlas = or_continue!(sprite.texture_atlas);
        if match ev.kind {
            WordTileEventKind::WordStarted => {
                atlas.layout = sprites.swing_anticipation_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::SwingAnticipation;
                true
            }
            WordTileEventKind::WordFinished(_) | WordTileEventKind::TileFinished { .. } => {
                atlas.layout = sprites.swing_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::Swing;
                true
            }
            _ => false,
        } {
            atlas.index = 0;
            timer.set_duration(Duration::from_millis(
                player_anim.frame_base_duration_ms(sprite.index),
            ));
            timer.reset();
            break;
        }
    }
    if word_tile_msg_r
        .read()
        .any(|ev| ev.kind == WordTileEventKind::WordStarted)
    {
        sprite.layout = sprites.swing_anticipation_anim_layout.clone_weak();
        *player_anim = PlayerAnimation::SwingAnticipation;
        sprite.index = 0;
        timer.set_duration(Duration::from_millis(
            player_anim.frame_base_duration_ms(sprite.index),
        ));
        timer.reset();
    }

    timer.tick(time.delta());
    if timer.just_finished() {
        sprite.index = (sprite.index + 1) % player_anim.len();
        if sprite.index == 0 && !player_anim.is_idle() {
            // todo: busy anticipation when the current anim is swing anticipation
            if *player_anim == PlayerAnimation::SwingAnticipation {
                sprite.layout = sprites.swing_anticipation_idle_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::SwingAnticipationIdle;
            } else {
                sprite.layout = sprites.idle_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::Idle;
            }
        }
        timer.set_duration(Duration::from_millis(
            player_anim.frame_base_duration_ms(sprite.index),
        ));
    }
}
