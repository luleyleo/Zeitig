
create table Actions (
    id integer,
    name text not null unique,

    primary key (id)
);

create table Subjects (
    id integer,
    name text not null unique,

    primary key (id)
);

create table TimeTable (
    action integer,
    subject integer,
    duration integer not null,

    primary key (action, subject),
    foreign key (action)
        references Actions (id),
    foreign key (subject)
        references Subjects (id)
);

create table History (
    started text not null,
    ended text not null,
    action integer,
    subject integer,

    primary key (started),
    foreign key (action)
        references Actions (id),
    foreign key (subject)
        references Subjects (id)
);

create table Meta (
    key text,
    value text,

    primary key (key)
);

insert into Meta (key, value) values ('version', 1);
