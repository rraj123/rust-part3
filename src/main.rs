

use rand::{thread_rng,seq::SliceRandom};

#[derive(Debug)]
struct Deck{
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self{
        let suits =["Hearts","Diamond","Spades"];
        let values =["Ace","Two","Three"];
    
        let mut cards = vec![];
    
        for suit in suits{
            for value in values {
               let card = format!("{} of {}",value, suit);   
               cards.push(card);  
            }
        }
        
        let deck = Deck{cards};

        // All three of these are valid
        return deck;
        // return Deck { cards};
        Deck { cards } // Implicit return 
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
}


fn main() {
    let mut deck = Deck::new();
   
    deck.shuffle();
    println!("Heres your deck {:#?}", deck)

}
