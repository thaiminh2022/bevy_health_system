//! Fast and easy way to add a health system to any bevy's entity
//! A solid foundation for any health system.
//!
//! # Quick start.
//! 1. Import the [`HealthSystem`] struct.
//! 2. Add [`HealthSystem`] struct just like any component for your entity.
//! ```
//! use bevy::prelude::*
//! use bevy_health_system::HealthSystem
//!
//! fn spawn_player(mut commands: Commands) {
//!     commands.spawn((Player{}, HealthSystem::new(100.0)))
//! }
//! ```
//! # Useful links
//!
//! # License
//! MIT

pub use self::health_system::HealthSystem;

pub mod health_system {
    use bevy::prelude::Component;

    /// Health System struct, the core of the entire crate.
    ///
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
        is_dead: bool,
    }

    impl HealthSystem {
        /// Create a new health system
        /// # Arguments
        /// * `max_health`: The max health of the health system, __health cannot exceed this value__
        pub fn new(max_health: f32) -> Self {
            return Self {
                health: max_health,
                max_health,
                is_dead: max_health < 0.0,
            };
        }

        /// Get the current health of the system.
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
            return self.health;
        }
        /// Get the current health of the system.
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
            return self.max_health;
        }
        /// Check if the system is dead
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0);
        ///     assert_eq!(false, health_system.is_dead());
        ///
        ///     health_system.deal_damage(90.0);
        ///     assert_eq!(true, health_system.is_dead());
        /// }
        /// `````
        pub fn is_dead(&self) -> bool {
            return self.is_dead;
        }
        /// Get normalized health of the system.
        /// this means: `health/max_health`
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0);
        ///     assert_eq!(0.9, health_system.get_health_normalized());
        /// }
        /// ```
        pub fn get_health_normalized(&self) -> f32 {
            return self.health / self.max_health;
        }

        /// Deal damage to the system.
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
        ///     health_system.deal_damage(10.0);
        ///     assert_eq!(90.0, health_system.get_health());
        /// }
        /// ```
        pub fn deal_damage(&mut self, amount: f32) {
            self.health -= amount;

            self.check_is_dead();
        }

        /// Kill the health system.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.kill_system();
        ///     assert_eq!(true, health_system.is_dead());
        /// }
        /// ```
        pub fn kill_system(&mut self) {
            self.health = 0.0;
            self.is_dead = true;
        }

        /// Heal the system.
        /// # Arguments
        /// * `amount`: The amount of damage dealt to this system.
        /// * `return`: The overflow heal value.
        /// # Example
        /// ```no_run
        /// use bevy::prelude::*;
        /// use bevy_health_system::HealthSystem;
        ///
        /// // Assuming health system was initialized with 100 health
        /// fn get_health_system(mut query: Query<&mut HealthSystem>) {
        ///     let mut health_system = query.get_single_mut().unwrap();
        ///     health_system.deal_damage(10.0);
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

            return overflow_value;
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
        ///     health_system.deal_damage(50.0);
        ///     assert_eq!(50.0, health_system.get_health());
        ///
        ///     health_system.heal_full();
        ///     assert_eq!(100.0, health_system.get_health());
        /// }
        /// ```
        pub fn heal_full(&mut self) {
            self.health = self.max_health;
        }

        /// Set the health of current health system
        /// # Arguments
        /// * `value`: This system new health value.
        /// * `return`: heal overflow amount.
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
            return self.max_health - value;
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

        fn check_is_dead(&mut self) {
            if self.health <= 0.0 {
                self.is_dead = true
            }
        }
    }
}
