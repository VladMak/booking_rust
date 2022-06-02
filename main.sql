create database booking;
\c booking

drop table if exists hotel;
drop table if exists classRoom;
drop table if exists apartment;
drop table if exists booking;

-- Таблица отелей
create table hotel (
    id serial primary key, 
    nameHotel varchar(255), 
    stars integer
);

--Таблица классов номеров (эконом, люкс)
create table classRoom (
    id serial primary key, 
    nameClass varchar(80)
);

-- Таблица номеров
create table apartment (
    id serial primary key, 
    countRoom integer, 
    classId integer, 
    hotelId integer,
    price float,
    constraint fk_classId foreign key (classId) references classRoom (id) on delete cascade,
    constraint fk_hotelId foreign key (hotelId) references hotel (id) on delete cascade
);

-- Таблица брони, какой отель, номер, дата
create table booking (
    id serial primary key, 
    hotelId integer, 
    apartmentId integer, 
    dateFrom date, 
    dateTo date,
    constraint fk_bhotelId foreign key (hotelId) references hotel (id) on delete cascade,
    constraint fk_bapartmenId foreign key (apartmentId) references apartment (id) on delete cascade
);

-- Фотографии
create table photo (
    id serial primary key,
    hotelId integer,
    apartmentId integer,
    src text,
    constraint fk_photelId foreign key (hotelId) references hotel (id) on delete cascade,
    constraint fk_papartmentId foreign key (apartmentId) references apartment (id) on delete cascade
)

insert into hotel (id, namehotel, stars) values (1, 'Hotel 1', 5), (2, 'Hotel 2', 3);