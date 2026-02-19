# tamk-ohsyte-2026
TAMK / Tietotekniikka / Ohjelmoinnin syventävät tekniikat 2026

## 01 Rust-työkalut

Asenna Rust-työkalut omalle koneellesi: [Rust : Getting Started](https://rust-lang.org/learn/get-started/)

## 02 Projektit, Rust-kielen perusteet

Tee Cargolla uusi projekti nimeltä `events`, käännä ja aja:

    cargo new events
    cd events
    cargo run

Tämän kerran kehittelyt ohjelmaan löytyvät hakemistosta `02/events`.

## 03 Funktiot, struct, enum, match, Option

Käytiin läpi funktiot (`fn`), tietuetyypit (`struct`), luetelmatyypit (`enum`) ja
optionaaliset tyypit (`Option<T>`). Päivitettiin events-ohjelma
käyttämään näitä tyyppejä (liittyen kotitehtävään 3, jossa on
myös lisäosio liittyen tapahtumien hakemiseen).

Tämän kerran kehittelyt ohjelmaan löytyvät hakemistosta `03/events`.

Jos kirjan PDF:stä kopioidun lähdekoodin sisennykset menevät
ihan sekaisin, voit muotoilla sen uudelleen komennolla `cargo fmt`
(lisätietoja komennolla `cargo help fmt`).

## 04 Vektorit, komentoriviparametrit, virheenkäsittely, piirteet, pakkaukset

Vaihdettiin tapahtumia sisältävä taulukko vektoriin. Luettiin tapahtumien suodattamiseen 
käytettävä päivämäärä komentoriviparametrista.

Tutkittiin miten käsitellään virhetilanne `Result`-tyypin avulla.

Tutustuttiin tarkemmin piirteisiin (traits).

Opeteltiin käyttämään Rustin pakkauksia (crates), jotka Cargo lataa crates.io-palvelusta.

Tämän kerran kehittelyt ohjelmaan löytyvät hakemistosta `04/events`.

## 05 Yksikkötestit, omistajuus, viittaukset ja lainaaminen

Käsiteltiin Rustin ominaispiirteitä: arvon omistajuus, viitteet arvoihin sekä
arvon "lainaaminen" viitteiden avulla.

Lisäksi opeteltiin tekemään yksikkötestejä käyttämällä cfg-asetusattribuuttia
sekä test-merkittyjä funktioita.

Tämän kerran kehittelyt löytyvät hakemistosta `05/dates`, jossa on malleja
yksikkötesteistä ohjelman tietotyypeille.

## 06 Moduulit, tapahtumatuottaja / piirteen toteuttaminen

Tutkittiin miten ohjelmakoodi jaetaan moduuleihin.

Tehtiin piirre eli trait, jonka kaikki tapahtumantuottajat
toteuttavat. 

Today-ohjelman perusta löytyy hakemistosta `06/today`.
