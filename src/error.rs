use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("CustomErrorMesg")]
    CustomErrorMesg { message : String },

    #[error("ErrorPayingTreasuries")]
    ErrorPayingTreasuries { text : String },

    #[error("FailedToFindNft")]
    FailedToFindNft { text : String },

    #[error("NftIndexOutOfBound")]
    NftIndexOutOfBound { text : String },

    #[error("NftStatusIsNotReadyForMinting")]
    NftStatusIsNotReadyForMinting { text : String },
     
    #[error("InvalidIndexOfNft")]
    InvalidIndexOfNft { text : String },

    #[error("InvalidCollectionStatus")]
    InvalidCollectionStatus { text : String },

    #[error("CollectionNotFound")]
    CollectionNotFound { text : String },

    #[error("MintByNameIsNotAllowed")]
    MintByNameIsNotAllowed { text : String },

    #[error("InsufficientFund")]
    InsufficientFund { text : String },
    
}
