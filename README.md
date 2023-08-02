# Bevy Health System

## __Version Tracker__ 

| Bevy | bevy_health_system |
| ------------- | ------------- |
| 0.11          | 0.2.0         |
| 0.10          | 0.1.1         |

## __Table of contents__

- [What is bevy health system](#what-is-bevy_health_system)
- [Design Goals](#design-goals)
- [Useful Links](#useful-links)
- [Quick Start](#quick-start)
  - [No Bevy](#no-bevy)
  - [Explanation](#explanation)
- [How to remove bevy](#how-to-remove-bevy)
- [Examples](#examples)
- [LICENSE](#license)

## __What is bevy_health_system__

An easy way to add a health system to your bevy's entity. </br>
__NOTICE__: This package is not entirely depend on bevy, the only "bevy thing" this package use is the Component struct. This is described in: [NO BEVY](#no-bevy) section.

## __Design Goals__

- __Simple__: Easy to pick up, flexible and easy to extend.
- __Capable__: Any idea of health system can implement this system.

## __Useful Links__

- __Documentation - Docrs__: <https://docs.rs/bevy_health_system/latest/bevy_health_system/>
- __Download - Crates__: <https://crates.io/crates/bevy_health_system>
- __Repository - Github__: <https://github.com/thaiminh2022/bevy_health_system>

## __Quick Start__

1. Import the `HealthSystem` struct.
2. Add `HealthSystem` struct just like any component for your entity.

```rust
use bevy::prelude::*
use bevy_health_system::HealthSystem

fn spawn_player(mut commands: Commands) {
  commands.spawn(HealthSystem::new(100.0));
}
```

## __No Bevy__

### Explanation

This package is not entirely depend on bevy, the only "bevy thing" this package use is the Component struct.

```rust
use bevy::prelude::Component;
```

### How to remove Bevy

If you want to use this package independently with bevy, here's what to do.

1. Go to "src/lib"
2. Copy HealthSystem struct and its implementation to your rust code.
3. Remove the Component trait.

## __Examples__

I'm too lazy to add examples, check the docs. Should be easy to implement to your projects.

## __LICENSE__

MIT
