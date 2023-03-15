use engine::Renderer;

mod engine;
mod game;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let mut renderer = Renderer::build("Breakout", WINDOW_WIDTH, WINDOW_HEIGHT)?;
    renderer.set_max_fps(120);
    renderer.set_color(150, 150, 150);

    let _ctx = renderer.window().gl_create_context()?;
    // At the end of the day this defines the correct GL functions based on which OS we're compiling for.
    gl::load_with(|name| renderer.window().subsystem().gl_get_proc_address(name) as *const _);

    let mut game = game::Game::build(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    game.play(&mut renderer);

    Ok(())
}
