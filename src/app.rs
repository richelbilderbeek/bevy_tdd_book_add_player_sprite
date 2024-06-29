use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

/// Create a Bevy app
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_systems(Startup, setup_player);
    app.update();
    return app;
}

// Add the player to our world
fn setup_player(mut commands: Commands) {
    commands.spawn(Player);
}

// Add the player to our world
fn setup_player_with_sprite(mut commands: Commands) {
    commands.spawn((SpriteBundle { ..default() }, Player));
}

#[cfg(test)]
fn count_n_players(app: &App) -> usize {
    let mut n = 0;
    for c in app.world.components().iter() {
        // The complete name will be '[crate_name]::Player'
        if c.name().contains("Player") {
            n += 1;
        }
    }
    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let app = App::new();
        assert_eq!(count_n_players(&app), 0);
    }

    #[test]
    fn test_setup_player_adds_a_player() {
        let mut app = App::new();
        assert_eq!(count_n_players(&app), 0);
        app.add_systems(Startup, setup_player);
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let app = create_app();
        assert_eq!(count_n_players(&app), 1);
    }
}
