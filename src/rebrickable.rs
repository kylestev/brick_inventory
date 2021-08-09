use std::error::Error;
use std::io;
use std::path::Path;
use std::process;

use csv;

use serde::Deserialize;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
pub(crate) struct Color {
    id: String,
    name: String,
    rgb: String,
    is_trans: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Element {
    element_id: String,
    part_num: String,
    color_id: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Minifig {
    fig_num: String,
    name: String,
    num_parts: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Theme {
    id: u32,
    name: String,
    parent_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Set {
    set_num: String,
    name: String,
    year: u32,
    theme_id: u32,
    num_parts: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InventorySet {
    inventory_id: u32,
    set_num: String,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Inventory {
    id: u32,
    version: u32,
    set_num: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InventoryMinifig {
    inventory_id: u32,
    fig_num: String,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct InventoryPart {
    inventory_id: u32,
    part_num: String,
    color_id: u32,
    quantity: u32,
    is_spare: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Part {
    part_num: String,
    name: String,
    part_cat_id: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PartCategory {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PartRelationship {
    rel_type: String,
    child_part_num: String,
    parent_part_num: String,
}

pub(crate) struct RebrickableDataset {
    parts: Vec<Part>,
    part_rels: Vec<PartRelationship>,
    part_cats: Vec<PartCategory>,
    
    themes: Vec<Theme>,
    sets: Vec<Set>,
    minifigs: Vec<Minifig>,
    inv_minifigs: Vec<InventoryMinifig>,
}

fn load_minifigs<P: AsRef<Path>>(path: P) -> Result<Vec<Minifig>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let records: Vec<Minifig> = Vec::new();

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let record = result?;
        // records.append(record);
        println!("{:?}", record);
    }
    Ok(records)
}

fn load_db() -> Result<RebrickableDataset, Box<dyn Error>> {
    let mut db = RebrickableDataset{
        inv_minifigs: vec![],
        minifigs: vec![],
        part_cats: vec![],
        part_rels: vec![],
        parts: vec![],
        sets: vec![],
        themes: vec![]
    };

    Ok(db)
}

// fn example() -> Result<(), Box<dyn Error>> {
//     // csv::Reader::from_path(path)
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     for result in rdr.deserialize() {
//         // Notice that we need to provide a type hint for automatic
//         // deserialization.
//         let record: Record = result?;
//         println!("{:?}", record);
//     }
//     Ok(())
// }

pub fn main2() {
    if let Err(err) = load_minifigs("./data/rebrickable/minifigs.csv") {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
