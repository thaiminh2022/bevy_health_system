//! Fast and easy way to add a health system to any bevy's entity
//! A solid foundation for any health system.
//!
//! # Quick start.
//! 1. Import the [`HealthSystem`] struct.
//! 2. Add [`HealthSystem`] struct just like any component for your entity.
//! ```
//! use bevy::prelude::*;
//! use bevy_health_system::HealthSystem;
//!
//! fn spawn_player(mut commands: Commands) {
//!     commands.spawn(HealthSystem::new(100.0));
//! }
//! ```
//! # Useful links
//!
//! ## Websites
//! - __Download - Crates__: <https://crates.io/crates/bevy_health_system>
//! - __Repository - Github__: <https://github.com/thaiminh2022/bevy_health_system>
//! - __Documentation - Docrs__: <https://docs.rs/bevy_health_system/0.0.1/bevy_health_system>
//!
//! ## Structs
//! 1. [HealthSystem]
//! 2. [HealthSystemState]
//! 3. [HealthSystemModifier]
//! 4. [HealthSystemReviveHealType]
//! # License
//! MIT

pub use self::health_system::{
    HealthSystem, HealthSystemModifier, HealthSystemReviveHealType, HealthSystemState,
};

pub mod health_system {
    use bevy::prelude::Component;

    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub enum HealthSystemState {
        /// __Default__. The entity having this system should be alive.
        #[default]
        ALIVE,
        /// The entity having this system should be dead.
        DEAD,
    }

    #[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
    pub enum HealthSystemModifier {
        /// __Default__.
        #[default]
        NONE,
        /// The entity having this system can't take any damage unless it was force.
        INVINCIBLE,
    }

    pub enum HealthSystemReviveHealType {
        /// Fully heal the system when revive.
        HealFull,

        /// Heal to this value when revive. Overflow health will be returned.
        HealTo(f32),

        /// Heal to a percentage of max health.
        HealPercentage(f32),
    }

    /// Health System struct, the core of the entire crate.
    /// # Fields
    /// * `health`: The current health of the system.
    /// * `max_health`: The max health of the system.
    /// * `system_state`: The current state of the system.
    /// * `system_modifier`: Modifier to this system, eg: Invincible,...
    /// # Examples
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_health_system::HealthSystem;
    /// //Spawn it using bevy commands
    /// fn set_up(mut commands: Commands) {
    ///     commands.spawn(HealthSystem::new(100.0));
    /// }
    /// //Query it like any other component
    /// fn get_player_health_system(query:Query<&HealthSystem>) {todo!();}
    ///
    ///```
    #[derive(Debug, Component)]
    pub struct HealthSystem {
        health: f32,
        max_health: f32,
        system_state: HealthSystemState,
        system_modifier: HealthSystemModifier,
    }

    impl HealthSystem {
        /// Create a new health system
        /// # Arguments
        /// * `max_health`: The max health of the health system, __health cannot exceed this value__
        pub fn new(max_health: f32) -> Self {
            let health_system_state = {
                if max_health < 0.0 {
                    HealthSystemState::DEAD
                } else {
                    HealthSystemState::default()
                }
            };

            Self {
                health: max_health,
                max_health,
                system_state: health_system_state,
                system_modifier: HealthSystemModifier::NONE,
            }
        }

        /// Returns the current health of the system.
        /// # Examples
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(query: Query<&HealthSystem>) {
        ///     let health_system = query.get_single().unwrap();
        ///     assert_eq!(100.0, health_system.get_health_max());
        /// }
        /// ```
        pub fn get_health(&self) -> f32 {
            self.health
        }
        /// Returns the current max health of the system.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let health_system = query.get_single_mut().unwrap();
        ///     assert_eq!(100.0, health_system.get_health());
        /// }
        /// ```
        pub fn get_health_max(&self) -> f32 {
            self.max_health
        }
        /// Returns true if the system is dead, else false
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0, false);
        ///     assert_eq!(false, health_system.is_dead());
        ///
        ///     health_system.deal_damage(90.0, false);
        ///     assert_eq!(true, health_system.is_dead());
        /// }
        /// `````
        pub fn is_dead(&self) -> bool {
            self.system_state == HealthSystemState::DEAD
        }
        /// Returns the normalized health of the system.
        /// this means: `health/max_health`
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0, false);
        ///     assert_eq!(0.9, health_system.get_health_normalized());
        /// }
        /// ```
        pub fn get_health_normalized(&self) -> f32 {
            self.health / self.max_health
        }

        /// Deal damage to the system.
        /// # Arguments
        /// * `amount`: The amount of damage dealt to this system.
        /// * `force`: Ignore any modifier that prevents dealing damages to this system.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0, false);
        ///     assert_eq!(90.0, health_system.get_health());
        /// }
        /// ```
        pub fn deal_damage(&mut self, amount: f32, force: bool) {
            if self.current_modifier_is(HealthSystemModifier::INVINCIBLE) && !force {
                return;
            }
            self.health -= amount;
            self.check_is_dead();
        }

        /// Kill the health system.
        // # Arguments
        /// * `force`: Ignore any modifier that prevents killing this system.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.kill_system(false);
        ///     assert_eq!(true, health_system.is_dead());
        /// }
        /// ```
        pub fn kill_system(&mut self, force: bool) {
            if self.current_modifier_is(HealthSystemModifier::INVINCIBLE) && !force {
                return;
            }
            self.health = 0.0;
            self.system_state = HealthSystemState::DEAD;
        }

