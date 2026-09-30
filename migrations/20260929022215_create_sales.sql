-- Add migration script here
CREATE TABLE IF NOT EXISTS sales (
    id TEXT PRIMARY KEY NOT NULL,
    unit_id TEXT NOT NULL,
    department_id TEXT NOT NULL,
    sail_date TEXT NOT NULL DEFAULT (datetime('now')),
    sold_items_count INTEGER NOT NULL,
    
    FOREIGN KEY (unit_id) REFERENCES product(item_number),
    FOREIGN KEY (department_id) REFERENCES department(id)
);