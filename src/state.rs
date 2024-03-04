use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameState {
    pub host:[u8;32],//
    pub waiting:u8,
    pub initialized:u8,
    pub gameseed:String,
    pub lamports:u64,
    pub initializer: [u8;32],
    pub gamehash: [u8;32],
    pub guest: [u8;32],
    pub whoseturn:u8,
    pub guest_move:u8,
    pub score_i:u8,
    pub score_g:u8,
    pub game_ends:u8,
    pub lastplaytime:u64,
    pub chat_line_1:String,
    pub chat_line_2:String,
    pub chat_line_3:String,
    pub chat_line_4:String,
    pub chat_line_5:String,
    pub chat_line_6:String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TGameState {
    pub game_id:String,//
    pub initialized:u8,
    pub gameseed:String,
    pub lamports:u64,
    pub initializer: [u8;32],
    pub gamehash: [u8;32],
    pub guest: [u8;32],
    pub whoseturn:u8,
    pub guest_move:u8,
    pub score_i:u8,
    pub score_g:u8,
    pub game_ends:u8,
    pub lastplaytime:u64,
    pub chat_line_1:String,
    pub chat_line_2:String,
    pub chat_line_3:String,
    pub chat_line_4:String,
    pub chat_line_5:String,
    pub chat_line_6:String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Init{
    pub bump:u8,
    pub game_ends:u8,
    pub gameseed:String,
    pub lamports:u64,
    pub game_hash:[u8;32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Join{
    pub mymove:u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitializerPlay{
    pub last_round_seed:String,
    pub lastmove:u8,
    pub new_game_hash:[u8;32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct UpdateRent {
    pub is_init:u8,
    pub rent:u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Tournament{
    pub is_init:u8,
    pub tournament_id:String,
    pub entrance_fee:u64,
    pub tournament_size:u8,
    pub rent:u64,
    pub starts_at:u64,
    pub time_is_up:u64,
    pub lvl_get:[u8;30],
    pub winner_get:u64,
    pub host_get:u8,
    pub number_of_counters:u8, //Number of each counterfinter has counters(each counterfinder should have equal number of counters)total number of counters = numbercounterfinders*numberofcounters
    pub number_of_counterfinders:u8,
    pub finderfinder:u8,
    pub game_ends:u8,
    pub bump:u8
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct TournamentAccount{
    pub player_find:String,//player_no +tournament_id
    pub tournamentid:String,
    pub player:[u8;32],
    pub opponent:u32,
    pub level:u8,
    pub playerno_int:u32,
    pub opponent_played_on:u64,
    pub is_playing:u8,
    pub waiting_opponent_to_join:u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct TournamentCounter{
    pub empty_tournament_id:String,
    pub counter_no:u16,
    pub player_participating:u32,
    pub capacity:u32,//131072 players for maximum capacity
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)] //update 32 different counters
pub struct CounterFinder{
    pub finder_no:u8,
    pub counters:[u8;256],
    pub tournament_id:String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)] //update once
pub struct FinderFinder{
    pub counters:[u8;32],
    pub tournament_id:String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct InitTournamentCounter{
    pub counter_no:u16,
    pub capacity:u32,
    pub tournament_id:String,
    pub bump:u8,
    pub rent:u64
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Chat{
    pub chat:String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ChatGlobal{
    pub chat_line_1:String,
    pub chat_line_2:String,
    pub chat_line_3:String,
    pub chat_line_4:String,
    pub chat_line_5:String,
    pub chat_line_6:String,
    pub chat_line_7:String,
    pub chat_line_8:String,
    pub chat_line_9:String,
    pub chat_line_10:String,
    pub chat_line_11:String,
    pub chat_line_12:String,
    pub chat_line_13:String,
    pub chat_line_14:String,
    pub chat_line_15:String,
    pub chat_line_16:String,
    pub chat_line_17:String,
    pub chat_line_18:String,
    pub chat_line_19:String,
    pub chat_line_20:String,
}

