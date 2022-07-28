create database booking;
\c booking

drop table if exists organization;
drop table if exists booking;
drop table if exists photo;
drop table if exists hotel;
drop table if exists classRoom;
drop table if exists apartment;

-- Таблица отелей
create table hotel (
    id serial primary key, 
    nameHotel varchar(255), 
    stars integer,
    addr varchar(255),
    avgprice float
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
    comforts varchar(255),
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
);

create table organization (
    orgid serial primary key,
    orgname varchar(255)
);

insert into classRoom (nameClass) values ('Люкс'), ('Эконом');
insert into hotel (namehotel, stars) values ('Hotel 1', 5), ('Hotel 2', 3);
insert into apartment (countRoom, price) values (4, 4);
insert into organization (orgname) values ();



--Сделаем все на формате json
drop table if exists hotel_j;
drop table if exists classRoom_j;
drop table if exists apartment_j;
drop table if exists booking_j;
drop table if exists photo_j;
drop table if exists organization_j;
--В формате json
create table hotel_j (
    hotj jsonb,
    --ограничение на пустую строку и null
    constraint validate_id check ((hotj->>'id') is not null and (length(hotj->>'id')) > 0)
);

--Только уникальные ид
create unique index ui_hotelj_id on hotel_j((hotj->'id'));

create table classRoom_j (
    claj jsonb--ограничение на пустую строку и null
    constraint validate_claid check ((claj->>'id') is not null and (length(claj->>'id')) > 0)
);

create unique index ui_classRoomj_id on classRoom_j((claj->'id'));

create table apartment_j (
    apaj jsonb--ограничение на пустую строку и null
    constraint validate_apaid check ((apaj->>'id') is not null and (length(apaj->>'id')) > 0)
);

create unique index ui_apartmentj_id on apartment_j((apaj->'id'));

--hotelId integer, 
  --  apartmentId integer, 
    --dateFrom date, 
    --dateTo date,
create table booking_j (
    booj jsonb--ограничение на пустую строку и null
    constraint validate_booid check ((booj->>'id') is not null and (length(booj->>'id')) > 0)
);

create unique index ui_bookingj_id on booking_j((booj->'id'));

create table photo_j (
    phoid uuid,
    phoimage bytea,
    hotelId uuid,
    apartmentId uuid
    --constraint fk_photelId foreign key (hotelId) references hotel_j ((hotj->>'id')::uuid) on delete cascade,
    --constraint fk_papartmentId foreign key (apartmentId) references apartment_j ((apaj->>'id')::uuid) on delete cascade
);

create table organization_j (
    orgj jsonb--ограничение на пустую строку и null
    constraint validate_orgid check ((orgj->>'id') is not null and (length(orgj->>'id')) > 0)
);

create unique index ui_organizationj_id on organization_j((orgj->'id'));

create table user_j (
    usej jsonb,
    constraint validate_useid check ((usej->>'id') is not null and (length(usej->>'id')) > 0)
);

create unique index ui_user_id on user_j((usej->'id'));
create unique index ui_user_email on user_j((usej->'email'));
create unique index ui_user_token on user_j((usej->'token'));

--Пример вставки
insert into hotel_j (hotj) values (('{"id":"'||gen_random_uuid()||'","name":"Vlad"}')::jsonb);

insert into hotel_j (hotj) values (('{"id":"404f6ed9-0062-494a-8bd5-7a1726656528","name":"Vlad"}')::jsonb);
insert into booking_j (booj) values (('{"id": 1, "hotelId": 1, "apartmentId": 1, "dateFrom": "2022-01-01", "dateTo": "2022-01-17"}'));








----------------------------------------------
drop table if exists keyboard;
drop table if exists mouse;
drop table if exists syst_block;
drop table if exists monitor;
drop table if exists software;
drop table if exists equipment;
drop table if exists local_net;
drop table if exists info_internet;
drop table if exists defend_net;
drop table if exists net_inf_tvsp;
drop table if exists info_system;
drop table if exists diag_equipment;
drop table if exists tvsp;
drop table if exists municip_instit;

create table municip_instit (
    munid serial primary key,
    munname varchar(255),
    munaddr varchar(255),
    munoid integer,
    munlevel integer
);

create table tvsp (
    tvsid serial primary key,
    tvsname varchar(255),
    tvstype varchar(255),
    tvsaddress varchar(255),
    tvsoid integer,
    tvsambulance boolean,
    tvspmsp boolean,
    tvsregist boolean,
    tvsstation boolean,
    tvskdl boolean,
    tvssmp boolean,
    tvsnmp boolean,
    tvsinstrdiag boolean,
    tvsllo boolean,
    tvsmse boolean,
    tvslist boolean,
    tvsssz boolean,
    tvscoord point,
    tvsfloor int,
    tvsform varchar(255),
    tvsstatus varchar(255),
    tvsplan varchar(255),
    munid integer,
    constraint fk_tvsp_municip foreign key (munid) references municip_instit (munid) on delete cascade
);

create table diag_equipment (
    diaid serial primary key,
    dianame varchar(255),
    diatype varchar(255),
    diadate date,
    diaplace varchar(255),
    diaenable boolean,
    diacount_research integer,
    diavolume_research float,
    diadicom boolean,
    diaactivate boolean,
    diaarm_diag boolean,
    tvsd_id integer,
    constraint fk_diag_tvsp foreign key (tvsd_id) references tvsp (tvsid) on delete cascade
);

