// use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::StdError;

#[derive(Error, Debug)]

pub enum ContractError {

    // Previous code omitted
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("InvalidUnitPrice")]
    InvalidUnitPrice {},

    #[error("InvalidMaxTokens")]
    InvalidMaxTokens {},

    #[error("InvalidTokenReplyId")]
    InvalidTokenReplyId {},

    #[error("Cw721AlreadyLinked")]
    Cw721AlreadyLinked {},

    #[error("SoldOut")]
    SoldOut {},

    #[error("UnauthorizedTokenContract")]
    UnauthorizedTokenContract {},

    #[error("Uninitialized")]
    Uninitialized {},

    #[error("WrongPaymentAmount")]
    WrongPaymentAmount {},

    #[error("Cw721NotLinked")]
    Cw721NotLinked {},

    

    // Following code omitted
}
