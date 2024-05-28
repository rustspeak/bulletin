use std::io;
#[derive(Debug, Clone)]

enum Eleve {
    Nom,
    Moyene,
    Note,
    Coeficient,

}

impl Eleve {

    fn lecture(&self) -> &'static str {

         match self  {

         Eleve::Coeficient => "coefficient",
         Eleve::Nom  => "wato",
         Eleve::Moyene  => "moyene",
         Eleve::Note => "note",
         }
    }
}
    fn calcule(   ceoficientt: Vec<f64> ,  notte: Vec<f64> )  {

       let  _t = ceoficientt.len();

        if  ceoficientt.len() == notte.len() {

            for  i in 0.. {

                let   moyenne:f64  = notte[i] * ceoficientt[i] ;
            }
        }
    }

fn main() {


    println!(" entrez les notes de vos matiere  ");
     
     let mut note: Vec<f64> = vec![];
     let mut coefficient: Vec<f64>  = vec![];

    for  i in 0.. {

        let mut  entre1 = String::new();
        io::stdin().read_line(&mut entre1 ).expect("erreur  lors  de  la lecture");
        let   notes: f64 = entre1.trim().parse().expect("ereur  lors  de  la lecture ");  
        note.push(notes);

            if note.len() == 5 {
                break;
            }

            if  notes > 20.0 {
            break;
            }
    }
    println!("{:?}", note);

    println!("entre les  coefficient des  eleves  ");

        for    i in 0.. {

            let  mut  entre2 = String::new();
            io::stdin().read_line(&mut entre2 ).expect("erreur  lors  de  la lecture");
            let   coefficients : f64 = entre2.trim().parse().expect("ereur  lors  de  la lecture ");
            coefficient.push(coefficients);

            if  coefficient.len() == 5 {
                break;
            }
            if  coefficients > 20.0 {
                break;
            }
        }
        
        println!("{:?}" , coefficient);
       let   moyenne =  calcule(   coefficient, note);

        println!("{:?}" , moyenne);

}
