///////////////////////////////////////////////////  ---> L
use rand::seq::SliceRandom;
use rand::thread_rng;

//(step 1) ---> Card
#[derive(Clone, Debug)]
struct Card{
    value:String, //the value of the card
    color:String  //the color of the card 
}
impl Card{
    fn get_value(&self) ->&str{ 
        &self.value
    }
    fn get_color(&self) ->&str{   
        &self.color
    }
    fn get_card_info(&self) ->Vec<String>{
        vec![self.get_value().to_string(), self.get_color().to_string()]
    }
}
//(step 2) --->Person
#[derive(Clone)]
struct Person{
    position_player:u8, // the player's position in the game
    cards:Vec<Card>,//Cards of player
}
impl Person{
    fn get_position_player(&self) ->u8{ 
        self.position_player
    }
    fn add_new_card_player(&mut self,new_card:Card){
        self.cards.push(new_card);
        println!("Card add successfully for {} Player {}","🧑",self.get_position_player()+1);
    }
    fn get_player_card(&self) ->&Vec<Card>{
        println!("{} Player {} cards :[ {:?} ]","🧑",self.get_position_player()+1,&self.cards);
        &self.cards
    }
}

//(step 3) --->Table
struct Table{
    players:Vec<Person>, //¨Players on the table
    table_cards:Vec<Card>, // Here we draw cards
    trash_card:Vec<Card>, // Here were we throw cards
    current_card:Option<Card>, // This is the card that is played 
    winners:Vec<Person> //here we save winners in order 
}
impl Table{
    fn get_cards(&self) ->Vec<Card>{
        //tools
        let simple_values=vec!["0","1","2","3","4","5","6","7","8","9","SKIP🚫","REVERSE🔄","+2"];
        let special_values=vec!["COLOR-CHANGE🎨","+4🌈"];
        let simple_colors=vec!["🔴","🟡","🟢","🔵"];
        let special_color="⬜";
        //cards
        let mut cards:Vec<Card>=Vec::new();
        let mut i=0;
        //simple color and value
        while i < simple_colors.len(){
            let mut j =0;
            while j < simple_values.len(){
                 if j != 0 {//for value 0 we only have 1 card
                    cards.push(Card{value:simple_values[j].to_string(),color:simple_colors[i].to_string()});
                 }
                 cards.push(Card{value:simple_values[j].to_string(),color:simple_colors[i].to_string()});
                 j+=1
            } 
            i+=1
        }
        //special color
        i=0;
        while i < special_values.len(){
            let mut j =0;
            while j < 4{ //we have 4 colors for each specials card 
                 cards.push(Card{value:special_values[i].to_string(),color:special_color.to_string()});
                 j+=1
            } 
            i+=1
        }
        println!("The program made {} cards {} succesfully {}",cards.len(),"🃏","✅");
        cards
    }
    fn shuffle_cards(&self,mut cards:Vec<Card>) ->Vec<Card>{
       let mut rng = thread_rng();
       cards.shuffle(&mut rng);
       cards
    }
    fn add_new_player(&mut self,new_player:Person){ 
         if self.players.len() < 10 {
            self.players.push(new_player);
            println!("Player {} added successfully {}", self.players.len(), "✅");
        } else {
            println!("The program can't have more than 10 players {}", "❌");
        }
    }
    fn set_table_cards(&mut self,cards:Vec<Card>){
         self.table_cards=self.shuffle_cards(cards.clone());;
         println!("{} All cards are ready  to be drawn {} ", "✅","🃏");
    }
    fn get_table_cards(&self) ->&Vec<Card>{
        println!("Cards = [ {:?} ]",&self.table_cards);
        &self.table_cards
    }
    fn draw_card(&mut self,player:&mut Person) {
        if self.table_cards.is_empty() { //if it's one card left 
           self.table_cards=self.shuffle_cards(self.trash_card.clone());
           self.trash_card.clear();
        } 
        if let Some(card) = self.table_cards.pop() {
            player.add_new_card_player(card);
            println!("Card add successfully for {} Player {}","🧑",player.get_position_player()+1);
        } else {
            println!("❌ No more cards to draw!");
        }
        
    }
    fn set_current_card(&mut self,card:Card,player:&Person){
        if let Some(current) = self.current_card.clone() {
           self.trash_card.push(current);
        }
        self.current_card = Some(card.clone()); // clone pour éviter le move
        println!("{} Player {} set {:?} as current card {}","🧑",player.get_position_player() + 1,card.get_card_info(),"🃏");
    }
    fn add_new_winners(&mut self,player:Person){
        self.winners.push(player.clone());
       if let Some(index) = self.players.iter().position(|p| p.position_player == player.position_player) {
          self.players.remove(index);
       }
        println!("{} Player {} is in position {} of winners list {}","🧑",player.get_position_player()+1,self.winners.len(),"🎊");
    }
}

fn main() {
    let mut table = Table {
        players: Vec::new(),
        table_cards: Vec::new(),
        trash_card: Vec::new(),
        current_card: None, // pas de carte courante au début
        winners: Vec::new(),
    };

    // Exemple : créer deux joueurs
    let player1 = Person { position_player: 0, cards: Vec::new() };
    let player2 = Person { position_player: 1, cards: Vec::new() };

    table.add_new_player(player1);
    table.add_new_player(player2);

    // Initialiser les cartes de la table
    let cards = table.get_cards();
    println!("Cards 0 {:?} et 128 {:?} ",cards[0].get_card_info(),cards[107].get_card_info());
    table.set_table_cards(cards);
    let h =table.get_table_cards();
}
