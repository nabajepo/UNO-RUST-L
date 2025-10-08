
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
    uno_speech:bool,//if a player said uno (True) if not (False) after the program noticed he has 1 card left 
}
impl Person{
    fn get_position_player(&self) ->u8{ 
        self.position_player
    }
    fn add_new_card_player(&mut self,new_card:Card){
        self.cards.push(new_card);
        println!("Card added successfully for 🧑 Player {}", self.position_player);
    }
    fn show_player_cards(&self) {
        println!("===> {} Player {} these are your cards : {:?} ","🧑",self.get_position_player(),&self.cards);
    }
    fn remove_card(&mut self,index:usize){
        println!("The card {:?} has been remove successfully for {} Player {}",self.cards[index].get_card_info(),
                                                                               "🧑",self.get_position_player());
        self.cards.remove(index);
        println!("You have {} cards {} left ",self.cards.len(),"🃏");                                                                       
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
    prev_pos_player:i8,//the position of the previous player
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
            self.players.push(Person { position_player: index, cards: Vec::new(),uno_speech:false});
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
                if (input_number >= end || input_number <start) && (type_id == "table" || type_id == "index_card" || type_id == "index_color"){
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
    fn set_current_card(&mut self,card:Card,type_id:&str){ //here we set a current card
        if type_id == "table" {
           if card.get_value() == "+2"  ||
              card.get_value() == "REVERSE🔄" ||
              card.get_value() == "COLOR-CHANGE🎨" ||
              card.get_value() == "+4🌈" ||
              card.get_value() == "SKIP🚫" {
                self.set_table_cards(self.table_cards.clone());
                self.set_current_card(self.table_cards[self.table_cards.len() - 1].clone(), "table");  
            }else{
               self.current_card = card.clone(); 
               self.table_cards.pop();
               println!("===> Table has set {:?} as current card {} {}",card.get_card_info(),"🃏","✅");
            }  
             
        }else {
           self.trash_card.push(self.current_card.clone());
           self.current_card = card.clone(); 
           println!(" ===> The  current card {} is {:?} {}","🃏",card,"✅");
        }
    }
    fn get_next_pos(&mut self) ->u8{ //here we are looking for the next pos of player
        if self.current_pos_player == 0 && self.order_of_play == "odd".to_string(){
            self.prev_pos_player=self.current_pos_player;
            self.current_pos_player=(self.players.len() - 1) as i8;
            (self.current_pos_player) as u8
        }else if self.current_pos_player == ((self.players.len() - 1) as i8) && 
                 self.order_of_play == "pair".to_string(){
            self.prev_pos_player=self.current_pos_player;
            self.current_pos_player=0;
            (self.current_pos_player) as u8        
        }else if self.order_of_play == "odd".to_string(){//impair
            self.prev_pos_player=self.current_pos_player;
            self.current_pos_player=self.current_pos_player - 1;
            (self.current_pos_player) as u8
        }else{//pair
            self.prev_pos_player=self.current_pos_player;
            self.current_pos_player=self.current_pos_player + 1;
            (self.current_pos_player) as u8
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
    fn is_card_special(&self)->bool{//here we check if a card is special or not 
       if self.current_card.get_value() != "COLOR-CHANGE🎨" &&
          self.current_card.get_value() != "+4🌈"&&
          self.current_card.get_value() != "+2" {return false;}
        true  
    }
    fn handle_skip(&mut self){//for the card skip
       println!("{} Player {} is going to be skipped {}","🧑",self.get_next_pos(),"🔄");
    } 
    fn handle_reverse(&mut self){//for the card reverse
       if self.order_of_play == "pair".to_string(){ //if the order is pair
          self.order_of_play="odd".to_string();
          println!("The order of the play is now odd {}","⬅️");
       }else{//if the order is odd
          self.order_of_play="pair".to_string();
          println!("The order of the play is now pair {}","➡️");
       }
    }
    fn handle_plus_two(&mut self){//for the card +2
        self.counter=self.counter+2;
        println!("The next player will have to draw {} cards {}",self.counter,"🃏");
    }    
    fn handle_color(&mut self){//for the color card
        let colors=vec!["🔴","🟡","🟢","🔵"];
        println!("Here are the available colors => {:?} ",colors);
        let index_colors=self.input_players("Please give the index of the color chosen ", 0, 4, "index_color") as usize;
        self.color_chosen=colors[ index_colors].to_string();
        println!("The chosen color is  => {} ",self.color_chosen);
    }
    fn handle_plus_four(&mut self){//for the card +4
        self.counter=self.counter+4;
        self.handle_color();
    }
    fn add_new_winners(&mut self,index:u8){ //here we add in order winners 
        self.winners.push(self.players[index as usize].clone());
        println!("{} Player {} is in position {} of winners list {}","🧑",self.players[index as usize].get_position_player(),self.winners.len(),"🎊");
        self.players.remove(index as usize);
    }
    fn show_ranking(&self){// we display winners
        println!("\n|___________________RANKING-PLAYERS___________________|");
        let mut index=0;
        while index < self.winners.len() && !self.winners.is_empty(){
            if index == 0 {//first
                println!("In positon {} we have => Player {} {}",index+1,self.winners[index].get_position_player(),"🏆");
            }else if index == self.winners.len() - 1 {//last
                println!("In positon {} we have => Player {} {}",index+1,self.winners[index].get_position_player(),"👏");
            }else{//middle
                println!("In positon {} we have => Player {} {}",index+1,self.winners[index].get_position_player(),"🎉");
            } 
            index+=1;
        }
        println!("|_____________________________________________________|");
    }
    fn check_table(&self,index:usize){
        for i in 0..self.players.len(){
            if (i != index ) && self.players[i].uno_speech {
                println!("===> {} {} The player {} said UNO ","⚠️","🧑",i);
            }
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
   //------------------------------------------------------------------------------------>time and table
   let time =Local::now();//time
   let mut table=Table{
                              players: Vec::new(),table_cards: Vec::new(),
                              trash_card: Vec::new(),current_card: Card{value:"".to_string(),color:"".to_string()},
                              winners: Vec::new(),current_pos_player:-1,prev_pos_player:-2,
                              order_of_play:"pair".to_string(),counter:0,color_chosen:"".to_string()
                            };       //table
                            
   //-------------------------------------------------------------------------------------->start game
   println!("====================================>{}",time);
   println!("-------------------------------------------------------------------------");
   println!("-------------------------------START-GAME--------------------------------");
   println!(" Welcome to the UNO-GAME ");
   println!(" _______________________");
   println!("");
   //-------------------------------------------------------------------------------------->create players and define cards
   table.create_players(table.input_players("Please choose the number of players ",2,11,"table") as u8);//we create players 
   table.set_table_cards(table.get_cards());//we set cards to table 
   //--------------------------------------------------------------------------------------> distr cards  to players 
    for i in 0..table.players.len() {
        for _ in 0..7 {
            if let Some(card) = table.draw_card() {
                 table.players[i].add_new_card_player(card);
            }
        } 
   }
   println!("=======================================>SIZE OF TABLE CARDS : {} cards {}",table.table_cards.len(),"🃏");
   println!("=======================================>SIZE OF TRASH CARDS : {} cards {}",table.trash_card.len(),"🃏");
   println!("=======================================>SIZE OF PLAYERS : {} players {}",table.players.len(),"🧑");
   println!("=======================================>SIZE OF WINNERS : {} winners {}",table.winners.len(),"🎉");
   //-------------------------------------------------------------------------------------> set the current card 
   table.set_current_card(table.table_cards[table.table_cards.len() - 1].clone(), "table");
   //-------------------------------------------------------------------------------------> game
   while table.players.len() >=2 {
         //if a player did not make a mistake during his turn 
         if table.current_pos_player != table.prev_pos_player{
           println!("-------------------------------------------------------------------------");
           println!("=======================================>SIZE OF TABLE CARDS : {} cards {}",table.table_cards.len(),"🃏");
           println!("=======================================>SIZE OF TRASH CARDS : {} cards {}",table.trash_card.len(),"🃏");
           println!("=======================================>SIZE OF PLAYERS : {} players {}",table.players.len(),"🧑");
           println!("=======================================>SIZE OF WINNERS : {} winners {}",table.winners.len(),"🎉");
         }
         let index_player=table.get_next_pos() as usize;
         println!("===> {} Player {} it's your turn. You have {} cards  {}","🧑",table.players[index_player].get_position_player(),table.players[index_player].cards.len(),"🃏");
         //check if a player said UNO 
         table.check_table(index_player);
         //here we get the players cards
         table.players[index_player].show_player_cards(); 
         //----------> Current card
         let current_card=table.current_card.clone();
         println!("===> The current card is {:?} {}",current_card,"🃏");
         if table.is_card_special(){//if the current card is special 
             println!("Not yet implemented ");
             break;
         }else{//if not 
             //----------> action
             let  choice=table.input_players("Please make your choice : 1.For drawing a card || 2.For playing a card ", 1, 2, "choice_action");
             if choice == 1 {//to draw a card 
                if let Some(card) = table.draw_card() {
                    table.players[index_player].add_new_card_player(card);
                    println!("===> Now you have {} cards {}",table.players[index_player].cards.len(),"🃏");
                    table.players[index_player].show_player_cards();
                }
                if table.players[index_player].uno_speech { //if the player said UNO
                    table.players[index_player].uno_speech = false;
                    println!("{} Now you are not  in UNO session ","⚠️");
                }
             } else{ //to play a card 
                //---------> index card 
                let index_card=table.input_players("Please give the index of the card to play ", 0, table.players[index_player].cards.len() as u8, "index_card");
                println!("You chose the card => {:?} {}",table.players[index_player].cards[index_card as usize],"🃏");
                //--------->check if the card chose is suitable
                if table.is_card_fit(table.players[index_player].cards[index_card as usize].clone()){ //if card is suitable 
                   println!("The card {:?} {} is suitable {}",table.players[index_player].cards[index_card as usize],"🃏","✅"); 
                   //save the card
                   let suitable_card=table.players[index_player].cards[index_card as usize].clone();
                   //remove the card from the player cards
                   table.players[index_player].remove_card(index_card as usize);
                   //check the card
                   if suitable_card.get_value() == "SKIP🚫"{
                      println!("The card {:?} is a special card {}",suitable_card,"🃏");
                      table.handle_skip();
                   }else if suitable_card.get_value() == "REVERSE🔄"{
                      println!("The card {:?} is a special card {}",suitable_card,"🃏");
                      table.handle_reverse(); 
                   }else if suitable_card.get_value() == "+2"{
                      println!("The card {:?} is a special card {}",suitable_card,"🃏");
                      table.handle_plus_two();
                   }else if suitable_card.get_value() == "COLOR-CHANGE🎨"{
                      println!("The card {:?} is a special card {}",suitable_card,"🃏");
                      table.handle_color();
                   }else if suitable_card.get_value() == "+4🌈"{
                      println!("The card {:?} is a special card {}",suitable_card,"🃏");
                      table.handle_plus_four();
                   }else{
                      println!("The card {:?} is a normal card {}",suitable_card,"🃏");
                   }
                   //set a new current card
                   table.set_current_card(suitable_card, "player");
                   //check if a student has 1 card left
                   if table.players[index_player].cards.len() == 1{// if you have a card left
                      println!("{} Dear {} Player {} you have 1 card left. If you don't say UNO you will have to draw 2 more cards","⚠️","🧑",table.players[index_player].get_position_player());
                      let choice_uno=table.input_players("Do you want to say UNO : 1.Yes || 2.No", 1, 2, "choice_action");
                      if choice_uno == 1 {//if yes
                         table.players[index_player].uno_speech= true;
                         println!("The program will inform other players  {}","✅");
                         println!("===> Now you have {} card {} left",table.players[index_player].cards.len(),"🃏");
                         table.players[index_player].show_player_cards();
                      } else{//if no
                         for _ in 0..2 {
                             if let Some(card) = table.draw_card() {
                                table.players[index_player].add_new_card_player(card);
                             }
                         }  
                         println!("===> Now you have {} cards {}",table.players[index_player].cards.len(),"🃏");
                         table.players[index_player].show_player_cards(); 
                      }
                   }else if table.players[index_player].cards.len() == 0 {//if no more cards 
                       println!("=========> YOU WON {}<=========","🥇");
                       table.add_new_winners(index_player as u8);
                       table.players[index_player].show_player_cards();
                   }else{//if you still have cards 
                       println!("===> Now you have {} cards {}",table.players[index_player].cards.len(),"🃏");
                       table.players[index_player].show_player_cards();
                   }
                }else{//if not the player play an another round  
                    table.current_pos_player=table.prev_pos_player;
                    println!("{} Dear {} Player {} the card you chose {:?} is not suitable for the current card {:?}","❌","🧑",table.players[index_player].get_position_player(),table.players[index_player].cards[index_card as usize],current_card);
                    println!("Please try again {}","♻️");
                }
             }
         }
   }
   //we add the last players 
   table.add_new_winners(0);
   //show ranking
   table.show_ranking();
   //END-GAME
   println!("-------------------------------------------------------------------------");
   println!("====================================>{}",time);
   println!("-------------------------------END-GAME----------------------------------");
}
//-------------------------------------------------------------------------------------------------------------------------> I am L
  