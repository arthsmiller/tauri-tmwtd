CREATE TABLE IF NOT EXISTS `priority` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `priority` VARCHAR(255) NOT NULL,
    UNIQUE(`priority`)
);

INSERT INTO priority (priority) VALUES
    ("Low"),
    ("Medium"),
    ("High"),
    ("Urgent")
;

CREATE TABLE IF NOT EXISTS `status` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `status` VARCHAR(255) NOT NULL,
    UNIQUE(`status`)
);

INSERT INTO status (status) VALUES
    ("Not Started"),
    ("In Progress"),
    ("Postponed"),
    ("Completed"),
    ("On Hold"),
    ("Cancelled")
;

CREATE TABLE IF NOT EXISTS `tasks` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `title` VARCHAR(255) NOT NULL,
    `description` VARCHAR(2048),
    `estimated_time` INTEGER,
);

CREATE TABLE IF NOT EXISTS `tags` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `task_id` INTEGER NOT NULL,
    `tag` VARCHAR(255) NOT NULL,
    FOREIGN KEY (task_id) REFERENCES task(id),
    CONSTRAINT UC_Tags_Tasks UNIQUE (tag, task_id)
);

CREATE TABLE IF NOT EXISTS `queue` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `task_id` INTEGER NOT NULL,
    `start` DATE NOT NULL,
    `end` DATE NOT NULL,
    `status` VARCHAR(255),
    `priority` INTEGER,
    FOREIGN KEY (task_id) REFERENCES task(id),
    FOREIGN KEY (status) REFERENCES status(id)
    FOREIGN KEY (priority) REFERENCES priority(id),
);
