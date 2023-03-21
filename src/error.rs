use cosmwasm_std::StdError;
use thiserror::Error;
use pix0_contract_common::error::CommonContractError;

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
   
    #[error("FailedToMakePayment")]
    FailedToMakePayment { text : String },
    
    #[error("ContractInfoNotFound")]
    ContractInfoNotFound { message : String },

    #[error("FailedToTransferNft")]
    FailedToTransferNft { text : String },
  
    #[error("FailedToBurnNft")]
    FailedToBurnNft { text : String },
}


impl From<CommonContractError> for ContractError {
    fn from(error : CommonContractError) -> ContractError {
        
        match error {

            CommonContractError::ContractInfoNotFound { message } => 
            ContractError::ContractInfoNotFound { message: message }
            ,

            CommonContractError::ErrorMakingPayment { message } => 
            ContractError::FailedToMakePayment { text : message }
            ,

            _ => ContractError::CustomErrorMesg { message: "Unknown error".to_string() }

        }
    }
}
