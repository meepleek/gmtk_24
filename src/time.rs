use crate::prelude::*;
use std::{marker::PhantomData, time::Duration};

pub enum CooldownAction {
    RemoveComponent,
    DespawnRecursive,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Cooldown<T: Send + Sync + 'static> {
    timer: Timer,
    action: Option<CooldownAction>,
    _phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl<T: Send + Sync> Cooldown<T> {
    pub fn new(duration_ms: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(duration_ms), TimerMode::Once),
            action: None,
            _phantom: default(),
        }
    }

    pub fn remove_component(mut self) -> Self {
        self.action = Some(CooldownAction::RemoveComponent);
        self
    }

    pub fn despawn(mut self) -> Self {
        self.action = Some(CooldownAction::DespawnRecursive);
        self
    }
}

pub fn tick_cooldown<T: Send + Sync + Component>(
    mut cmd: Commands,
    mut cooldown_q: Query<(Entity, &mut Cooldown<T>)>,
    time: Res<Time>,
) {
    for (e, mut cooldown) in cooldown_q.iter_mut() {
        cooldown.timer.tick(time.delta());

        if cooldown.timer.just_finished() {
            let mut e_cmd = cmd.entity(e);
            e_cmd.remove::<Cooldown<T>>();

            if let Some(action) = &cooldown.action {
                match action {
                    CooldownAction::RemoveComponent => {
                        e_cmd.remove::<T>();
                    }
                    CooldownAction::DespawnRecursive => {
                        e_cmd.despawn();
                    }
                }
            }
        }
    }
}
