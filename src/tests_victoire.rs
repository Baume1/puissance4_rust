fn verticale_bis(tableau: &Vec<Vec<char>>, joueur: char, ligne: usize, colonne: usize) -> bool {
    for cpt in 0..4 {
        if !(tableau[ligne - cpt][colonne] == joueur) {
            return false;
        }
    }
    return true;
}

fn verticale(tableau: &Vec<Vec<char>>, joueur: char) -> bool {
    for ligne in (3..6).rev() {
        for colonne in 0..=6 {
            if verticale_bis(tableau, joueur, ligne, colonne) {
                return true;
            }
        }
    }
    return false;
}

fn horizontale_bis(tableau: &Vec<Vec<char>>, joueur: char, ligne: usize, colonne: usize) -> bool {
    for cpt in 0..4 {
        if !(tableau[ligne][colonne + cpt] == joueur) {
            return false;
        }
    }
    return true;
}

fn horizontale(tableau: &Vec<Vec<char>>, joueur: char) -> bool {
    for ligne in 0..6 {
        for colonne in 0..4 {
            if horizontale_bis(tableau, joueur, ligne, colonne) {
                return true;
            }
        }
    }
    return false;
}

fn diagonale_vers_haut(
    tableau: &Vec<Vec<char>>,
    ligne: usize,
    colonne: usize,
    joueur: char,
) -> bool {
    for cpt in 0..4 {
        if !(tableau[ligne - cpt][colonne + cpt] == joueur) {
            return false;
        }
    }
    return true;
}

fn diagonale_vers_bas(
    tableau: &Vec<Vec<char>>,
    ligne: usize,
    colonne: usize,
    joueur: char,
) -> bool {
    for cpt in 0..4 {
        if !(tableau[ligne + cpt][colonne + cpt] == joueur) {
            return false;
        }
    }
    return true;
}

fn diagonale(tableau: &Vec<Vec<char>>, joueur: char) -> bool {
    // Commence en bas à gauche jusqu'en haut à droite
    for ligne in (3..6).rev() {
        for colonne in 0..4 {
            if diagonale_vers_haut(tableau, ligne, colonne, joueur) {
                return true;
            }
        }
    }
    // Commence en haut à gauche jusqu'en bas à droite
    for ligne in 0..3 {
        for colonne in 0..4 {
            if diagonale_vers_bas(tableau, ligne, colonne, joueur) {
                return true;
            }
        }
    }
    return false;
}

pub fn gagne(tableau: &Vec<Vec<char>>, joueur: i32) -> bool {
    let symbole_joueur;
    if joueur == 1 {
        symbole_joueur = 'X';
    } else {
        symbole_joueur = 'O';
    }
    return verticale(tableau, symbole_joueur)
        || horizontale(tableau, symbole_joueur)
        || diagonale(tableau, symbole_joueur);
}
