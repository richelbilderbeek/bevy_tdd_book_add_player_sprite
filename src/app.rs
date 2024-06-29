use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[macro_use]
extern crate partial_application;

pub fn create_app(initial_player_position: Option<Vec3>) -> App {
    let mut app = App::new();
    let f = partial!(add_player_with_sprite => _, initial_player_position);
    app.add_systems(Startup, f);
    app.update();
    return app;
}

fn add_player(mut commands: Commands) {
    commands.spawn(Player);
}

fn add_player_with_sprite(mut commands: Commands, initial_player_position: Option<Vec3>) {
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
fn get_player_coordinat(app: &mut App) -> Vec3 {
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    return transform.translation;
}

#[cfg(test)]
fn get_all_components_names(app: &App) -> Vec<String> {
    use std::str::FromStr;

    let mut v: Vec<String> = Vec::new();
    for c in app.world.components().iter() {
        v.push(String::from_str(c.name()).unwrap());
    }
    return v;
}

#[cfg(test)]
fn print_all_components_names(app: &App) {
    for c in app.world.components().iter() {
        println!("{}", c.name())
    }
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
        app.add_systems(Startup, add_player);
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let app = create_app();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        assert_eq!(get_player_coordinat(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_player_is_at_custom_place() {
        let initial_coordinat = Vec3::new(1.2, 3.4, 5.6);
        let mut app = create_app(initial_coordinat);
        assert_eq!(get_player_coordinat(&mut app), initial_coordinat);
    }

    #[test]
    fn test_get_all_components_names_for_empty_app() {
        let app = App::new();
        let v = get_all_components_names(&app);
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_get_all_components_names_for_our_app() {
        let app = create_app();
        let v = get_all_components_names(&app);
        assert_eq!(v.len(), 8);
    }

    #[test]
    fn test_print_names_of_empty_app() {
        let app = App::new();
        print_all_components_names(&app);
        /*
        bevy_ecs::schedule::schedule::Schedules
        bevy_ecs::reflect::AppTypeRegistry
        bevy_app::main_schedule::MainScheduleOrder
        bevy_app::main_schedule::FixedMainScheduleOrder
        bevy_ecs::event::Events<bevy_app::app::AppExit>
        */
    }

    #[test]
    fn test_print_names_of_app_with_player() {
        let mut app = App::new();
        app.add_systems(Startup, add_player);
        app.update();
        print_all_components_names(&app);
        /*
        // First 5 of empty App, then
        bevy_ecs::schedule::stepping::Stepping
        bevy_tdd_book_add_player_sprite::app::Player
        bevy_ecs::event::EventUpdateSignal
        */
    }

    #[test]
    fn test_print_names_of_app_with_player_sprite() {
        let mut app = App::new();
        app.add_systems(Startup, add_player_with_sprite);
        app.update();
        print_all_components_names(&app);
        /*
        // First 5 of empty App, then:
        bevy_ecs::schedule::stepping::Stepping // From Player
        bevy_sprite::sprite::Sprite
        bevy_transform::components::transform::Transform
        bevy_transform::components::global_transform::GlobalTransform
        bevy_asset::handle::Handle<bevy_render::texture::image::Image>
        bevy_render::view::visibility::Visibility
        bevy_render::view::visibility::InheritedVisibility
        bevy_render::view::visibility::ViewVisibility
        bevy_tdd_book_add_player_sprite::app::Player // From Player
        bevy_ecs::event::EventUpdateSignal // From Player
        */
    }
}
