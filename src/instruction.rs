use crate::error::GameError::InvalidInstruction;
use crate::state::{Init,Join,InitializerPlay,UpdateRent,Tournament,Chat,InitTournamentCounter,CounterFinder,FinderFinder};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum GameInstruction {
  InitGame{ init : Init},
  JoinGame{ join : Join},
  IPlay{ play : InitializerPlay},
  GPlay{ play : Join},
  ClaimVictory,
  Abort,
  InitTournament{t:Tournament},
  JoinTournament{init:Init},
  InitTournamentMatch{init:Init},
  TournamentMatchAccept{join:Join},
  TournamentInPlay{play:InitializerPlay},
  TournamentGuPlay{play:InitializerPlay},
  EliminateForIn,
  EliminateForMv,
  InitCounter{t_counter:InitTournamentCounter},
  UpdateCounterFinder{c_finder:CounterFinder},
  UpdateFinderFinder{f_finder:FinderFinder},
  CloseAccount,
  ChatGlobal{chat:Chat},
  ChatLocal{chat:Chat},
  UpRent{r:UpdateRent},
  WinnerClaim

}

impl GameInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    Ok(match tag {
      0 => Self::InitGame{
        init: Init::try_from_slice(&rest)?,
      },
      1 => Self::JoinGame{
        join: Join::try_from_slice(&rest)?,
      },
      2 => Self::IPlay{
        play: InitializerPlay::try_from_slice(&rest)?,
      },
      3 => Self::GPlay{
        play: Join::try_from_slice(&rest)?,
      },
      4 => Self::ClaimVictory,
      5 => Self::Abort,
      6 => Self::InitTournament{
        t: Tournament::try_from_slice(&rest)?,
      },
      7 => Self::JoinTournament{
        init: Init::try_from_slice(&rest)?,
      },
      8 => Self::InitTournamentMatch{
        init: Init::try_from_slice(&rest)?,
      },
      9 => Self::TournamentMatchAccept{
        join: Join::try_from_slice(&rest)?,
      },
      11 => Self::TournamentInPlay{
        play: InitializerPlay::try_from_slice(&rest)?,
      },
      12 => Self::TournamentGuPlay{
        play: InitializerPlay::try_from_slice(&rest)?,
      },
      13 => Self::EliminateForIn,
      14 => Self::EliminateForMv,
      15 => Self::InitCounter{
        t_counter: InitTournamentCounter::try_from_slice(&rest)?,
      },
      16 => Self::UpdateCounterFinder{
        c_finder: CounterFinder::try_from_slice(&rest)?,
      },
      17 => Self::UpdateFinderFinder{
        f_finder: FinderFinder::try_from_slice(&rest)?,
      },
      18 => Self::CloseAccount,
      19 => Self::ChatGlobal{
        chat: Chat::try_from_slice(&rest)?,
      },
      20 => Self::ChatLocal{
        chat: Chat::try_from_slice(&rest)?,
      },
      21 => Self::UpRent{
        r: UpdateRent::try_from_slice(&rest)?,
      },
      22 => Self::WinnerClaim,

      _ => return Err(InvalidInstruction.into()),
    })
  }
}
