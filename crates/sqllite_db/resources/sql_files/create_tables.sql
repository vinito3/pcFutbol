/* TABLE PLAYER */
create table if not exists player (id integer primary key,name text,age integer,football_club text,media integer);

create table if not exists football_club (id integer primary key,name text,foundation_date integer,stadium_name text,money integer);