use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app(initial_player_position: Vec2, initial_player_size: Vec2) -> App {
    let mut app = App::new();
    let add_player_fn = move |commands: Commands| {
        add_player(commands, initial_player_position, initial_player_size);
    };
    app.add_systems(Startup, add_player_fn);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands, initial_player_position: Vec2, initial_player_size: Vec2) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec2::extend(initial_player_position, 0.0),
                size: Vec2::extend(initial_player_size, 1.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    return query.iter(app.world_mut()).len();
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    // Do 'app.update()' before calling this function,
    // else this assert goes off.
    assert_eq!(count_n_players(app), 1);
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_size(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.size.xy()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_can_create_app() {
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        create_app(initial_player_position, initial_player_size);
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let initial_player_position = Vec2::new(0.0, 0.0);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(initial_player_position, initial_player_size);
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_get_player_position() {
        let initial_player_position = Vec2::new(1.2, 3.4);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(initial_player_position, initial_player_size);
        app.update();
        assert_eq!(get_player_position(&mut app), initial_player_position);
    }

    #[test]
    fn test_player_has_a_custom_size() {
        let initial_player_position = Vec2::new(1.2, 3.4);
        let initial_player_size = Vec2::new(64.0, 32.0);
        let mut app = create_app(initial_player_position, initial_player_size);
        app.update();
        assert_eq!(get_player_size(&mut app), initial_player_size);
    }



}
