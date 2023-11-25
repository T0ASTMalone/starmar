use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct AnimationIndeces  {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    /*
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
    */
    
    // load sprite sheet
    let texture_handle = asset_server.load("../assets/Cat-Sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, Vec2::new(24.0, 24.0), 8, 6, None, None, 
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndeces { first: 1, last: 47 };

    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        }, 
        animation_indices,
    ));
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera_setup, setup))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
