use std::ops::{Div, Mul};

/**
Enunciado:
Un viejo banquero retirado se mudó a vivir al campo lejos de toda la tecnología.
Para vivir, invierte la plata que hizo durante sus años de trabajo mediante los amigos que tiene en diversos fondos de inversión.
Al inicio de cada semana les envía por correo el dinero para invertir a sus amigos;
luego espera hasta el final de la semana a que le envíen a su buzón el resultado de esas inversiones.
Modelar la situación planteada en Rust, considerando que:
- A todos los amigos les envía el mismo monto
- Que el dinero resultante lo vuelve a invertir la próxima semana
- Que las inversiones pueden dar una ganancia entre -50% y 50% de lo invertido.
**/
use chrono::prelude::*;
use rand::Rng;

fn main() {
    let mut should_invest = true;
    let mut current_funds: f64 = assign_some_funds();
    let mut friends: Vec<Friend> = friends();
    let mut current_date: Weekday = Utc::now().date().weekday();

    while should_invest {
        if is_beginning_of_the_week(current_date) {
            invest_funds(current_funds, &mut friends);
            current_date = next_date(&mut current_date);
            println!("{} funds were assigned", current_funds);
        } else {
            let available_funds = receive_invested_funds(&mut friends);
            should_invest = continue_investing_funds(current_funds, available_funds);
            current_funds = available_funds;
            current_date = next_date(&mut current_date);
            println!("{} funds were retrieved", available_funds);
            println!("...............");
        }
    }
}

/**
*
*
*
*
*
***/
trait Financier {
    fn investment_result(&self) -> f64;
}

struct Friend {
    funds: f64,
    multiplier: f64,
}

impl Financier for Friend {
    fn investment_result(&self) -> f64 {
        return self.funds.mul(self.multiplier);
    }
}

fn receive_invested_funds(friends: &mut Vec<Friend>) -> f64 {
    let mut funds: f64 = 0.0;
    for i in 0..friends.len() {
        funds += friends[i].investment_result();
    }
    return funds;
}

fn assign_some_funds() -> f64 {
    return rand::thread_rng().gen::<f64>().mul(5000.0);
}

fn invest_funds(funds: f64, friends: &mut Vec<Friend>) {
    let funds_to_friend = calculate_funds_to_friend(funds, friends);
    for i in 0..friends.len() {
        friends[i].funds += funds_to_friend;
        friends[i].multiplier = generate_multiplier();
    }
}

fn calculate_funds_to_friend(
    funds: f64,
    friends: &mut Vec<Friend>,
) -> f64 {
    funds.div(friends.len() as f64)
}

fn generate_multiplier() -> f64 {
    let m1: f64 = rand::thread_rng().gen();
    let m2: f64 = rand::thread_rng().gen();
    m1 + m2
}

fn friends() -> Vec<Friend> {
    let amount_of_friends: i32 = rand::thread_rng().gen_range(0, 100);
    let mut friends: Vec<Friend> = Vec::new();
    for _ in 1..amount_of_friends {
        friends.push(Friend {
            funds: 0.0,
            multiplier: 1.0,
        });
    };
    friends
}

fn continue_investing_funds(
    current_funds: f64,
    available_funds: f64,
) -> bool {
    return current_funds.div(available_funds) - 1.0 < 0.5 &&
        current_funds.div(available_funds) - 1.0 > -0.5;
}

fn next_date(current_date: &mut Weekday) -> Weekday {
    if is_beginning_of_the_week(*current_date) {
        return Weekday::Sun;
    }
    return Weekday::Mon;
}

fn is_beginning_of_the_week(day: Weekday) -> bool {
    return if day.num_days_from_monday() < 4 { true } else { false };
}
