// Storage daemon (stored): microservice frontend for different storage backends
// used in LNP/BP nodes.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use crate::{Chunk, ChunkId};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[derive(Api)]
#[api(encoding = "strict")]
pub enum Request {
    #[api(type = 0x10)]
    #[display("store({0})")]
    Store(StoreReq),

    #[api(type = 0x12)]
    #[display("retrieve({0})")]
    Retrieve(ChunkInfo),
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{db}, ...")]
pub struct StoreReq {
    pub db: String,
    pub chunk: Chunk,
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash, Display)]
#[derive(NetworkEncode, NetworkDecode)]
#[display("{db}, {chunk_id}")]
pub struct ChunkInfo {
    pub db: String,
    pub chunk_id: ChunkId,
}