        /// - Revive the health system, heal to full.
        /// - Also returns the the amount of health overflowed after revival.
        /// # Arguments
        /// * `revive_type`: The type of the revival.
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        /// use bevy_health_system::HealthSystemReviveHealType;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.kill_system(false);
        ///     assert_eq!(true, health_system.is_dead());
        ///
        ///     health_system.revive_system(HealthSystemReviveHealType::HealFull);
        ///     assert_eq!(false, health_system.is_dead());
        ///     assert_eq!(100.0, health_system.get_health());
        /// }
        /// ```
        pub fn revive_system(&mut self, revive_type: HealthSystemReviveHealType) -> f32 {
            self.system_state = HealthSystemState::ALIVE;
            let mut overflow_amount = 0.0;

            match revive_type {
                HealthSystemReviveHealType::HealFull => self.heal_full(),
                HealthSystemReviveHealType::HealPercentage(value) => {
                    let new_health = self.max_health * (value / 100.0);
                    self.set_health(new_health);
                }
                HealthSystemReviveHealType::HealTo(value) => {
                    overflow_amount = self.set_health(value);
                }
            }

            overflow_amount
        }
        /// - Heal the system.
        /// - Returns the amount of health that overflowed after heal.
        /// # Arguments
        /// * `amount`: The amount of damage dealt to this system.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0, false);
        ///     assert_eq!(90.0, health_system.get_health());
        ///
        ///     let over_flow_amount = health_system.heal(20.0);
        ///     assert_eq!(100.0, health_system.get_health());
        ///     assert_eq!(10.0, over_flow_amount);
        /// }
        /// ```
        pub fn heal(&mut self, amount: f32) -> f32 {
            let overflow_value = self.max_health - amount;
            self.health += amount;

            overflow_value
        }

        /// Heal the system fully.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(50.0, false);
        ///     assert_eq!(50.0, health_system.get_health());
        ///
        ///     health_system.heal_full();
        ///     assert_eq!(100.0, health_system.get_health());
        /// }
        /// ```
        pub fn heal_full(&mut self) {
            self.health = self.max_health;
        }

        /// - Set the health of current health system
        /// - Returns the health that overflowed when set a new health value
        /// # Arguments
        /// * `value`: This system new health value.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.set_health(10.0);
        ///     assert_eq!(10.0, health_system.get_health());
        ///
        ///     let over_flow_amount = health_system.set_health(200.0);
        ///     assert_eq!(100.0, health_system.get_health());
        ///     assert_eq!(100.0, over_flow_amount);
        /// }
        /// ```
        pub fn set_health(&mut self, value: f32) -> f32 {
            let mut final_value = value;

            if value < 0.0 {
                final_value = 0.0;
            } else if value > self.max_health {
                final_value = self.max_health;
            }

            self.check_is_dead();
            self.health = final_value;

            // overflow
            self.max_health - value
        }

        /// Set the max health of the system
        /// # Arguments
        /// * `value`: This system new max health value.
        /// * `heal_system`: heal the current health to the new max health.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.set_health_max(200.0, false);
        ///     assert_eq!(100.0, health_system.get_health());
        ///     assert_eq!(200.0, health_system.get_health_max());
        ///
        ///     health_system.set_health_max(400.0, true);
        ///     assert_eq!(400.0, health_system.get_health());
        ///     assert_eq!(400.0, health_system.get_health_max());
        /// }
        pub fn set_health_max(&mut self, value: f32, heal_system: bool) {
            let mut final_value = value;

            if value < 0.0 {
                final_value = 0.0;
            }

            self.max_health = final_value;
            if heal_system {
                self.heal_full();
            }

            self.check_is_dead();
        }
        /// Set a new modifier for this system
        /// # Arguments
        /// * `value`: This system new modifier.
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        /// use bevy_health_system::HealthSystemModifier;
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     assert_eq!(HealthSystemModifier::NONE, health_system.get_modifier());
        ///
        ///     health_system.set_modifier(HealthSystemModifier::INVINCIBLE);
        ///     assert_eq!(HealthSystemModifier::INVINCIBLE, health_system.get_modifier());
        /// }
        ///```
        pub fn set_modifier(&mut self, value: HealthSystemModifier) {
            self.change_modifier(value);
        }

        /// Returns the system current modifier
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        /// use bevy_health_system::HealthSystemModifier;
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     assert_eq!(HealthSystemModifier::NONE, health_system.get_modifier());
        /// }
        ///```
        pub fn get_modifier(&self) -> HealthSystemModifier {
            self.system_modifier
        }

        /// Returns whenever the current modifier is the specified modifier.
        /// # Arguments
        /// * `modifier`: The modifier to check.
        /// # Example
        /// ```
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        /// use bevy_health_system::HealthSystemModifier;
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     assert_eq!(HealthSystemModifier::NONE, health_system.get_modifier());
        /// }
        ///```
        pub fn current_modifier_is(&self, modifier: HealthSystemModifier) -> bool {
            self.get_modifier() == modifier
        }

        fn change_modifier(&mut self, modifier: HealthSystemModifier) {
            if self.is_dead() {
                return;
            }
            self.system_modifier = modifier;
        }
        fn check_is_dead(&mut self) {
            if self.health <= 0.0 {
                self.system_state = HealthSystemState::DEAD;
            }
        }
    }
}
