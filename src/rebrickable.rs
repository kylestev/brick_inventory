use std::error::Error;
use std::path::Path;

use csv;
use serde::Deserialize;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
pub struct Color {
    id: String,
    name: String,
    rgb: String,
    is_trans: bool,
}

#[derive(Debug, Deserialize)]
pub struct Element {
    element_id: String,
    part_num: String,
    color_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct Minifig {
    fig_num: String,
    name: String,
    num_parts: u32,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    id: u32,
    name: String,
    parent_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Set {
    set_num: String,
    name: String,
    year: u32,
    theme_id: u32,
    num_parts: u32,
}

#[derive(Debug, Deserialize)]
pub struct InventorySet {
    inventory_id: u32,
    set_num: String,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Inventory {
    id: u32,
    version: u32,
    set_num: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryMinifig {
    inventory_id: u32,
    fig_num: String,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct InventoryPart {
    inventory_id: u32,
    part_num: String,
    color_id: i32,
    quantity: u32,
    is_spare: bool,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    part_num: String,
    name: String,
    part_cat_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct PartCategory {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct PartRelationship {
    rel_type: String,
    child_part_num: String,
    parent_part_num: String,
}

pub struct RebrickableDataset {
    pub colors: Vec<Color>,
    pub elements: Vec<Element>,
    pub themes: Vec<Theme>,
    pub sets: Vec<Set>,
    pub minifigs: Vec<Minifig>,

    pub parts: Vec<Part>,
    pub part_rels: Vec<PartRelationship>,
    pub part_cats: Vec<PartCategory>,

    pub invs: Vec<Inventory>,
    pub inv_sets: Vec<InventorySet>,
    pub inv_parts: Vec<InventoryPart>,
    pub inv_minifigs: Vec<InventoryMinifig>,
}

fn load_minifigs<P: AsRef<Path>>(path: P) -> Result<Vec<Minifig>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Minifig> = Vec::new();
    println!("reading Minifig");

    for result in rdr.records() {
        let record = result?;
        let row: Minifig = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_inv_minifigs<P: AsRef<Path>>(path: P) -> Result<Vec<InventoryMinifig>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<InventoryMinifig> = Vec::new();
    println!("reading InventoryMinifig");

    for result in rdr.records() {
        let record = result?;
        let row: InventoryMinifig = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_invs<P: AsRef<Path>>(path: P) -> Result<Vec<Inventory>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Inventory> = Vec::new();
    println!("reading Inventory");

    for result in rdr.records() {
        let record = result?;
        let row: Inventory = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_colors<P: AsRef<Path>>(path: P) -> Result<Vec<Color>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Color> = Vec::new();
    println!("reading Color");

    for result in rdr.records() {
        let record = result?;
        let row: Color = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_parts<P: AsRef<Path>>(path: P) -> Result<Vec<Part>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Part> = Vec::new();
    println!("reading Part");

    for result in rdr.records() {
        let record = result?;
        let row: Part = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_part_rels<P: AsRef<Path>>(path: P) -> Result<Vec<PartRelationship>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<PartRelationship> = Vec::new();
    println!("reading PartRelationship");

    for result in rdr.records() {
        let record = result?;
        let row: PartRelationship = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_part_cats<P: AsRef<Path>>(path: P) -> Result<Vec<PartCategory>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<PartCategory> = Vec::new();
    println!("reading PartCategory");

    for result in rdr.records() {
        let record = result?;
        let row: PartCategory = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_themes<P: AsRef<Path>>(path: P) -> Result<Vec<Theme>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Theme> = Vec::new();
    println!("reading Theme");

    for result in rdr.records() {
        let record = result?;
        let row: Theme = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_inv_parts<P: AsRef<Path>>(path: P) -> Result<Vec<InventoryPart>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<InventoryPart> = Vec::new();
    println!("reading InventoryPart");

    for result in rdr.records() {
        let record = result?;
        let row: InventoryPart = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_inv_sets<P: AsRef<Path>>(path: P) -> Result<Vec<InventorySet>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<InventorySet> = Vec::new();
    println!("reading InventorySet");

    for result in rdr.records() {
        let record = result?;
        let row: InventorySet = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

fn load_sets<P: AsRef<Path>>(path: P) -> Result<Vec<Set>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Set> = Vec::new();
    println!("reading Set");

    for result in rdr.records() {
        let record = result?;
        let row: Set = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

// fn load_generic<'t, P: AsRef<Path>, T : Deserialize<'t>>(path: P) -> Result<Vec<T>, Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_path(path)?;
//     let mut records: Vec<T> = Vec::new();
//     // println!("reading {}", path.as_ref().display());

//     for result in rdr.records() {
//         let record = result?;
//         let row: T = record.deserialize(None)?;
//         records.push(row);
//     }
//     Ok(records)
// }

fn load_elements<P: AsRef<Path>>(path: P) -> Result<Vec<Element>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records: Vec<Element> = Vec::new();
    println!("reading Element");

    for result in rdr.records() {
        let record = result?;
        let row: Element = record.deserialize(None)?;
        records.push(row);
    }
    Ok(records)
}

pub fn load_db() -> Result<RebrickableDataset, Box<dyn Error>> {
    let db = RebrickableDataset {
        colors: load_colors("./data/rebrickable/colors.csv")?,
        elements: load_elements("./data/rebrickable/elements.csv")?,
        minifigs: load_minifigs("./data/rebrickable/minifigs.csv")?,
        sets: load_sets("./data/rebrickable/sets.csv")?,
        themes: load_themes("./data/rebrickable/themes.csv")?,

        invs: load_invs("./data/rebrickable/inventories.csv")?,
        inv_sets: load_inv_sets("./data/rebrickable/inventory_sets.csv")?,
        inv_parts: load_inv_parts("./data/rebrickable/inventory_parts.csv")?,
        inv_minifigs: load_inv_minifigs("./data/rebrickable/inventory_minifigs.csv")?,

        parts: load_parts("./data/rebrickable/parts.csv")?,
        part_cats: load_part_cats("./data/rebrickable/part_categories.csv")?,
        part_rels: load_part_rels("./data/rebrickable/part_relationships.csv")?,
    };

    Ok(db)
}
