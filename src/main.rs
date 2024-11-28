
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

    fn deal(&mut self,num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
    
}


fn main() {
    let mut deck = Deck::new();
   
    let cards = deck.deal(2);
    // Error handling .. to be added.
    println!("Heres your deck {:#?}", deck);
    println!("Heres your hand {:#?}", cards);

}
