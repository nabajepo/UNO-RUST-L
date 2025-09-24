///////////////////////////////////////////////////  ---> L
use rand::rng;
use chrono::Local;
use rand::seq::SliceRandom;
use std::io::{self, Write}; 


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
    }
    fn get_player_cards(&self) ->&Vec<Card>{
        println!("{} Player {} your cards : {:?} ","🧑",self.get_position_player(),&self.cards);
        &self.cards
    }
    fn remove_cards(&mut self,index:u8){
        println!("The card {:?} has been remove successfully for {} Player {}",self.cards[index as usize].get_card_info(),
                                                                               "🧑",self.get_position_player());
        self.cards.remove(index as usize);
    }
}

//(step 3) --->Table
struct Table{
    players:Vec<Person>, //¨Players on the table
    table_cards:Vec<Card>, // Here we draw cards
    trash_card:Vec<Card>, // Here were we throw cards
    current_card:Card, // This is the card that is played 
    winners:Vec<Person>,//here we save winners in order 
    current_pos_player:i8,//the position of the current player
    order_of_play:String,//if pair the order is -> if odd the order is <- 
    counter:u8,//to count the sequence of a card 
    color_chosen:String //the color for special card 
}
impl Table{
    fn get_cards(&self) ->Vec<Card>{ //here we get all cards 
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
    fn shuffle_cards(&self,mut cards:Vec<Card>) ->Vec<Card>{ //here we shuffle cards before the game 
       let mut rng = rng();
       cards.shuffle(&mut rng);
       cards
    }
    fn create_players(&mut self,number_p:u8){ //here we add players to the table 
         let mut index :u8=0;
         while index < number_p{
            self.players.push(Person { position_player: index, cards: Vec::new()});
            println!("Player {} added successfully {}", self.players.len()-1, "✅");  
            index+=1;
         }
    }
    fn input_players(&self,msg:&str,start:u8,end:u8,type_id:&str) ->u8{ //all input player as to enter 
    loop {
        let mut input = String::new();

        print!("{} => ", msg);
        io::stdout().flush().unwrap(); 

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input.parse::<u8>() {
            Ok(input_number) => {
                if (input_number >= end || input_number <start) && (type_id == "table" || type_id =="index_card"){
                    println!("Please choose the number of players >={} && <{} , try again ♻️",start,end);
                    continue; 
                }else if input_number !=1 && input_number !=2 && type_id=="choice_action"{
                    println!("Please choose the number between 1 and 2 , try again ♻️");
                    continue; 
                }
                return input_number; 
            }
            Err(_) => {
                println!("'{}' is not a valid number, try again ♻️", input);
            }
        }
    }
}
    fn set_table_cards(&mut self,cards:Vec<Card>){ //here we set cards after shuffling 
         self.table_cards=self.shuffle_cards(cards.clone());
         println!("{} All cards are ready  to be drawn {} ", "✅","🃏");
    }
    fn get_table_cards(&self) ->&Vec<Card>{//here we return all cards shuffled 
        println!("Cards = [ {:?} ]",&self.table_cards);
        &self.table_cards
    }
    fn draw_card(&mut self) -> Option<Card>{ //here a person draw a card 
        if self.table_cards.is_empty() {
            if self.trash_card.is_empty() {
                println!("❌ No more cards to draw!");
                return None;
            }
            self.table_cards = self.shuffle_cards(self.trash_card.clone());
            self.trash_card.clear();
        }
        self.table_cards.pop() 
    }
    fn set_current_card(&mut self,card:Card,player:&Person,type_id:&str){ //here we set a current card
        if type_id == "table" {
           if card.get_value() == "+2"  ||
              card.get_value() == "REVERSE🔄" ||
              card.get_value() == "COLOR-CHANGE🎨" ||
              card.get_value() == "+4🌈" ||
              card.get_value() == "SKIP🚫" {
                self.set_table_cards(self.table_cards.clone());
                self.set_current_card(self.table_cards[self.table_cards.len() - 1].clone(), &Person { position_player: 254, cards: Vec::new()}, "table");  
            }else{
               self.current_card = card.clone(); 
               self.table_cards.pop();
               println!(" =======> Table set {:?} as current card {}",card.get_card_info(),"🃏");
            }  
             
        }else {
           self.trash_card.push(self.current_card.clone());
           self.current_card = card.clone(); 
           println!(" =======> {} Player {} set {:#?} as current card {}","🧑",player.get_position_player() ,card.get_card_info(),"🃏");
        }
    }
    fn add_new_winners(&mut self,player:Person){ //here we add in order winners 
        self.winners.push(player.clone());
       if let Some(index) = self.players.iter().position(|p| p.position_player == player.position_player) {
          self.players.remove(index);
       }
        println!("{} Player {} is in position {} of winners list {}","🧑",player.get_position_player()+1,self.winners.len(),"🎊");
    }
    fn skip_reverse_card(&mut self) {//here we what happen to the game after using skip and reverse card 
       let len = self.players.len();
       if self.current_card.get_value() == "SKIP🚫" {//if skip
            self.current_pos_player=self.get_next_pos() as i8;    
       } else {//if reverse card 
            if self.order_of_play == "pair".to_string() {
                self.order_of_play = "odd".to_string();
            } else {
                self.order_of_play = "pair".to_string();
          }
       }
    }
    fn get_next_pos(&self) ->u8{ //here we are looking for the next pos of player
        if self.current_pos_player == 0 && self.order_of_play == "odd".to_string(){
            ((self.players.len() as u8) - 1) as u8
        }else if self.current_pos_player == ((self.players.len() - 1) as i8) && 
                 self.order_of_play == "pair".to_string(){
            0 as u8
        }else if self.order_of_play == "odd".to_string(){
            (self.current_pos_player - 1) as u8
        }else{//pair
            (self.current_pos_player + 1) as u8
        }         
    }
    fn is_card_fit(&self,card:Card) ->bool{ //we check if the card given  is adapted to the current 
        if self.current_card.get_value() != card.get_value() && 
           self.current_card.get_color() != card.get_color() && 
           card.get_color() != "⬜"{
           return false;  
        }   
        true
    }
    fn is_card_particular(&self)->bool{//here we check if a card is special or not 
       if self.current_card.get_value() != "SKIP🚫" && 
          self.current_card.get_value() != "REVERSE🔄" &&
          self.current_card.get_value() != "+2" &&
          self.current_card.get_value() != "COLOR-CHANGE🎨" &&
          self.current_card.get_value() != "+4🌈" {return false;}
        true  
    }
    fn is_card_special(&self)->bool{//here we check if a card is special or not 
       if self.current_card.get_value() != "COLOR-CHANGE🎨" &&
          self.current_card.get_value() != "+4🌈"&&
          self.current_card.get_value() != "+2" {return false;}
        true  
    }
    fn show_table(&self){
        let len = self.players.len();
        for i in 0..len {
            self.players[i].clone().get_player_cards();
        }
    
    }
}



fn main() {
    println!("-------------------------------------------------------------------------");
    println!(
               r#"
      ___
     /\__\         
    /:/  /
   /:/  / 
  /:/  /  
 /:/__/   
 \:\  \   
  \:\  \  
   \:\  \ 
    \:\__\
     \/__/
            "#);
   println!("-------------------------------------------------------------------------");
   /////////////////////////////////////////////////////////////////////////////////////
   let time =Local::now();//time
   let mut table=Table{
                              players: Vec::new(),table_cards: Vec::new(),
                              trash_card: Vec::new(),current_card: Card{value:"".to_string(),color:"".to_string()},
                              winners: Vec::new(),current_pos_player:-1,
                              order_of_play:"pair".to_string(),counter:0,color_chosen:"".to_string()
                            };       //table
                            
   /////////////////////////////////////////////////////////////////////////////////////
   println!("====================================>{}",time);
   println!("-------------------------------------------------------------------------");
   println!(" Welcome to the UNO-GAME ");
   println!(" _______________________");
   println!("");
   /////////////////////////////////////////////////////////////////////////////////////--->
   table.create_players(table.input_players("Please choose the number of players ",2,11,"table"));//we create players 
   table.set_table_cards(table.get_cards());//we set cards to table 
   ///////////////////////////////////////////////////////////////////////////////////// ---> distr cards  to players 
    for i in 0..table.players.len() {
        for _ in 0..7 {
            if let Some(card) = table.draw_card() {
                 table.players[i].add_new_card_player(card);
                 println!("Card added successfully for 🧑 Player {}", i + 1);
            }
        } 
   }
   println!("=======================================>SIZE OF TABLE CARDS : {} cards {}",table.table_cards.len(),"🃏");
   ///////////////////////////////////////////////////////////////////////////////////// --> set the current card 
   table.set_current_card(table.table_cards[table.table_cards.len() - 1].clone(), &Person { position_player: 254, cards: Vec::new()}, "table");
   //////////////////////////////////////////////////////////////////////////////////// ---> game
   while !table.players.is_empty() {
         println!("-------------------------------------------------------------------------");
         println!("=======================================>SIZE OF TABLE CARDS : {} cards {}",table.table_cards.len(),"🃏");
         let mut i=table.get_next_pos() as usize;
         let mut player=table.players[table.get_next_pos() as usize].clone();//we get the person who is going to play 
         println!("🧑 Player {} it's your turn ",player.get_position_player());
         player.get_player_cards();//here we get the players cards 
         if table.is_card_special(){//if the current card is special 
             println!("Not yet implemented ");
             break;
         }else{//if not 
             println!("Not yet implemented ");
             break;
         }
   }
}

