# Vampire Duo

Mini Vampire Survivors en Rust (Bevy 0.16) : 1 personnage, 1 arme, pas d'or.

## Lancer
    cargo run                                  # 1re compilation longue (Bevy), ensuite rapide
    cargo run --features bevy/dynamic_linking  # recompilation encore plus rapide en dev
Déplacement : ZQSD/WASD ou flèches. L'arme tire toute seule.
Sous Linux, Bevy demande des libs système : voir https://bevy.org/learn/quick-start/getting-started/setup/

## Règles d'équipe
1. `./check.sh` doit passer avant tout merge (fmt + clippy -D warnings + tests).
2. Pas de `unwrap`/`expect`/`panic` (interdits par clippy). Utiliser `?`, `let Ok(..) else`, `match`.
3. Un `#[allow(...)]` doit toujours avoir un commentaire "Justifié : ...".
4. `components.rs`, `state.rs`, `error.rs` : modifs discutées à deux.
5. Une branche par fonctionnalité (`feat/xp`, `feat/ui`...), merge sur `main` via PR relue par l'autre.

## Répartition
| Fichier | Statut | Qui |
|---|---|---|
| player, weapon, collision | squelette fonctionnel | Dev A |
| enemy, ui | squelette fonctionnel | Dev B |
| xp | vide (TODO) | Dev B |
| concurrency/director | fonctionnel | Dev A |
| concurrency/saver, stats | fonctionnel | Dev B |

## Backlog (dans l'ordre)
1. Vérifier que le squelette compile et se joue (ensemble).
2. xp.rs : gemmes lâchées à la mort d'un ennemi (ajouter un composant `Gem`), aimantation, barre d'XP.
3. Niveaux : passer en `GameState::LevelUp`, écran 3 choix, modifier `WeaponStats`.
4. Vider le canal du Director en sortie de LevelUp (ou lui envoyer Pause/Resume).
5. Game over : bouton rejouer (reset des entités et de `GameStats`).
6. Highscore : lire `highscore.txt` au démarrage, ne sauvegarder que si meilleur.
7. Tests unitaires (formules de difficulté, `record_kill`...).
8. Finitions : sons, screen shake, vrais sprites.
