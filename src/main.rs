use anyhow::{Context, Result};
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_flycam::prelude::*;
use dusk::prelude::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Cosmic Crew: Galaxy".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    asset_folder: {
                        if cfg!(target_os = "macos") {
                            "../Resources/assets".to_string()
                        } else {
                            "assets".to_string()
                        }
                    },
                }),
        )
        .add_plugin(PlayerPlugin)
        .add_system(close_on_esc)
        .add_startup_system(setup.pipe(handle_setup_errors))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result<()> {
    //plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::SEA_GREEN.into()),
        ..default()
    });

    // //model
    commands.spawn(SceneBundle {
        scene: asset_server.load("spacekit/models/craft_miner.glb#Scene0"),
        transform: Transform::from_rotation(Quat::from_rotation_y(-PI)),
        ..default()
    });

    let root_path = std::env::current_exe()
        .map(|path| {
            path.parent()
                .map(|exe_parent_path| exe_parent_path.to_owned())
                .unwrap()
        })
        .unwrap();

    let models_dir = root_path.join(MODELS_DIR);

    let length = std::fs::read_dir(models_dir.clone())
        .context(format!(
            "reading length of directory: {:?}",
            models_dir.clone()
        ))?
        .count() as f32;

    let area = length.sqrt().ceil() as u32;

    let mut rolling_x = 0;
    let mut rolling_z = 0;

    let errors = std::fs::read_dir(models_dir.clone())?
        .map(|entry| -> Result<()> {
            let path = entry?.path();

            ensure!(path.is_file(), "{path:?} is not a file!");

            //SAFETY: as we just ensured all of the items are actually files, this will
            // always work.
            let file_name = path.file_name().unwrap().to_str().unwrap();

            commands.spawn(SceneBundle {
                scene: asset_server.load(format!("{MODELS}{file_name}#Scene0")),
                transform: Transform::from_xyz((rolling_x * 2) as f32, 0., (rolling_z * 2) as f32),
                ..default()
            });

            commands.spawn(PbrBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: 0.01,
                        height: 10.,
                        ..default()
                    }
                    .into(),
                ),
                material: materials.add(Color::BLACK.into()),
                transform: Transform::from_xyz((rolling_x * 2) as f32, 0.3, (rolling_z * 2) as f32),
                ..default()
            });

            if rolling_x < area {
                rolling_x += 1;
            } else {
                rolling_x = 0;
                rolling_z += 1;
            }

            Ok(())
        })
        .filter_map(|x| x.err())
        .collect::<Vec<anyhow::Error>>();
    for err in errors {
        warn!("{:?}", err);
    }

    Ok(())
}

fn handle_setup_errors(In(result): In<Result<()>>) {
    if let Err(e) = result.context("Error in setup function.") {
        warn!("{:?}", e);
    }
}
