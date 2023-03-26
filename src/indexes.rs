use crate::state::{Collection, NftItem};
use cosmwasm_std::Addr;
use cw_storage_plus::{UniqueIndex, Index, IndexList, IndexedMap, Map};
use crate::ins::collection_id;


pub const COLLECTION_ITEMS_STORE : Map<(Addr,String,String), NftItem> = Map::new("COLLECTION_ITEMS_STORE");

pub struct CollectionIndexes<'a> {

    // unique index by wallet address
    pub collections : UniqueIndex<'a, (Addr,String), Collection>,

    // unique index by name and symbols
    pub name_symbols : UniqueIndex<'a, String, Collection>,
}


impl IndexList<Collection> for CollectionIndexes<'_> {

    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Collection>> + '_> {

        let v : Vec<&dyn Index<Collection>> = vec![&self.collections, &self.name_symbols];
        Box::new(v.into_iter())
    } 
}

pub fn collections_store<'a>() -> IndexedMap<'a,(Addr,String), Collection, CollectionIndexes<'a>> {

    let indexes = CollectionIndexes {

        collections : UniqueIndex::new(|u| (u.owner.clone().unwrap_or(Addr::unchecked("unknown")),
        collection_id(u.name.clone(), u.symbol.clone())), "COLLECTIONS"),

        name_symbols :  UniqueIndex::new(|u|  
        collection_id(u.name.clone(), u.symbol.clone()), "COLLECTION_NAME_SYMBS"),
    };

    IndexedMap::new("COLLECTIONS_STORE", indexes)
}


