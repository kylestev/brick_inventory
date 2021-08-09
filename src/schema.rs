table! {
    colors (id) {
        id -> Int4,
        name -> Text,
        rgb -> Varchar,
        is_trans -> Nullable<Bool>,
    }
}

table! {
    elements (element_id) {
        element_id -> Varchar,
        part_num -> Varchar,
        color_id -> Int4,
    }
}

table! {
    inventories (id) {
        id -> Int4,
        version -> Int4,
        set_num -> Varchar,
    }
}

table! {
    inventory_minifigs (inventory_id, fig_num) {
        inventory_id -> Int4,
        fig_num -> Varchar,
        quantity -> Nullable<Int4>,
    }
}

table! {
    inventory_parts (id) {
        id -> Int4,
        inventory_id -> Int4,
        part_num -> Varchar,
        color_id -> Int4,
        quantity -> Nullable<Int4>,
        is_spare -> Nullable<Bool>,
    }
}

table! {
    inventory_sets (inventory_id, set_num) {
        inventory_id -> Int4,
        set_num -> Varchar,
        quantity -> Int4,
    }
}

table! {
    minifigs (fig_num) {
        fig_num -> Varchar,
        name -> Varchar,
        num_parts -> Int4,
    }
}

table! {
    part_categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    part_relationships (parent_part_num, child_part_num) {
        rel_type -> Varchar,
        child_part_num -> Varchar,
        parent_part_num -> Varchar,
    }
}

table! {
    parts (part_num) {
        part_num -> Varchar,
        name -> Varchar,
        part_cat_id -> Int4,
        part_material -> Nullable<Varchar>,
    }
}

table! {
    sets (set_num) {
        set_num -> Varchar,
        name -> Varchar,
        year -> Int4,
        theme_id -> Int4,
        num_parts -> Int4,
    }
}

table! {
    themes (id) {
        id -> Int4,
        name -> Varchar,
        parent_id -> Nullable<Int4>,
    }
}

joinable!(elements -> colors (color_id));
joinable!(inventory_sets -> sets (set_num));
joinable!(parts -> part_categories (part_cat_id));
joinable!(sets -> themes (theme_id));

allow_tables_to_appear_in_same_query!(
    colors,
    elements,
    inventories,
    inventory_minifigs,
    inventory_parts,
    inventory_sets,
    minifigs,
    part_categories,
    part_relationships,
    parts,
    sets,
    themes,
);
