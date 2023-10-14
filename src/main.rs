use macroquad::audio;
use macroquad::prelude::*;

fn collide(a: &Rect, b: &Rect) -> bool {
    // check les collisions
    let (left_a, left_b) = (a.x, b.x);
    let (right_a, right_b) = (a.x + a.w, b.x + b.w);
    let (top_a, top_b) = (a.y, b.y);
    let (bottom_a, bottom_b) = (a.y + a.h, b.y + b.h);
    if bottom_a <= top_b {
        return false;
    }

    if top_a >= bottom_b {
        return false;
    }

    if right_a <= left_b {
        return false;
    }

    if left_a >= right_b {
        return false;
    }

    //Si conditions collision detectee
    true
}

enum StatusJeu {
    EnJeu,
    Mort,
    Recommencer,
}

const PLAYER_SPEED: f32 = 450.;

struct Jeu {
    player: Rect,      // position du joueur
    cpu: Rect,         // position du cpu
    ball: Rect,        // position de la balle
    balldir: Vec2,     // direction de la balle
    ballspeed: f32,    // vitesse de la balle
    conteur: usize,    // conteur de points
    collisionne: bool, // la balle est en collision ?
    level_max: usize,  // nombre de points maximum
    son: audio::Sound, // son du jeu
}

impl Jeu {
    fn new(lvl_max: usize, son: audio::Sound) -> Jeu {
        Jeu {
            player: Rect::new(20., screen_height() / 2. - 40., 20., 80.),
            cpu: Rect::new(screen_width() - 40., screen_height() / 2. - 40., 20., 80.),
            ball: Rect::new(screen_width() / 2., screen_width() / 3., 15., 15.),
            balldir: Vec2::new(50., 50.),
            ballspeed: 50., // configuration initiale du jeu
            conteur: 0,
            collisionne: false,
            level_max: lvl_max,
            son,
        }
    }
    fn update(&mut self, delta: f32, status: &mut StatusJeu) {
        match status {
            StatusJeu::EnJeu => {
                // le joueur est encore en vie

                if is_key_down(KeyCode::Up) && self.player.y > 0. {
                    self.player.y += -PLAYER_SPEED * delta; // touche haut pressé
                }
                if is_key_down(KeyCode::Down) && self.player.y < screen_height() - 80. {
                    self.player.y += PLAYER_SPEED * delta; // touche bas pressé
                }

                self.ball.y += self.balldir.y * delta;
                self.ball.x += self.balldir.x * delta; // mouvement de la ballle
                if self.ball.y >= screen_height() - self.ball.h {
                    self.balldir.y = -self.balldir.y;
                    self.ball.y += self.balldir.y * delta;
                } // collisions de la balle avec les murs
                if self.ball.y <= 0. {
                    self.balldir.y = -self.balldir.y;
                    self.ball.y += self.balldir.y * delta;
                }
                if self.ball.x < 0. {
                    *status = StatusJeu::Mort; // detecter si le jeu est perdu
                    return;
                }
                //self.cpu.y = self.ball.y - 40.;                         // le cpu se déplace au niveau de la balle
                if self.ball.y > self.cpu.y + 75. {
                    self.cpu.y += PLAYER_SPEED * delta * 0.5;
                } else if self.ball.y < self.cpu.y {
                    self.cpu.y -= PLAYER_SPEED * delta * 0.5;
                }

                if collide(&self.ball, &self.cpu) {
                    self.balldir.x = -self.ballspeed;
                    self.ballspeed *= 1.1; // collision entre la balle est le cpu
                    self.balldir.y = rand::gen_range(-100., 100.);
                    audio::play_sound_once(&self.son);
                }

                if !collide(&self.ball, &self.player) {
                    self.collisionne = false
                }
                if collide(&self.ball, &self.player) && !self.collisionne {
                    self.collisionne = true;
                    self.balldir.x = self.ballspeed;
                    self.ballspeed *= 1.1; // collision entre la balle et le joueur
                    self.balldir.y = rand::gen_range(-self.ballspeed, self.ballspeed);
                    self.conteur += 1;
                    audio::play_sound_once(&self.son);
                }

                let conteur_s = self.conteur.to_string();
                draw_text(
                    &conteur_s,
                    screen_width() / 2. - measure_text(&conteur_s, None, 30, 1.).width / 2.,
                    20.,
                    30.,
                    BLACK,
                );
                draw_rectangle(
                    self.player.x,
                    self.player.y,
                    self.player.w,
                    self.player.h,
                    DARKBLUE,
                );
                draw_rectangle(self.cpu.x, self.cpu.y, self.cpu.w, self.cpu.h, DARKBLUE); // dessiner tout
                draw_rectangle(self.ball.x, self.ball.y, self.ball.w, self.ball.h, RED);
            }
            StatusJeu::Mort => {
                // si le joueur est mort
                if self.conteur > self.level_max {
                    self.level_max = self.conteur; // mettre à jour le compteur maximum
                }
                let lvl_max = format!("score maximum : {}", self.level_max);
                draw_text(
                    &lvl_max,
                    screen_width() / 2. - measure_text(&lvl_max, None, 40, 1.).width / 2.,
                    screen_height() / 2. - 40., // écrir le nombre de points maximum
                    40.,
                    RED,
                );
                draw_text(
                    "Vous êtes mort !",
                    screen_width() / 2. - measure_text("Vous êtes mort !", None, 40, 1.).width / 2.,
                    screen_height() / 2., // Vous êtes mort !
                    40.,
                    RED,
                );
                draw_text(
                    "appuyez sur espace pour recommencer.",
                    screen_width() / 2.
                        - measure_text("appuyez sur espace pour recommencer.", None, 40, 1.).width
                            / 2.,
                    screen_height() / 2. + 40., // appuyez sur espace pour recommencer
                    40.,
                    RED,
                );
                let conteur_s = format!("score : {}", self.conteur);
                draw_text(
                    &conteur_s,
                    screen_width() / 2. - measure_text(&conteur_s, None, 40, 1.).width / 2.,
                    screen_height() / 2. + 80., // affiche le score
                    40.,
                    RED,
                );
                if is_key_pressed(KeyCode::Space) {
                    *status = StatusJeu::Recommencer; // recommence si espace est appuyé
                }
            }
            StatusJeu::Recommencer => {
                *self = Jeu::new(self.level_max, self.son.clone()); // resset le jeu
                *status = StatusJeu::EnJeu;
            }
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        window_resizable: false, // configuration
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    next_frame().await;

    if let Ok(son) = audio::load_sound("pongue.wav").await {
        // charge le son du jeu
        let mut jeu = Jeu::new(0, son);
        let mut status = StatusJeu::EnJeu;

        loop {
            clear_background(LIGHTGRAY);

            let delta = get_frame_time();

            draw_text(format!("fps : {}", get_fps()).as_str(), 20., 20., 20., RED);

            jeu.update(delta, &mut status); // boucle du jeu

            next_frame().await;
        }
    } else {
        eprintln!("le fichier pongue.wav n'as pas été trouvé"); // le son n'as pas été trouvé
        loop {
            draw_text(format!("fps : {}", get_fps()).as_str(), 20., 20., 20., RED);
            draw_text(
                "le fichier pongue.wav n'as pas été trouvé",
                40.,
                screen_height() / 2.,
                40.,
                RED,
            );
            next_frame().await;
        }
    };
}
