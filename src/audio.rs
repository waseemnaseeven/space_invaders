use rusty_audio::Audio;

// On ajoute le & pour qu'il utilise l'instance de Audio sans la consommer (modifier)
pub fn start_audio(audio: &mut Audio) {

    audio.add("explode","audio/explode.wav");
    audio.add("lose","audio/lose.wav");
    audio.add("move","audio/move.wav");
    audio.add("pew","audio/pew.wav");
    audio.add("startup","audio/startup.wav");
    audio.add("win","audio/win.wav");
    audio.play("startup");

}
