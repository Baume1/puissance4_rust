use std::io;
mod tests_victoire;

fn affiche_grille(tableau: &Vec<Vec<char>>) {
    println!("┌───┬───┬───┬───┬───┬───┬───┐");
    for i in 0..6 {
        for j in 0..7 {
            print!("| {} ", tableau[i][j]);
        }
        println!("|");
        println!("├───┼───┼───┼───┼───┼───┼───┤");
    }
    for label in 0..7 {
        print!("│");
        print!(" {} ", label + 1);
    }
    println!("│");
    println!("└───┴───┴───┴───┴───┴───┴───┘");
}

fn initialiser_tableau() -> Vec<Vec<char>> {
    let taille = 7;
    let mut nouveau_tableau = Vec::new();
    for _ in 0..taille {
        let mut ligne = Vec::new();
        for _ in 0..taille {
            ligne.push(' ');
        }
        nouveau_tableau.push(ligne);
    }
    return nouveau_tableau;
}

fn colonne_libre(tableau: &Vec<Vec<char>>, colonne: usize) -> bool {
    return tableau[0][colonne] == ' ';
}

fn egalite(tableau: &Vec<Vec<char>>) -> bool {
    let mut resultat = true;
    for cpt in (0..7).step_by(2) {
        if colonne_libre(tableau, cpt) {
            resultat = false; // Si il y a ne serait-ce qu'une colonne de libre -> alors c'est pas égalité
        }
    }
    return resultat;
}

fn recuperer_input() -> i32 {
    let mut pos_joueur = String::new();

    println!("Entrez la position");
    io::stdin()
        .read_line(&mut pos_joueur)
        .expect("Erreur lors de la lecture de l'entrée");

    let mut resultat = 0;
    let mut puissance = 1;
    for caractere in pos_joueur.trim().chars().rev() {
        match caractere.to_digit(10) {
            Some(chiffre) => {
                resultat += chiffre as i32 * puissance;
                puissance *= 10;
            }
            None => {
                println!("Mauvais input, recommencez");
                return recuperer_input();
            }
        }
    }
    return resultat;
}
fn trouve_ligne(tableau: &Vec<Vec<char>>, colonne: usize, ligne: i32) -> i32 {
    if tableau[ligne as usize][colonne] == ' ' {
        return ligne as i32;
    } else if ligne >= 0 {
        return trouve_ligne(tableau, colonne, ligne - 1);
    }
    return -1;
}

fn place_jeton(tableau: &mut Vec<Vec<char>>, colonne: usize, symbole_joueur: char, _ligne: usize) {
    if colonne_libre(tableau, colonne) {
        let ligne = trouve_ligne(tableau, colonne, 5);
        if ligne >= -1 {
            tableau[ligne as usize][colonne] = symbole_joueur;
        }
    }
}

fn tour_joueur(tableau: &mut Vec<Vec<char>>, joueur: i32) {
    println!("Joueur {}, à vous de jouer !", joueur);
    let pos_joueur = recuperer_input() - 1; // Car le label des colonnes commence à 1

    if pos_joueur < 0 || pos_joueur >= 7 {
        println!("Position invalide");
        return tour_joueur(tableau, joueur);
    } else if !colonne_libre(tableau, pos_joueur as usize) {
        // Si la colonne n'est PAS libre
        println!("Colonne pleine.");
        return tour_joueur(tableau, joueur);
    }

    let ligne_max = 5;
    if joueur == 1 {
        place_jeton(tableau, pos_joueur as usize, 'X', ligne_max);
    } else {
        place_jeton(tableau, pos_joueur as usize, 'O', ligne_max);
    }
}

fn jouer(tableau: &mut Vec<Vec<char>>) {
    let mut joueur = 1; // Joueur 1 = X, joueur 2 = O
    let mut bool_victoire = false;
    let mut bool_egalite = false;
    affiche_grille(tableau);

    println!("Le joueur 1 utilise les pions 'X' et le joueur 2 les pions 'O'");
    while !bool_egalite && !bool_victoire {
        // Tant que personne n'a gagné et qu'il reste des colonnes libres
        tour_joueur(tableau, joueur);
        affiche_grille(tableau);

        bool_victoire = tests_victoire::gagne(tableau, joueur);
        bool_egalite = egalite(tableau);

        if bool_victoire {
            println!("\nLe joueur {} a gagné !", joueur);
        }
        if bool_egalite {
            println!("\nEgalité, toutes les colonnes sont pleines");
        }

        joueur = joueur % 2 + 1;
    }
}

fn main() {
    let mut mon_tableau = initialiser_tableau();
    jouer(&mut mon_tableau);
}
