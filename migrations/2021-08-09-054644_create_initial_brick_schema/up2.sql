-- -------------------------------------------------------------
-- TablePlus 3.12.6(366)
--
-- https://tableplus.com/
--
-- Database: bricks
-- Generation Time: 2021-08-08 23:53:43.1860
-- -------------------------------------------------------------


-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."colors" (
    "id" int4 NOT NULL primary key,
    "name" text NOT NULL,
    "rgb" varchar(6) NOT NULL,
    "is_trans" bool DEFAULT false
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."elements" (
    "element_id" varchar(10) NOT NULL primary key,
    "part_num" varchar(20) NOT NULL,
    "color_id" int4 NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."inventories" (
    "id" int4 NOT NULL primary key,
    "version" int4 NOT NULL,
    "set_num" varchar(20) NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."inventory_minifigs" (
    "inventory_id" int4,
    "fig_num" varchar(20),
    "quantity" int4
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."inventory_parts" (
    "inventory_id" int4,
    "part_num" varchar,
    "color_id" int4,
    "quantity" int4,
    "is_spare" bool
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."inventory_sets" (
    "inventory_id" int4 NOT NULL,
    "set_num" varchar(20) NOT NULL,
    "quantity" int4 NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."minifigs" (
    "fig_num" varchar(20) NOT NULL primary key,
    "name" varchar(256) NOT NULL,
    "num_parts" int4 NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."part_categories" (
    "id" int4 NOT NULL primary key,
    "name" varchar(200) NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."part_relationships" (
    "rel_type" varchar(1) NOT NULL primary key,
    "child_part_num" varchar(20) NOT NULL,
    "parent_part_num" varchar(20) NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."parts" (
    "part_num" varchar(20) NOT NULL primary key,
    "name" varchar(256) NOT NULL,
    "part_cat_id" int4 NOT NULL,
    "part_material" varchar(200)
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."sets" (
    "set_num" varchar(20) NOT NULL primary key,
    "name" varchar(256) NOT NULL,
    "year" int4 NOT NULL,
    "theme_id" int4 NOT NULL,
    "num_parts" int4 NOT NULL
);

-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Table Definition
CREATE TABLE "public"."themes" (
    "id" int4 NOT NULL primary key,
    "name" varchar(50) NOT NULL,
    "parent_id" int4
);


ALTER TABLE "public"."elements" ADD FOREIGN KEY ("color_id") REFERENCES "public"."colors"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."inventory_sets" ADD FOREIGN KEY ("set_num") REFERENCES "public"."sets"("set_num") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."part_relationships" ADD FOREIGN KEY ("child_part_num") REFERENCES "public"."parts"("part_num") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."part_relationships" ADD FOREIGN KEY ("parent_part_num") REFERENCES "public"."parts"("part_num") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."parts" ADD FOREIGN KEY ("part_num") REFERENCES "public"."parts"("part_num") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."parts" ADD FOREIGN KEY ("part_cat_id") REFERENCES "public"."part_categories"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."sets" ADD FOREIGN KEY ("theme_id") REFERENCES "public"."themes"("id") ON DELETE CASCADE ON UPDATE CASCADE;
