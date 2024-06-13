use serde::{Deserialize, Serialize};
use crate::{proto, ErrorReport, Result};


/// Represents a Weighted Vote option of gov module
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WeightedVoteOption {
    /// Selected vote option
    pub option: i32,

    /// Weight of the vote on the selected vote option
    pub weight: String
}

impl TryFrom<proto::cosmos::gov::v1::WeightedVoteOption> for WeightedVoteOption {
    type Error = ErrorReport;

    fn try_from(proto: proto::cosmos::gov::v1::WeightedVoteOption) -> Result<Self> {
        Self::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::gov::v1::WeightedVoteOption> for WeightedVoteOption {
    type Error = ErrorReport;

    fn try_from(proto: &proto::cosmos::gov::v1::WeightedVoteOption) -> Result<Self> {
        Ok(Self {
            option: proto.option,
            weight: proto.weight.to_string()
        })
    }
}

impl From<WeightedVoteOption> for proto::cosmos::gov::v1::WeightedVoteOption {
    fn from(wtd_vote_option: WeightedVoteOption) -> proto::cosmos::gov::v1::WeightedVoteOption {
        proto::cosmos::gov::v1::WeightedVoteOption::from(&wtd_vote_option)
    }
}

impl From<&WeightedVoteOption> for proto::cosmos::gov::v1::WeightedVoteOption {
    fn from(wtd_vote_option: &WeightedVoteOption) -> proto::cosmos::gov::v1::WeightedVoteOption {
        proto::cosmos::gov::v1::WeightedVoteOption {
           option: wtd_vote_option.option,
           weight: wtd_vote_option.weight.to_string()
        }
    }
}