create table info_system (
    infid serial primary key,
    infname varchar(255),
    ingtype varchar(255),
    infdeveloper varchar(255),
    infenemy varchar(255),
    infcert integer,
    infgis boolean,
    infobjkii boolean,
    infcat varchar(255),
    munid integer,
    constraint fk_info_municip foreign key (munid) references municip_instit (munid) on delete cascade
);

create table net_inf_tvsp (
    netid serial primary key,
    tvsid integer,
    constraint fk_netinf_tvsp foreign key (tvsid) references tvsp (tvsid) on delete cascade
);

create table defend_net (
    defid serial primary key,
    defstatus boolean,
    deftech varchar(255),
    defvipcoord varchar(255),
    defviphw varchar(255),
    defvipclient varchar(255),
    netid integer,
    constraint fk_defend_net foreign key (netid) references net_inf_tvsp (netid) on delete cascade
);

create table info_internet (
    intid serial primary key,
    intstat varchar(255),
    inttech varchar(255),
    intchanel varchar(255),
    inttarif varchar(255),
    intsupl varchar(255),
    intprice float,
    intfp boolean,
    netid integer,
    constraint fk_info_net foreign key (netid) references net_inf_tvsp (netid) on delete cascade
);

create table local_net (
    locid serial primary key,
    loccount integer,
    loceq varchar(255),
    locip varchar(255),
    locserver boolean,
    locareasize float,
    loclocation varchar(255),
    loctreb boolean,
    locschema varchar(255),
    netid integer,
    constraint fk_local_net foreign key (netid) references net_inf_tvsp (netid) on delete cascade
);

create table equipment (
    equid serial primary key,
    equcabinet integer,
    netid integer,
    constraint fk_equ_net foreign key (netid) references net_inf_tvsp (netid) on delete cascade
);

create table software (
    sofid serial primary key,
    sofname varchar(255),
    sofdeveloper varchar(255),
    sofrussian boolean,
    sofversion varchar(10),
    sofprice float,
    sofdate date,
    equid integer,
    constraint fk_soft_equ foreign key (equid) references equipment (equid) on delete cascade
);

create table monitor (
    monid serial primary key,
    monmodel varchar(255),
    mondiag varchar(10),
    moninv varchar(255),
    mondate date,
    equid integer,
    constraint fk_monit_equ foreign key (equid) references equipment (equid) on delete cascade
);

create table syst_block (
    sysid serial primary key,
    sysdata date,
    sysinv varchar(255),
    sysmodel varchar(255),
    sysmodram varchar(255),
    sysvolram varchar(10),
    systypehold varchar(255),
    sysvolumehold varchar(10),
    sysmodelhold varchar(255),
    sysos varchar(255),
    equid integer,
    constraint fk_syst_equ foreign key (equid) references equipment (equid) on delete cascade
);

create table mouse (
    id serial primary key,
    model varchar(255),
    date_buy date,
    inv integer,
    equipmentId integer,
    constraint fk_mouse_equ foreign key (equipmentId) references equipment (equid) on delete cascade
);

create table keyboard (
    id serial primary key,
    model varchar(255),
    date_buy date,
    inv integer,
    equipmentId integer,
    constraint fk_key_equ foreign key (equipmentId) references equipment (equid) on delete cascade
);

create table test_json (
    j jsonb
);

insert into test_json (j) values ('[{
    "token": "jQw62fyzVbsmMzRGjhfsdgy67ashqyHyfgAGSQHSFXNXHASDFKL8fsd6sHSADFfsdns",
    "id": "12345",
    "name": "Vlad",
    "card": "1234567890",
    "email": "kue@kd.ru",
    "password": "qwerty",
    "enter": true
}, {
    "token": "jQw62fyzVbsmMzRGjhfsdgy67ashqyHyfgAGSQHSFXNXHASDFKL8fsd6sHSADFfsdns",
    "id": "12345",
    "name": "Vlad",
    "card": "1234567890",
    "email": "kue@kd.ru",
    "password": "qwerty",
    "enter": true
}]');

insert into test_json (j) values ('{
    "token": "jQw62fyzVbsmMzRGjhfsdgy67ashqyHyfgAGSQHSFXNXHASDFKL8fsd6sHSADFfsdns",
    "id": "12345",
    "name": "Vlad",
    "card": "1234567890",
    "email": "kue@kd.ru",
    "password": "qwerty",
    "enter": true
}');

select j->'id', 
    j->'id'='"12345"'::jsonb, 
    '12345'::jsonb jb, 
    j->>'id'='12345' 
from test_json;

select j from test_json where j->'id' = '"12345"'::jsonb;

select hotj from hotel_j where hotj->>'isModified' <> 'true' and hotj->>'isModified' <> 'false';
update hotel_j set hotj->>'isModified' = 'true' where hotj->>'isModified' is null;
update hotel_j set hotj = hotj || '{"isModified": "true"}'::jsonb where hotj->>'isModified' is null;

select 
    hotj,
    hotj || '{"isModified": "true"}'::jsonb
from hotel_j where hotj->>'id' = '0ce1bb26-536a-4724-92aa-cb6f3ed99e1e';
