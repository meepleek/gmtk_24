# 2Do

## Game

### v0.1.0 typing

- [x] typing loop
- [x] move using typing - just single chars
- mine using words
  - [x] type words
  - [x] remove ground when fully typed
- [x] fix level reset
- [x] get a reasonable word collection to pull from
- [x] don't use words with movement characters/keys
- [x] add exit
- [x] move to next level

### v0.2.0 typing juice - initial pass

- type/mine animation
  - [x] animation
  - [x] screenshake
  - [x] SFX

### v0.3.0 platforming - initial pass

- [x] continuous/f32 movement/translation
- [x] add avian
- [x] track coords
- [x] player gravity
- [x] jump
- [x] clamp fall-speed
- [x] horizontal easing
- [x] coyote time
- [x] input buffer
- [x] variable jump height
- [x] apex gravity
- [ ] verlet integration (instead of euler)
- [x] wall-jump

### v0.4.0 obstacles

- [x] gravity enabled rocks 
  - [ ] rocks can crush the player
- [ ] spikes
- [ ] crush blocks
- [ ] bomb/mine

### v0.5.0 enemies

- [ ] patrolling melee enemy
- [ ] AoE enemy
- [ ] projectile (numbers, special chars) enemy
- [ ] wall spawner

### v0.6.0 combat

- [ ] deflect (followed by enemy stun + attack)
  - timed character or word to type
  - deflect projectiles too?
- [ ] headstomp or pogo (press char to pogo)

### v0.7.0 platforming - polish

- [ ] jump nudge/vertical correction
- [ ] movement horizontal correction (when leaving the edge)
- [x] wall slide
- [ ] wall jump leeway/correction (change to jump from hop etc.)


### better feedback

- [ ] highlight neighbour/mine-able tiles

### fog of war


## Notes

- game res => probly 320x180

