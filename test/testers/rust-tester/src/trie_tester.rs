// Copyright (c) 2019 Web3 Technologies Foundation

// This file is part of Polkadot RE Test Suite

// Polkadot RE Test Suite is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot RE Tests is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Foobar.  If not, see <https://www.gnu.org/licenses/>.

///This file is an interface to run Parity implementation of state trie used in Polkadot RE.

extern crate clap;
extern crate serde_yaml;
extern crate trie_root;
extern crate trie_db;
extern crate reference_trie;

//use trie_root::trie_root_no_ext;
use trie_root::{trie_root_no_ext, unhashed_trie_no_ext, Hasher};
use reference_trie::ReferenceTrieStreamNoExt as ReferenceTrieStream;
use reference_trie::LayoutNewH;
use trie_db::{TrieRootPrint, trie_visit};
use memory_db::{MemoryDB, HashKey};
use std::collections::BTreeMap;

//use trie::{Encode, Decode, HasCompact, Compact, EncodeAsRef, CompactAs};
use clap::{ArgMatches};

use crate::hasher::blake2::Blake2Hasher;

fn compute_state_root(matches: &ArgMatches) {
    let trie_key_value_file = matches.value_of("state-file").unwrap();

    let f = std::fs::File::open(trie_key_value_file).unwrap();

    // We are deserializing the state data in a BTree
    let key_value_map: BTreeMap<String, Vec<String>> = serde_yaml::from_reader(f).unwrap();
    
    //let trie_value =  key_value_map["data"];
    let trie_vec: BTreeMap<Vec<u8>, Vec<u8>> = key_value_map["keys"].iter().zip(key_value_map["values"].iter())
      .map(|(s1, s2)|(AsRef::<[u8]>::as_ref(&s1).to_vec(), AsRef::<[u8]>::as_ref(&s2).to_vec()))
      .collect();


    let root_new: <Blake2Hasher as Hasher>::Out = {
      let mut cb = TrieRootPrint::<Blake2Hasher, _>::default();
      trie_visit::<LayoutNewH<Blake2Hasher>, _, _, _, _>(trie_vec.clone().into_iter(), &mut cb);
      cb.root.unwrap_or(Default::default())
    };
    disp(trie_vec.iter());
    println!("[rust] iter_build state root: {:x}", &root_new);
    let state_trie_root = trie_root_no_ext::<Blake2Hasher, ReferenceTrieStream, _, _, _>(trie_vec.clone());
    println!("[rust] trie_root state root: {:x}", &state_trie_root);
    let root_encoding = unhashed_trie_no_ext::<Blake2Hasher, ReferenceTrieStream, _, _, _>(trie_vec);
    //println!("[rust] state root: {:x}", &state_trie_root);
    println!("[rust] encoded root: {:x?}", &root_encoding);
    println!("[rust] len: {}", root_encoding.len())

}

pub fn process_state_trie_command(subcmd_matches: &ArgMatches) {
    if subcmd_matches.is_present("state-root") {
            compute_state_root(subcmd_matches);
    }
}

fn disp<I: Iterator<Item = (A, A)>, A: AsRef<[u8]>>(
	data: I,
) {

use memory_db::{MemoryDB, HashKey};
use reference_trie::{
	RefTrieDBMutNoExt,
	RefTrieDBNoExt,
	ref_trie_root,
};
use trie_db::{TrieMut};



	let mut memdb = MemoryDB::<_, HashKey<_>, _>::default();

	let root = {
		let mut root = Default::default();
		let mut t = RefTrieDBMutNoExt::new(&mut memdb, &mut root);
		for i in data {
//      println!("{:?}", (i.0.as_ref(), i.1.as_ref()));
			t.insert(i.0.as_ref(), i.1.as_ref()).unwrap();
		}
		t.root().clone()
	};
	{
		let db : &dyn hash_db::HashDB<_,_> = &memdb;
			let t = RefTrieDBNoExt::new(&db, &root).unwrap();
			println!("{:?}", t);
	}
}

