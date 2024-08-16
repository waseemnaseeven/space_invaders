use rusty_audio::Audio;

use crossterm:: {
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    ExecutableCommand,
};

use std::{
    error::Error, 
    sync::mpsc,
    thread,
    time::{Duration, Instant}
};

use space_invaders::{
    audio::start_audio, frame::{new_frame, Drawable}, invaders::Invaders, player::Player, render
};



// Result is either Ok() or Error()
fn main() -> Result <(), Box<dyn Error>> {
    
    let mut audio = Audio::new();
    start_audio(&mut audio);

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?; // le mode "raw" désactive les traitements automatiques du terminal, permettant ainsi un contrôle plus direct sur les entrées du clavier
    stdout.execute(EnterAlternateScreen)?; // affiche un écran alternatif, qui sera utilisé pour le jeu
    stdout.execute(Hide)?; // cache le curseur

    // Render Loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = std::io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player: Player = Player::new();
    let mut instant: Instant = Instant::now();
    let mut invaders: Invaders = Invaders::new();

    // Game Loop 
    // start with a single quote to name the infinite loop, here the name is gameloop
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame: Vec<Vec<&str>> = new_frame();
    
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') =>  if player.shoot(){},
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {

        }
        if player.detect_hits(&mut invaders) {

        }

        // Draw && Render
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win or lose?
        if invaders.all_killed() {
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            break 'gameloop;
        }
    }

    // Clean up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?; // affiche le curseur
    stdout.execute(LeaveAlternateScreen)?; // quitte l'écran alternatif
    terminal::disable_raw_mode()?; // désactive le mode "raw"

    Ok(())

}

