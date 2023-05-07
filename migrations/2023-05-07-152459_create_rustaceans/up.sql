Create table rustaceans (
    id Integer primary key autoincrement,
    name varchar(255) not null,
    email varchar(255) not null,
    created_at timestamp not null default current_timestamp
)