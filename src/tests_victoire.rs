fn verticale_bis(tableau: &Vec<Vec<char>>, joueur: char, ligne: usize, colonne: usize) -> bool {
    for cpt in 0..4 {
        if !(tableau[ligne - cpt][colonne] == joueur) {
            return false;
        }
    }
    return true;
}

fn verticale(tableau: &Vec<Vec<char>>, joueur: char) -> bool {
    for ligne in 5..2 {
        for colonne in 0..6 {
            if verticale_bis(tableau, joueur, ligne, colonne) {
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
    return verticale(tableau, symbole_joueur);
}
