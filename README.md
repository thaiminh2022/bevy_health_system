# Bevy Health System

An easy way to add a health system to your bevy's entity. </br>
__NOTICE__: This package is not entirely depend on bevy, the only "bevy thing" this package use is the Component struct. This is describe in: [NO BEVY](#no-bevy) section.

# Design Goals

- __Simple__: Easy to pick up, flexible and easy to extend. 
- __Capable__: Any idea of health system can implement this system.

# Useful Links
- __Docrs__: https://docs.rs/bevy_health_system/0.0.1/bevy_health_system/
- __Github__: https://github.com/thaiminh2022/bevy_health_system

# Quick Start.
1. Import the `HealthSystem` struct.
2. Add `HealthSystem` struct just like any component for your entity.

```rust
use bevy::prelude::*
use bevy_health_system::HealthSystem

fn spawn_player(mut commands: Commands) {
  commands.spawn(HealthSystem::new(100.0));
}
```
# No Bevy
## Explaination
This package is not entirely depend on bevy, the only "bevy thing" this package use is the Component struct.
```rust
use bevy::prelude::Component;
```
## How to remove bevy
If you want to use this package indepentdently with bevy, here's what to do.
1. Go to "src/lib"
2. Copy HealthSystem struct and its implementation to your rust code. 
3. Remove the Component trait.

# LICENSE
MIT
