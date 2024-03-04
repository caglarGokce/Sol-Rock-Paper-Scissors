use crate::instruction::GameInstruction;
use crate::state::{Chat, ChatGlobal, CounterFinder, FinderFinder, GameState, Init, InitTournamentCounter, InitializerPlay, Join, TGameState, Tournament, TournamentAccount, TournamentCounter, UpdateRent
};

use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{clock::Clock, Sysvar,},
  keccak,
  program::invoke_signed,
  system_instruction
};


pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = GameInstruction::unpack(instruction_data)?;

    match instruction {
      GameInstruction::InitGame {init} => {
        Self::init_game(accounts,init, program_id)
      }
      GameInstruction::JoinGame { join } => {
        Self::join_the_game(accounts,program_id,join)
      }
      GameInstruction::IPlay { play } => {
        Self::initializer_play(accounts,program_id,play)
      }
      GameInstruction::GPlay { play } => {
        Self::guest_play(accounts,program_id,play)
      }
      GameInstruction::ClaimVictory  => {
        Self::claim_victory_over_time(accounts,program_id)
      }
      GameInstruction::Abort => {
        Self::abort_game(accounts,program_id)
      }
      GameInstruction::InitTournament {t}=> {
        Self::init_tournament(accounts, t, program_id)
      }
      GameInstruction::JoinTournament {init} => {
        Self::join_tournament(accounts, program_id,init)
      }
      GameInstruction::InitTournamentMatch {init} => {
        Self::tournament_match_initialize(accounts, program_id, init)
      }
      GameInstruction::TournamentMatchAccept {join} => {
        Self::tournament_match_accept(accounts, program_id, join)
      }
      GameInstruction::TournamentInPlay {play} => {
        Self::tournament_initializer_play(program_id, accounts, play)
      }
      GameInstruction::TournamentGuPlay {play} => {
        Self::tournament_guest_play(program_id, accounts, play)
      }
      GameInstruction::EliminateForIn  => {
        Self::eliminate_player_who_hasnt_initialized_his_game(accounts, program_id)
      }
      GameInstruction::EliminateForMv  => {
        Self::eliminate_player_who_hasnt_made_his_move(accounts, program_id)
      }
      GameInstruction::InitCounter {t_counter} => {
        Self::initialize_counter(accounts, t_counter)
      }
      GameInstruction::UpdateCounterFinder {c_finder} => {
        Self::update_counter_finder(accounts, c_finder)
      }
      GameInstruction::UpdateFinderFinder {f_finder} => {
        Self::update_finder_finder(accounts, f_finder)
      }
      GameInstruction::CloseAccount  => {
        Self::close_account(accounts)
      }
      GameInstruction::ChatGlobal {chat} => {
        Self::chat_global(accounts, chat)
      }
      GameInstruction::ChatLocal {chat} => {
        Self::chat_local(accounts, chat)
      }
      GameInstruction::UpRent { r } => {
        Self::update_rent(accounts,r)
      }
      GameInstruction::WinnerClaim  => {
        Self::winner_claim_prize(accounts, program_id)
      }
    }
  }

  fn init_game(
    accounts: &[AccountInfo],
    init: Init,
    program_id:&Pubkey) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let host: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let rent_data: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let game_state_check: Pubkey = Pubkey::create_with_seed(initializer.key, &init.gameseed, program_id).unwrap();


    if game_state.key != &game_state_check{panic!()}
    if game_state.owner != program_id{panic!()}


    let rents: UpdateRent = UpdateRent::try_from_slice(&rent_data.data.borrow())?;

    if rents.is_init != 1 {panic!()}
    if rent_data.owner != program_id {panic!()}

    if game_state.data.borrow()[0] != 0 {panic!()}
    if init.gameseed.len() != 10 {panic!()}
    if init.game_ends > 5 {panic!()}
    if init.game_ends < 1 {panic!()}
    if init.game_ends == 2 {panic!()}
    if init.game_ends == 4 {panic!()}

    let state: GameState = GameState{
    host:host.key.to_bytes(),
    waiting:1,
    initialized : 1,
    gameseed : init.gameseed,
    lamports:init.lamports,
    initializer:initializer.key.to_bytes(),

    gamehash:init.game_hash,
    guest: [0;32],

    whoseturn:0,
    guest_move:0,
    score_i:0,
    score_g:0,
    game_ends : init.game_ends,
    lastplaytime : 0,
    chat_line_1: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    chat_line_2: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    chat_line_3: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    chat_line_4: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    chat_line_5: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    chat_line_6: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    };


    if **game_state.lamports.borrow() < init.lamports + rents.rent {panic!()}

    state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;

    Ok(())
  }
  fn join_the_game(
    accounts: &[AccountInfo], 
    program_id: &Pubkey,
    join: Join ) -> ProgramResult {

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let temp_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let host: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if game_state.owner != program_id{panic!()}

    let mut state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;


    let hoast_address: Pubkey = Pubkey::new_from_array(state.host);

    if &hoast_address != host.key{panic!()}

    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;


    state.guest = guest.key.to_bytes();
    state.waiting = 2;
    state.lastplaytime = current_time;
    state.whoseturn = 1;
    state.initialized = 2;
    state.guest_move = join.mymove;

    **temp_account.lamports.borrow_mut()-= state.lamports;
    **game_state.lamports.borrow_mut()+= state.lamports;

    state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;

    Ok(())
  }
  fn initializer_play(
    accounts: &[AccountInfo], 
    program_id: &Pubkey,
    play: InitializerPlay ) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let host: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if game_state.owner != program_id{panic!()}


    let mut state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;


    let host_address: Pubkey = Pubkey::new_from_array(host.key.to_bytes());
    let initializer_address: Pubkey = Pubkey::new_from_array(initializer.key.to_bytes());
    let guest_address: Pubkey = Pubkey::new_from_array(guest.key.to_bytes());


    let game_state_check: Pubkey = Pubkey::create_with_seed(initializer.key, &state.gameseed, program_id).unwrap();
    let last_game_hash: keccak::Hash = keccak::hashv(&[&play.last_round_seed.to_string().as_bytes(),play.lastmove.to_string().as_ref(),&play.last_round_seed.to_string().as_bytes()]);



    if initializer.key != &initializer_address{panic!()}
    if game_state.key != &game_state_check{panic!()}
    if guest.key != &guest_address{panic!()}
    if host.key != &host_address{panic!()}
    if state.initialized != 2 {panic!()}
    if state.whoseturn != 1 {panic!()}
    if state.gamehash != last_game_hash.0{panic!()}

    if !initializer.is_signer {panic!()}


    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;


    state.lastplaytime = current_time;
    state.whoseturn = 2;
    state.gamehash = play.new_game_hash;

    
    let mut iwins:bool=false;
    let mut gwins:bool=false;

    if state.guest_move == 1 {

      if play.lastmove == 2 {state.score_g += 1}
      if play.lastmove == 3 {state.score_i += 1}
    }
    if state.guest_move == 2 {

      if play.lastmove == 3 {state.score_g += 1}
      if play.lastmove == 1 {state.score_i += 1}
    }
    if state.guest_move == 3 {

      if play.lastmove == 1 {state.score_g += 1}
      if play.lastmove == 2 {state.score_i += 1}
    }

    state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;


    if state.game_ends == state.score_g{
      gwins = true;
    }
    if state.game_ends == state.score_i{
      iwins = true;
    }

    if iwins == true{

      let host_fee: u64 = state.lamports/50;

      **game_state.lamports.borrow_mut()-= host_fee;
      **host.lamports.borrow_mut()+= host_fee;

      let value: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= value;
      **initializer.lamports.borrow_mut()+= value;

    }
    if gwins == true{
      let host_fee: u64 = state.lamports/50;

      **game_state.lamports.borrow_mut()-= host_fee;
      **host.lamports.borrow_mut()+= host_fee;

      let rew: u64 = (&state.lamports*2)-&host_fee;

      **game_state.lamports.borrow_mut()-= rew;
      **guest.lamports.borrow_mut()+= rew;

      let value: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= value;
      **initializer.lamports.borrow_mut()+= value;
    }


    Ok(())
  }
  fn guest_play(
    accounts: &[AccountInfo], 
    program_id: &Pubkey,
    play: Join ) -> ProgramResult {
      

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let host: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if game_state.owner != program_id{panic!()}

    let mut state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;


    let host_address: Pubkey = Pubkey::new_from_array(host.key.to_bytes());
    let initializer_address: Pubkey = Pubkey::new_from_array(initializer.key.to_bytes());
    let guest_address: Pubkey = Pubkey::new_from_array(guest.key.to_bytes());


    let game_state_check: Pubkey = Pubkey::create_with_seed(initializer.key, &state.gameseed, program_id).unwrap();


    if initializer.key != &initializer_address{panic!()}
    if guest.key != &guest_address{panic!()}
    if host.key != &host_address{panic!()}
    if game_state.key != &game_state_check{panic!()}
    if state.initialized != 2 {panic!()}
    if state.whoseturn != 2 {panic!()}

    if !guest.is_signer {panic!()}


    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;

    state.lastplaytime = current_time;
    state.whoseturn = 1;
    state.guest_move = play.mymove;
    

    state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;

    Ok(())
  }
  fn claim_victory_over_time(        
    accounts: &[AccountInfo],
    program_id: &Pubkey,) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let host: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;

    let host_address: Pubkey = Pubkey::new_from_array(host.key.to_bytes());
    let initializer_address: Pubkey = Pubkey::new_from_array(initializer.key.to_bytes());
    let guest_address: Pubkey = Pubkey::new_from_array(guest.key.to_bytes());

    if host.key != &host_address{panic!()}
    if guest.key != &guest_address{panic!()}
    if initializer.key != &initializer_address{panic!()}

    let game_state_check: Pubkey = Pubkey::create_with_seed(initializer.key, &state.gameseed, program_id).unwrap();

    if game_state.key != &game_state_check{panic!()}
    if state.initialized != 2 {panic!()}

    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;

    let time_passed: u64 = &current_time - &state.lastplaytime;

    if time_passed<120{panic!()}

    let mut gwins:bool=false;
    let mut iwins:bool=false;

    if state.whoseturn == 1{
      gwins = true;
    }
    if state.whoseturn == 2{
      iwins = true;
    }

    if iwins == true{

      let host_fee: u64 = state.lamports/100;

      **game_state.lamports.borrow_mut()-= host_fee;
      **host.lamports.borrow_mut()+= host_fee;

      let value: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= value;
      **initializer.lamports.borrow_mut()+= value;

    }

    if gwins == true{
      let host_fee: u64 = state.lamports/100;

      **game_state.lamports.borrow_mut()-= host_fee;
      **host.lamports.borrow_mut()+= host_fee;

      let rew: u64 = (&state.lamports*2)-&host_fee;

      **game_state.lamports.borrow_mut()-= rew;
      **guest.lamports.borrow_mut()+= rew;

      let value: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= value;
      **initializer.lamports.borrow_mut()+= value;
    }

    Ok(())
  }
  fn abort_game(        
    accounts: &[AccountInfo],
    program_id: &Pubkey,) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;


  
    let initializer_address: Pubkey = Pubkey::new_from_array(initializer.key.to_bytes());


    if initializer.key != &initializer_address{panic!()}


    let game_state_check: Pubkey = Pubkey::create_with_seed(initializer.key, &state.gameseed, program_id).unwrap();


    if game_state.key != &game_state_check{panic!()}
    if state.initialized != 1 {panic!()}

    if !initializer.is_signer {panic!()}

    let value: u64 = **game_state.lamports.borrow();

    **game_state.lamports.borrow_mut()-= value;
    **initializer.lamports.borrow_mut()+= value;


    Ok(())
  }
  fn update_rent(        
    accounts: &[AccountInfo], 
    r: UpdateRent
    ) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let rent: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      if !authority.is_signer {panic!()}

      let authority_key: Pubkey = Pubkey::from_str("4YbLBRXwseG1NuyJbteSD5u81Q2QjFqJBp6JmxwYBKYm").unwrap();

      if authority.key != &authority_key {panic!()}

      let rent_account: UpdateRent = UpdateRent{is_init:1,rent:r.rent};
  

      rent_account.serialize(&mut &mut rent.data.borrow_mut()[..])?;


    Ok(())
  }
  fn init_tournament(        
    accounts: &[AccountInfo],
    t: Tournament,
    program_id: &Pubkey
    ) -> ProgramResult {


      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
      let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      let authority = Pubkey::from_str("4YbLBRXwseG1NuyJbteSD5u81Q2QjFqJBp6JmxwYBKYm").unwrap();
  
      if initializer.key != &authority {panic!()}
      if !initializer.is_signer{panic!()}

      invoke_signed(
        &system_instruction::create_account( 
            &initializer.key, 
            &tournament.key,
            t.rent,
            87, //312???m
            &program_id
        ),
        &[
          initializer.clone(), 
          tournament.clone(),
        ],
        &[&[t.tournament_id.as_ref(), &[t.bump]]],
      )?;
  
      t.serialize(&mut &mut tournament.data.borrow_mut()[..])?;


    Ok(())
  }
  fn join_tournament(        
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    init: Init) -> ProgramResult {


    let accounts_iter = &mut accounts.iter();

    let participant = next_account_info(accounts_iter)?;
    let tournament_account = next_account_info(accounts_iter)?;
    let tournament = next_account_info(accounts_iter)?;
    let counter = next_account_info(accounts_iter)?;

    let t = Tournament::try_from_slice(&tournament.data.borrow())?;
    let mut t_counter = TournamentCounter::try_from_slice(&counter.data.borrow())?;

    let value = **tournament_account.lamports.borrow();

    if value < t.entrance_fee{panic!()}
    if tournament.is_writable{panic!()}
    if tournament.owner != program_id{panic!()}
    if t.is_init != 1 {panic!()}
    if tournament_account.owner != program_id{panic!()}
    if t_counter.player_participating >= t_counter.capacity{panic!()}


    t_counter.player_participating += 1;

    let mut str_no = String::new();
    let str_tournament_id = t.tournament_id;
    //let str_counter_no = t_counter.counter_no.to_string();
    let player_no = ((t_counter.counter_no-1)*t.number_of_counters as u16) as u32+t_counter.player_participating;
    let str_player_no = player_no.to_string();


    let mut somestr = String::from("pppppppppp");
    let len = str_player_no.len();
    somestr.replace_range(somestr.len() - len.., &str_player_no);

    str_no += &somestr.chars().rev().collect::<String>();

    str_no += &str_tournament_id;

    let full = String::from("F");
    let mut empty_tournament_id = String::new();
    empty_tournament_id += &full;
    empty_tournament_id += &str_tournament_id;


    let mut opp = player_no + 1;

    if player_no%2 == 0{
      opp = player_no - 1;
    }

    invoke_signed(
      &system_instruction::create_account( 
          &participant.key, 
          &tournament_account.key,
          t.rent,
          96, //312???m
          &program_id
      ),
      &[
        participant.clone(), 
        tournament_account.clone(),
      ],
      &[&[str_no.as_ref(), &[init.bump]]],
    )?;

    let t_account = TournamentAccount{
      player_find:str_no,
      tournamentid:str_tournament_id,
      player:participant.key.to_bytes(),
      opponent:opp,
      level:0,
      playerno_int:player_no,
      opponent_played_on:t.starts_at,
      is_playing:0,
      waiting_opponent_to_join:0
    };

    if t_counter.player_participating == t_counter.capacity{
      t_counter.empty_tournament_id = empty_tournament_id;
    }

    t_account.serialize(&mut &mut tournament_account.data.borrow_mut()[..])?;
    t_counter.serialize(&mut &mut counter.data.borrow_mut()[..])?;

    Ok(())
  }
  fn tournament_match_initialize(        
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    init: Init) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let initializer_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

    let mut t_account: TournamentAccount = TournamentAccount::try_from_slice(&initializer_tour_acc.data.borrow())?;
 
    
    let mut substract:bool = false;
    let initializer_no: u32 = t_account.playerno_int;
    let pwr: u32 = t_account.level as u32;
    let mut pwrplus: u32 = t_account.level as u32;
    pwrplus += 1;
    let pwoftwo:u32 = 2;
    let divisibleby: &u32 = &pwoftwo.pow(pwrplus);
    if initializer_no%divisibleby == 0 {
      substract = true;
    }
    let mut opponent_no: u32 = 0;
    if substract == false {
      opponent_no = &initializer_no + &pwoftwo.pow(pwr);
    }
    if substract == true {
      opponent_no = &initializer_no - &pwoftwo.pow(pwr);
    }
    let mut game_seed: String = String::new();
    let opponent_no_str: &String = &opponent_no.to_string();
    let initializer_no_str: &String = &initializer_no.to_string();
    let seed: String = String::from("v");
    if opponent_no > initializer_no{
      game_seed += initializer_no_str;
      game_seed += &seed;
      game_seed += opponent_no_str;
    }
    if opponent_no < initializer_no{
      game_seed += opponent_no_str;
      game_seed += &seed;
      game_seed += initializer_no_str;
    }
    
    let mut game_id: String = String::new();

    game_id += &t.tournament_id;
    game_id += &game_seed;


    invoke_signed(
      &system_instruction::create_account( 
          &initializer.key, 
          &game_state.key,
          t.rent,
          557, //312???m
          &program_id
      ),
      &[
        initializer.clone(), 
        game_state.clone(),
      ],
      &[&[game_id.as_ref(), &[init.bump]]],
    )?;

    let initializer_address = Pubkey::new_from_array(initializer.key.to_bytes());



    if initializer.key != &initializer_address{panic!()}


    if !initializer.is_signer{panic!()}
    if t.is_init != 1{panic!()}
    if t.tournament_id != t_account.tournamentid{panic!()}
    if tournament.owner != program_id{panic!()}
    if game_state.owner != program_id{panic!()}
    if initializer_tour_acc.owner != program_id{panic!()}
    if tournament.is_writable {panic!()}



    let gamestate: TGameState = TGameState{
      game_id:game_id,
      initialized :3,
      gameseed:"XXXXXXXXXX".to_string(),
      lamports:0,
      initializer: initializer.key.to_bytes(),

      gamehash:init.game_hash,
      guest:[0;32],

      whoseturn:0,
      guest_move:0,
      score_i:0,
      score_g:0,
      game_ends : t.game_ends,
      lastplaytime:0,
      chat_line_1:"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
      chat_line_2:"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
      chat_line_3:"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
      chat_line_4:"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
      chat_line_5:"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
      chat_line_6:"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),

    };

    t_account.opponent = opponent_no;
    t_account.waiting_opponent_to_join = 1;

    gamestate.serialize(&mut &mut game_state.data.borrow_mut()[..])?;
    t_account.serialize(&mut &mut initializer_tour_acc.data.borrow_mut()[..])?;

    Ok(())
  }
  fn tournament_match_accept(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    join: Join) -> ProgramResult {



    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let initializer_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

    let o_t_account: TournamentAccount = TournamentAccount::try_from_slice(&opponent_tour_acc.data.borrow())?;
    let opponent_address: Pubkey = Pubkey::new_from_array(o_t_account.player);

    let t_account: TournamentAccount = TournamentAccount::try_from_slice(&initializer_tour_acc.data.borrow())?;
    let initializer_address: Pubkey = Pubkey::new_from_array(t_account.player);

    let state: TGameState = TGameState::try_from_slice(&game_state.data.borrow())?;


    if !opponent.is_signer{panic!()}
    if t.is_init != 1{panic!()}
    if t.tournament_id != t_account.tournamentid{panic!()}
    if t.tournament_id != o_t_account.tournamentid{panic!()}
    if o_t_account.level == t_account.level {panic!()}
    if o_t_account.playerno_int == t_account.opponent {panic!()}
    if opponent.key != &opponent_address{panic!()}
    if initializer.key != &initializer_address{panic!()}
    if state.initializer != t_account.player {panic!()}
    if tournament.owner != program_id{panic!()}
    if opponent_tour_acc.owner != program_id{panic!()}
    if initializer_tour_acc.owner != program_id{panic!()}
    if game_state.owner != program_id{panic!()}
    if state.initialized != 3 {panic!()}
    if tournament.is_writable {panic!()}


    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;

    let gamestate: TGameState = TGameState{
      game_id:state.game_id,
      initialized:4,
      gameseed:state.gameseed,
      lamports:state.lamports,
      initializer: state.initializer,
      gamehash: state.gamehash,
      guest: opponent.key.to_bytes(),
      whoseturn:1,
      guest_move:join.mymove,
      score_i:state.score_i,
      score_g:state.score_g,
      game_ends:state.game_ends,
      lastplaytime:state.lastplaytime,
      chat_line_1: state.chat_line_1,
      chat_line_2: state.chat_line_2,
      chat_line_3: state.chat_line_3,
      chat_line_4: state.chat_line_4,
      chat_line_5: state.chat_line_5,
      chat_line_6: state.chat_line_6,
    };


    let opponent_tounament_account: TournamentAccount = TournamentAccount{
      player_find:o_t_account.player_find,
      tournamentid:o_t_account.tournamentid,
      player:o_t_account.player,
      opponent:o_t_account.opponent,
      level:o_t_account.level,
      playerno_int:o_t_account.playerno_int,
      opponent_played_on:o_t_account.opponent_played_on,
      is_playing:1,
      waiting_opponent_to_join:o_t_account.waiting_opponent_to_join,
    };

    let initializer_tounament_account: TournamentAccount = TournamentAccount{
      player_find:t_account.player_find,
      tournamentid:t_account.tournamentid,
      player:t_account.player,
      opponent:t_account.opponent,
      level:t_account.level,
      playerno_int:t_account.playerno_int,
      opponent_played_on:current_time,
      is_playing:1,
      waiting_opponent_to_join:0,
    };

    opponent_tounament_account.serialize(&mut &mut opponent_tour_acc.data.borrow_mut()[..])?;
    initializer_tounament_account.serialize(&mut &mut initializer_tour_acc.data.borrow_mut()[..])?;
    gamestate.serialize(&mut &mut game_state.data.borrow_mut()[..])?;

    Ok(())
  }
  //TODO tek sayidaki oyuncular turnuva ilk basladiginda oyunu kurarlar
  fn tournament_initializer_play(        
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    play: InitializerPlay,) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let initializer_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let mut state: TGameState = TGameState::try_from_slice(&game_state.data.borrow())?;

    let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

    let mut opponent_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&opponent_tour_acc.data.borrow())?;
    let mut initializer_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&initializer_tour_acc.data.borrow())?;

    let opponent_address: Pubkey = Pubkey::new_from_array(opponent_tournament_account.player);
    let initializer_address: Pubkey = Pubkey::new_from_array(initializer_tournament_account.player);


    if t.is_init != 1{panic!()}
    if t.tournament_id != initializer_tournament_account.tournamentid{panic!()}
    if t.tournament_id != opponent_tournament_account.tournamentid{panic!()}
    if state.initializer != initializer_tournament_account.player{panic!()}
    if state.guest != opponent_tournament_account.player{panic!()}
    if opponent_tournament_account.level == initializer_tournament_account.level {panic!()}
    if opponent_tournament_account.playerno_int == initializer_tournament_account.opponent {panic!()}
    if opponent_tournament_account.opponent == initializer_tournament_account.playerno_int {panic!()}
    if opponent.key != &opponent_address{panic!()}
    if initializer.key != &initializer_address{panic!()}
    if tournament.owner != program_id{panic!()}
    if opponent_tour_acc.owner != program_id{panic!()}
    if initializer_tour_acc.owner != program_id{panic!()}
    if game_state.owner != program_id{panic!()}
    if tournament.is_writable {panic!()}
    if opponent_tournament_account.is_playing != 1{panic!()}
    if initializer_tournament_account.is_playing != 1{panic!()}


    if state.initialized != 4 {panic!()}
    if play.lastmove > 3 {panic!()}
    if play.lastmove < 1 {panic!()}
    if state.whoseturn != 1 {panic!()}


    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;

    let last_game_hash: keccak::Hash = keccak::hashv(&[&play.last_round_seed.to_string().as_bytes(),play.lastmove.to_string().as_ref(),&play.last_round_seed.to_string().as_bytes()]);


    if last_game_hash.0 != state.gamehash{panic!()}



    state.lastplaytime = current_time;
    state.whoseturn = 2;
    state.gamehash = play.new_game_hash;
    
    let mut iwins:bool=false;
    let mut gwins:bool=false;

    if state.guest_move == 1 {

      if play.lastmove == 2 {state.score_g += 1}
      if play.lastmove == 3 {state.score_i += 1}
    }
    if state.guest_move == 2 {

      if play.lastmove == 3 {state.score_g += 1}
      if play.lastmove == 1 {state.score_i += 1}
    }
    if state.guest_move == 3 {

      if play.lastmove == 1 {state.score_g += 1}
      if play.lastmove == 2 {state.score_i += 1}
    }

    state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;


    if state.game_ends == state.score_g{
      gwins = true;
    }
    if state.game_ends == state.score_i{
      iwins = true;
    }

    let mut the_no = 0;
    if initializer_tournament_account.playerno_int>opponent_tournament_account.playerno_int{
      the_no = initializer_tournament_account.playerno_int;
    }
    if initializer_tournament_account.playerno_int<opponent_tournament_account.playerno_int{
      the_no = opponent_tournament_account.playerno_int;
    }


    if iwins == true{

      let us: usize = opponent_tournament_account.level as usize;
      let multiply: u64 = t.lvl_get[us] as u64; 
      let reward:u64 = multiply*t.entrance_fee;

      **opponent_tour_acc.lamports.borrow_mut()-= reward;
      **opponent.lamports.borrow_mut()+= reward;

      let value: u64 = **opponent_tour_acc.lamports.borrow();

      **opponent_tour_acc.lamports.borrow_mut()-= value;
      **initializer_tour_acc.lamports.borrow_mut()+= value;

      let game: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= game;
      **initializer_tour_acc.lamports.borrow_mut()+= game;

      let str_player_no: String = the_no.to_string();
      let mut somestr: String = String::from("pppppppppp");
      let len: usize = str_player_no.len();
      somestr.replace_range(somestr.len() - len.., &str_player_no);

      let offset2: usize = somestr.len();
      let mut find_me: String = initializer_tournament_account.player_find;
      find_me.replace_range(..offset2,&somestr);

      initializer_tournament_account.player_find = find_me;
      initializer_tournament_account.is_playing = 0;
      initializer_tournament_account.waiting_opponent_to_join = 0;
      initializer_tournament_account.level += 1;
      initializer_tournament_account.playerno_int = the_no;
      initializer_tournament_account.opponent_played_on = current_time;

      initializer_tournament_account.serialize(&mut &mut initializer_tour_acc.data.borrow_mut()[..])?;

    }
    if gwins == true{
      let us: usize = initializer_tournament_account.level as usize;
      let multiply: u64 = t.lvl_get[us] as u64; 
      let reward:u64 = multiply*t.entrance_fee;


      **initializer_tour_acc.lamports.borrow_mut()-= reward;
      **initializer.lamports.borrow_mut()+= reward;

      let value: u64 = **initializer_tour_acc.lamports.borrow();

      **initializer_tour_acc.lamports.borrow_mut()-= value;
      **opponent_tour_acc.lamports.borrow_mut()+= value;

      let game: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= game;
      **initializer_tour_acc.lamports.borrow_mut()+= game;

      let str_player_no: String = the_no.to_string();
      let mut somestr: String = String::from("pppppppppp");
      let len: usize = str_player_no.len();
      somestr.replace_range(somestr.len() - len.., &str_player_no);

      let offset2: usize = somestr.len();
      let mut find_me: String = opponent_tournament_account.player_find;
      find_me.replace_range(..offset2,&somestr);

      opponent_tournament_account.player_find = find_me;
      opponent_tournament_account.is_playing = 0;
      opponent_tournament_account.waiting_opponent_to_join = 0;
      opponent_tournament_account.level += 1;
      opponent_tournament_account.playerno_int = the_no;
      opponent_tournament_account.opponent_played_on = current_time;

      opponent_tournament_account.serialize(&mut &mut opponent_tour_acc.data.borrow_mut()[..])?;

    }


    Ok(())
  }
  fn tournament_guest_play(        
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    play: InitializerPlay,) -> ProgramResult {
    

    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let initializer_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let mut state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;

    let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

    let mut opponent_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&opponent_tour_acc.data.borrow())?;

    let mut initializer_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&initializer_tour_acc.data.borrow())?;

    let opponent_address: Pubkey = Pubkey::new_from_array(opponent_tournament_account.player);
    let initializer_address: Pubkey = Pubkey::new_from_array(initializer_tournament_account.player);

    if t.is_init != 1{panic!()}
    if t.tournament_id != initializer_tournament_account.tournamentid{panic!()}
    if t.tournament_id != opponent_tournament_account.tournamentid{panic!()}
    if state.initializer != initializer_tournament_account.player{panic!()}
    if state.guest != opponent_tournament_account.player{panic!()}
    if opponent_tournament_account.level == initializer_tournament_account.level {panic!()}
    if opponent_tournament_account.playerno_int == initializer_tournament_account.opponent {panic!()}
    if opponent_tournament_account.opponent == initializer_tournament_account.playerno_int {panic!()}
    if opponent.key != &opponent_address{panic!()}
    if initializer.key != &initializer_address{panic!()}
    if tournament.owner != program_id{panic!()}
    if opponent_tour_acc.owner != program_id{panic!()}
    if initializer_tour_acc.owner != program_id{panic!()}
    if game_state.owner != program_id{panic!()}
    if tournament.is_writable {panic!()}
    if opponent_tournament_account.is_playing != 1{panic!()}
    if initializer_tournament_account.is_playing != 1{panic!()}


    if state.initialized != 4 {panic!()}
    if play.lastmove > 3 {panic!()}
    if play.lastmove < 1 {panic!()}
    if state.whoseturn != 2 {panic!()}


    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;


    state.lastplaytime = current_time;
    state.whoseturn = 1;


    let mut iwins:bool=false;
    let mut gwins:bool=false;

    if state.guest_move == 1 {

      if play.lastmove == 2 {state.score_g += 1}
      if play.lastmove == 3 {state.score_i += 1}
    }
    if state.guest_move == 2 {

      if play.lastmove == 3 {state.score_g += 1}
      if play.lastmove == 1 {state.score_i += 1}
    }
    if state.guest_move == 3 {

      if play.lastmove == 1 {state.score_g += 1}
      if play.lastmove == 2 {state.score_i += 1}
    }

    state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;


    if state.game_ends == state.score_g{
      gwins = true;
    }
    if state.game_ends == state.score_i{
      iwins = true;
    }

    let mut the_no: u32 = 0;
    if initializer_tournament_account.playerno_int>opponent_tournament_account.playerno_int{
      the_no = initializer_tournament_account.playerno_int;
    }
    if initializer_tournament_account.playerno_int<opponent_tournament_account.playerno_int{
      the_no = opponent_tournament_account.playerno_int;
    }


    if iwins == true{

      let us: usize = opponent_tournament_account.level as usize;
      let multiply: u64 = t.lvl_get[us] as u64; 
      let reward:u64 = multiply*t.entrance_fee;

      **opponent_tour_acc.lamports.borrow_mut()-= reward;
      **opponent.lamports.borrow_mut()+= reward;

      let value: u64 = **opponent_tour_acc.lamports.borrow();

      **opponent_tour_acc.lamports.borrow_mut()-= value;
      **initializer_tour_acc.lamports.borrow_mut()+= value;

      let game: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= game;
      **initializer_tour_acc.lamports.borrow_mut()+= game;

      let str_player_no: String = the_no.to_string();
      let mut somestr: String = String::from("pppppppppp");
      let len: usize = str_player_no.len();
      somestr.replace_range(somestr.len() - len.., &str_player_no);

      let offset2: usize = somestr.len();
      let mut find_me: String = initializer_tournament_account.player_find;
      find_me.replace_range(..offset2,&somestr);

      initializer_tournament_account.player_find = find_me;
      initializer_tournament_account.is_playing = 0;
      initializer_tournament_account.waiting_opponent_to_join = 0;
      initializer_tournament_account.level += 1;
      initializer_tournament_account.playerno_int = the_no;
      initializer_tournament_account.opponent_played_on = current_time;

      initializer_tournament_account.serialize(&mut &mut initializer_tour_acc.data.borrow_mut()[..])?;

    }
    if gwins == true{
      let us: usize = initializer_tournament_account.level as usize;
      let multiply: u64 = t.lvl_get[us] as u64; 
      let reward:u64 = multiply*t.entrance_fee;


      **initializer_tour_acc.lamports.borrow_mut()-= reward;
      **initializer.lamports.borrow_mut()+= reward;

      let value: u64 = **initializer_tour_acc.lamports.borrow();

      **initializer_tour_acc.lamports.borrow_mut()-= value;
      **opponent_tour_acc.lamports.borrow_mut()+= value;

      let game: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= game;
      **initializer_tour_acc.lamports.borrow_mut()+= game;

      let str_player_no: String = the_no.to_string();
      let mut somestr: String = String::from("pppppppppp");
      let len: usize = str_player_no.len();
      somestr.replace_range(somestr.len() - len.., &str_player_no);

      let offset2: usize = somestr.len();
      let mut find_me: String = opponent_tournament_account.player_find;
      find_me.replace_range(..offset2,&somestr);

      opponent_tournament_account.player_find = find_me;
      opponent_tournament_account.is_playing = 0;
      opponent_tournament_account.waiting_opponent_to_join = 0;
      opponent_tournament_account.level += 1;
      opponent_tournament_account.playerno_int = the_no;
      opponent_tournament_account.opponent_played_on = current_time;

      opponent_tournament_account.serialize(&mut &mut opponent_tour_acc.data.borrow_mut()[..])?;

    }


    Ok(())
  }
  fn eliminate_player_who_hasnt_initialized_his_game(
    accounts: &[AccountInfo],
    program_id: &Pubkey,) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let eliminate: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let eliminate_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    //let host = next_account_info(accounts_iter)?;

    let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

    let el_t_account: TournamentAccount = TournamentAccount::try_from_slice(&eliminate_tour_acc.data.borrow())?;

    let el_check: Pubkey = Pubkey::new_from_array(el_t_account.player);

    let mut opponent_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&opponent_tour_acc.data.borrow())?;

    let opponent_check: Pubkey = Pubkey::new_from_array(opponent_tournament_account.player);


    if t.is_init != 1{panic!()}
    if t.tournament_id != el_t_account.tournamentid{panic!()}
    if t.tournament_id != opponent_tournament_account.tournamentid{panic!()}
    if eliminate.key != &el_check{panic!()}
    if opponent.key != &opponent_check{panic!()}
    if tournament.owner != program_id{panic!()}
    if el_t_account.waiting_opponent_to_join != 0 {panic!()}
    if eliminate_tour_acc.owner != program_id{panic!()}
    if tournament.is_writable {panic!()}
    if el_t_account.is_playing != 0  {panic!()}
    if el_t_account.opponent != opponent_tournament_account.playerno_int{panic!()}
    if el_t_account.playerno_int != opponent_tournament_account.opponent{panic!()}
    if el_t_account.level != opponent_tournament_account.level{panic!()}

    if el_t_account.level == 0{
      if el_t_account.playerno_int % 2 == 0 {panic!()}
    }

    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;
    let time_passed: u64 = current_time - el_t_account.opponent_played_on;

    if time_passed < t.time_is_up {panic!()}

    let mut the_no: u32 = 0;
    if el_t_account.playerno_int>opponent_tournament_account.playerno_int{
      the_no = el_t_account.playerno_int;
    }
    if el_t_account.playerno_int<opponent_tournament_account.playerno_int{
      the_no = opponent_tournament_account.playerno_int;
    }

    let us: usize = el_t_account.level as usize;
    let multiply: u64 = t.lvl_get[us] as u64;
    let reward:u64 = multiply*t.entrance_fee;

    **eliminate_tour_acc.lamports.borrow_mut()-= reward;
    **eliminate.lamports.borrow_mut()+= reward;

    let value: u64 = **eliminate_tour_acc.lamports.borrow();

    **eliminate_tour_acc.lamports.borrow_mut()-= value;
    **opponent_tour_acc.lamports.borrow_mut()+= value;

    let str_player_no: String = the_no.to_string();
    let mut somestr: String = String::from("pppppppppp");
    let len: usize = str_player_no.len();
    somestr.replace_range(somestr.len() - len.., &str_player_no);

    let offset2: usize = somestr.len();
    let mut find_me: String = opponent_tournament_account.player_find;
    find_me.replace_range(..offset2,&somestr);

    opponent_tournament_account.player_find = find_me;
    opponent_tournament_account.is_playing = 0;
    opponent_tournament_account.waiting_opponent_to_join = 0;
    opponent_tournament_account.level += 1;
    opponent_tournament_account.playerno_int = the_no;

    opponent_tournament_account.serialize(&mut &mut opponent_tour_acc.data.borrow_mut()[..])?;


    Ok(())
  }
  fn eliminate_player_who_hasnt_made_his_move(
    accounts: &[AccountInfo],
    program_id: &Pubkey,) -> ProgramResult {


    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let initializer_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let opponent_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    //let host = next_account_info(accounts_iter)?;

    let state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;

    let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

    let mut opponent_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&opponent_tour_acc.data.borrow())?;

    let mut initializer_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&initializer_tour_acc.data.borrow())?;

    let initializer_account_check: Pubkey = Pubkey::new_from_array(initializer_tournament_account.player);
    let opponent_check: Pubkey = Pubkey::new_from_array(opponent_tournament_account.player);



    if t.is_init != 1{panic!()}
    if t.tournament_id != initializer_tournament_account.tournamentid{panic!()}
    if t.tournament_id != opponent_tournament_account.tournamentid{panic!()}
    if state.initializer != initializer_tournament_account.player{panic!()}
    if state.guest != opponent_tournament_account.player{panic!()}
    if opponent_tournament_account.level == initializer_tournament_account.level {panic!()}
    if opponent_tournament_account.opponent == initializer_tournament_account.playerno_int {panic!()}
    if opponent_tournament_account.playerno_int == initializer_tournament_account.opponent {panic!()}
    if opponent.key != &opponent_check{panic!()}
    if initializer.key != &initializer_account_check{panic!()}
    if tournament.owner != program_id{panic!()}
    if opponent_tour_acc.owner != program_id{panic!()}
    if initializer_tour_acc.owner != program_id{panic!()}
    if game_state.owner != program_id{panic!()}
    if tournament.is_writable {panic!()}
    if opponent_tournament_account.is_playing != 1{panic!()}
    if initializer_tournament_account.is_playing != 1{panic!()}
    if state.initialized != 4 {panic!()}


    let mut the_no: u32 = 0;
    if initializer_tournament_account.playerno_int>opponent_tournament_account.playerno_int{
      the_no = initializer_tournament_account.playerno_int;
    }
    if initializer_tournament_account.playerno_int<opponent_tournament_account.playerno_int{
      the_no = opponent_tournament_account.playerno_int;
    }

    let mut iwins:bool=false;
    let mut gwins:bool=false;

    let clock: Clock= Clock::get()?;
    let current_time: u64 = clock.unix_timestamp as u64;


    if state.whoseturn == 1{
      let time_passed: u64 = current_time - state.lastplaytime;
      if time_passed > t.time_is_up {gwins=true;}
      if time_passed < t.time_is_up {panic!()}
    }
    if state.whoseturn == 2{
      let time_passed: u64 = current_time - state.lastplaytime;
      if time_passed > t.time_is_up {iwins=true;}
      if time_passed < t.time_is_up {panic!()}
    }

    if !iwins && !gwins{panic!()}
    if !iwins && gwins{panic!()}

    if iwins == true{

      let us: usize = opponent_tournament_account.level as usize;
      let multiply: u64 = t.lvl_get[us] as u64; 
      let reward:u64 = multiply*t.entrance_fee;

      **opponent_tour_acc.lamports.borrow_mut()-= reward;
      **opponent.lamports.borrow_mut()+= reward;

      let value: u64 = **opponent_tour_acc.lamports.borrow();

      **opponent_tour_acc.lamports.borrow_mut()-= value;
      **initializer_tour_acc.lamports.borrow_mut()+= value;

      let game: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= game;
      **initializer_tour_acc.lamports.borrow_mut()+= game;

      let str_player_no: String = the_no.to_string();
      let mut somestr: String = String::from("pppppppppp");
      let len: usize = str_player_no.len();
      somestr.replace_range(somestr.len() - len.., &str_player_no);

      let offset2: usize = somestr.len();
      let mut find_me: String = initializer_tournament_account.player_find;
      find_me.replace_range(..offset2,&somestr);

      initializer_tournament_account.player_find = find_me;
      initializer_tournament_account.is_playing = 0;
      initializer_tournament_account.waiting_opponent_to_join = 0;
      initializer_tournament_account.level += 1;
      initializer_tournament_account.playerno_int = the_no;
      initializer_tournament_account.opponent_played_on = current_time;

      initializer_tournament_account.serialize(&mut &mut initializer_tour_acc.data.borrow_mut()[..])?;

    }
    if gwins == true{

      let us: usize = initializer_tournament_account.level as usize;
      let multiply: u64 = t.lvl_get[us] as u64; 
      let reward:u64 = multiply*t.entrance_fee;

      **initializer_tour_acc.lamports.borrow_mut()-= reward;
      **initializer.lamports.borrow_mut()+= reward;

      let value: u64 = **initializer_tour_acc.lamports.borrow();

      **initializer_tour_acc.lamports.borrow_mut()-= value;
      **opponent_tour_acc.lamports.borrow_mut()+= value;

      let game: u64 = **game_state.lamports.borrow();

      **game_state.lamports.borrow_mut()-= game;
      **initializer_tour_acc.lamports.borrow_mut()+= game;

      let str_player_no: String = the_no.to_string();
      let mut somestr: String = String::from("pppppppppp");
      let len: usize = str_player_no.len();
      somestr.replace_range(somestr.len() - len.., &str_player_no);

      let offset2: usize = somestr.len();
      let mut find_me: String = opponent_tournament_account.player_find;
      find_me.replace_range(..offset2,&somestr);

      opponent_tournament_account.player_find = find_me;
      opponent_tournament_account.is_playing = 0;
      opponent_tournament_account.waiting_opponent_to_join = 0;
      opponent_tournament_account.level += 1;
      opponent_tournament_account.playerno_int = the_no;
      opponent_tournament_account.opponent_played_on = current_time;

      opponent_tournament_account.serialize(&mut &mut opponent_tour_acc.data.borrow_mut()[..])?;

    }

    Ok(())
  }
  fn initialize_counter(        
    accounts: &[AccountInfo],
    t_counter: InitTournamentCounter) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tournament_counter: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let a_k: Pubkey = Pubkey::from_str("4YbLBRXwseG1NuyJbteSD5u81Q2QjFqJBp6JmxwYBKYm").unwrap();
      if authority.key != &a_k {panic!()}
      if !authority.is_signer {panic!()}

      t_counter.serialize(&mut &mut tournament_counter.data.borrow_mut()[..])?;


      Ok(())
  }
  fn update_counter_finder(        
    accounts: &[AccountInfo],
    c_finder: CounterFinder) -> ProgramResult {
  
        let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
        let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
        let finder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
        let a_k: Pubkey = Pubkey::from_str("4YbLBRXwseG1NuyJbteSD5u81Q2QjFqJBp6JmxwYBKYm").unwrap();
        if authority.key != &a_k {panic!()}
        if !authority.is_signer {panic!()}
  
      let f: CounterFinder = CounterFinder{
        finder_no:c_finder.finder_no,
        counters:c_finder.counters,
        tournament_id:c_finder.tournament_id,
      };

      f.serialize(&mut &mut finder.data.borrow_mut()[..])?;

        Ok(())
  }
  fn update_finder_finder(        
        accounts: &[AccountInfo],
    f_finder: FinderFinder) -> ProgramResult {
    
          let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
          let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
          let finder: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    
          let a_k: Pubkey = Pubkey::from_str("4YbLBRXwseG1NuyJbteSD5u81Q2QjFqJBp6JmxwYBKYm").unwrap();
          if authority.key != &a_k {panic!()}
          if !authority.is_signer {panic!()}
    
        let f: FinderFinder = FinderFinder{
          counters:f_finder.counters,
          tournament_id:f_finder.tournament_id,
        };
  
        f.serialize(&mut &mut finder.data.borrow_mut()[..])?;
    
          Ok(())
  }
  fn close_account(        
    accounts: &[AccountInfo]) -> ProgramResult {

      let accounts_iter = &mut accounts.iter();

      let authority = next_account_info(accounts_iter)?;
      let account = next_account_info(accounts_iter)?;

      let a_k = Pubkey::from_str("4YbLBRXwseG1NuyJbteSD5u81Q2QjFqJBp6JmxwYBKYm").unwrap();
      if authority.key != &a_k {panic!()}
      if !authority.is_signer {panic!()}

      let account_value = **account.lamports.borrow();

      **account.lamports.borrow_mut()-= account_value;
      **authority.lamports.borrow_mut()+= account_value;

      Ok(())

  }
  fn chat_global(        
    accounts: &[AccountInfo],
    chat:Chat) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let chat_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

      let g_chat: ChatGlobal = ChatGlobal::try_from_slice(&chat_account.data.borrow())?;


      let new_chat: ChatGlobal = ChatGlobal{
        chat_line_1:g_chat.chat_line_2,
        chat_line_2:g_chat.chat_line_3,
        chat_line_3:g_chat.chat_line_4,
        chat_line_4:g_chat.chat_line_5,
        chat_line_5:g_chat.chat_line_6,
        chat_line_6:g_chat.chat_line_7,
        chat_line_7:g_chat.chat_line_8,
        chat_line_8:g_chat.chat_line_9,
        chat_line_9:g_chat.chat_line_10,
        chat_line_10:g_chat.chat_line_11,
        chat_line_11:g_chat.chat_line_12,
        chat_line_12:g_chat.chat_line_13,
        chat_line_13:g_chat.chat_line_14,
        chat_line_14:g_chat.chat_line_15,
        chat_line_15:g_chat.chat_line_16,
        chat_line_16:g_chat.chat_line_17,
        chat_line_17:g_chat.chat_line_18,
        chat_line_18:g_chat.chat_line_19,
        chat_line_19:g_chat.chat_line_20,
        chat_line_20:chat.chat,
      };

      new_chat.serialize(&mut &mut chat_account.data.borrow_mut()[..])?;


      Ok(())
  }
  fn chat_local(
    accounts: &[AccountInfo],
    chat: Chat) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let guest: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let speaker: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let game_state: &AccountInfo<'_> = next_account_info(accounts_iter)?;


      if speaker.key != guest.key && speaker.key != initializer.key {panic!()}

      let state: GameState = GameState::try_from_slice(&game_state.data.borrow())?;



      let guest_check: Pubkey = Pubkey::new_from_array(state.guest);
      let initializer_check:Pubkey = Pubkey::new_from_array(state.initializer);

      if initializer.key != &initializer_check{panic!()}
      if guest.key != &guest_check{panic!()}

      if chat.chat.len() > 50{panic!()}

      let new_state: GameState = GameState{
        host:state.host,
        waiting:state.waiting,
        initialized:state.initialized,
        gameseed:state.gameseed,
        lamports:state.lamports,
        initializer: state.initializer,
        gamehash:state.gamehash,
        guest:state.guest,
        whoseturn:state.whoseturn,
        guest_move:state.guest_move,
        score_i:state.score_i,
        score_g:state.score_g,
        game_ends:state.game_ends,
        lastplaytime:state.lastplaytime,
        chat_line_1:state.chat_line_2,
        chat_line_2:state.chat_line_3,
        chat_line_3:state.chat_line_4,
        chat_line_4:state.chat_line_5,
        chat_line_5:state.chat_line_6,
        chat_line_6:chat.chat,
      };

      new_state.serialize(&mut &mut game_state.data.borrow_mut()[..])?;


      Ok(())
  }
  fn winner_claim_prize(
    accounts: &[AccountInfo],
    program_id: &Pubkey) -> ProgramResult {

      let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

      let initializer: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let initializer_tour_acc: &AccountInfo<'_> = next_account_info(accounts_iter)?;
      let tournament: &AccountInfo<'_> = next_account_info(accounts_iter)?;
  
      let t: Tournament = Tournament::try_from_slice(&tournament.data.borrow())?;

      let initializer_tournament_account: TournamentAccount = TournamentAccount::try_from_slice(&initializer_tour_acc.data.borrow())?;

  
      let initializer_account_check: Pubkey = Pubkey::new_from_array(initializer_tournament_account.player);

  
      if t.is_init != 1{panic!()}
      if t.tournament_id != initializer_tournament_account.tournamentid{panic!()}


      if initializer.key != &initializer_account_check{panic!()}

      if tournament.owner != program_id{panic!()}
      if initializer_tour_acc.owner != program_id{panic!()}

      if tournament.is_writable {panic!()}
      if initializer_tournament_account.level != t.tournament_size{panic!()}

      **initializer_tour_acc.lamports.borrow_mut()-= t.winner_get;
      **initializer.lamports.borrow_mut()+= t.winner_get;


      Ok(())
  }

}




// 1 tas
// 2 makas
// 3 kagit


