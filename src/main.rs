mod audio;
mod pong;
mod systems;
use crate::pong::Pong;
use crate::systems::PaddleStystem;

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    start_logger,
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    Result,
};
use audio::Music;
use systems::{BounceSystem, MoveBallsSystem, WinnerSystem};

fn main() -> Result<()> {
    start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let bindings_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(PaddleStystem, "paddle_system", &["input_system"])
        .with(MoveBallsSystem, "ball_system", &[])
        .with(
            BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;

    game.run();

    Ok(())
}
