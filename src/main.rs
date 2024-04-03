fn affiche_grille(tableau: &Vec<Vec<char>>){
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
    for cpt in (0..7).step_by(2){
        if !colonne_libre(tableau, cpt){
            return false;
        }
    }
    return true;
}

fn jouer(tableau: &Vec<Vec<char>>){
    let joueur = 1; // Joueur 1 = X, joueur 2 = O

    affiche_grille(tableau);
    while (!egalite(tableau) && !gagne(tableau, joueur)){
        tour_joueur(tableau, joueur);
        affiche_grille(tableau);
        
        if(gagne(tableau, joueur)){
            println!("Le joueur {} a gagné !", joueur);
        }
        if (egalite(tableau)){
            println!("Egalité, toutes les colonnes sont pleines");
        }

        joueur = joueur%2+1;
    }
}


fn main() {
    let mon_tableau = initialiser_tableau();
    affiche_grille(&mon_tableau);
}
